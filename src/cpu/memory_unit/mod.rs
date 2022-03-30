use self::memory_segments::*;
use super::helperModules::*;
use super::Flags;
use super::instructions::*;

use std::collections::HashMap;
use queues::*;

pub mod memory_segments;

/**
 * This is the way to returns reference to register/memory.
 */
pub enum Operand {
    Register(*mut u16),
    // true - high byte. false - low byte
    SmallRegister(*mut u16, bool),
    Memory(MemoryPointer),
    Data(u32),
    None,
} impl Operand {

    // Setter function
    pub fn set(&mut self, val: u32) {
        match self {
            &mut Self::Register(reg) => unsafe{*reg = val as u16},
            &mut Self::SmallRegister(reg, level) => unsafe{
                if level {
                    *reg &= 0x00FF;
                    *reg += (val << 8) as u16;
                }
                else {
                    *reg &= 0xFF00;
                    *reg += val as u16;
                }
            },
            Self::Memory(mem_pointer) => mem_pointer.set(val as u32),
            &mut Self::Data(mut _data) => _data = val,
            Self::None => (),
        }
    }

    // Getter function
    pub fn get(&self) -> u32 {
        match self {
            &Self::Register(reg) => unsafe{*reg as u32},
            &Self::SmallRegister(reg, level) => unsafe{
                if level {
                    return (*reg >> 8) as u32;
                }
                *reg as u32},
            Self::Memory(mem_pointer) => mem_pointer.get(),
            &Self::Data(data) => data as u32,
            Self::None => 0,
        }
    }
}

/**
 * This struct is responsible for including all the registers and memory related 
 * objects.
 */
pub struct MemoryUnit {
    pub flags: Flags,
    pub ip: u16, // Instruction Pointer
    pub sp: u16, // Stack Pointer
    pub bp: u16, // Base Pointer
    pub si: u16, // Source Index
    pub di: u16, // Destination Index
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    pub memory: MemorySegments,

    // ModRM related
    pub reference: u16, // holds either a coded register or address
    ref_is_reg: bool,

    pub inst_bus: Buffer<u8>,
    pub segment: Segment,
    pub opcodes: HashMap<u8, InstructionFormat>,

    pub dst_operand: Operand,
    pub src_operand: Operand,

} 
impl MemoryUnit {
    pub fn new() -> Self {
        Self {
            flags: Flags::default(),
            ip: 0,
            sp: 0xFFFE,
            bp: 0,
            si: 0,
            di: 0,
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            reference: 0,
            ref_is_reg: true,
            memory: MemorySegments::new(),
            inst_bus: Buffer::new(6),
            segment: Segment::CS,
            opcodes: HashMap::new(),
            dst_operand: Operand::None,
            src_operand: Operand::None

        }
    }

    /**
     * decodes a reg code and returns a Operand::Register containing it.
     * @param reg_code: code of the register.
     * @param word: is the register should be a world register.
     * @return: a mutable reference to a register.
     */
    pub fn get_reg(&mut self, mut reg_code: u8, regfd: ModrmRegField, word: bool) -> Operand {
        // if not word, get only general usage registers
        let level = reg_code > 3;
        if !word {
            reg_code &= 0b011;
        }
        
        let reg = match regfd {
            ModrmRegField::Reg => {
                // getting the register
                match reg_code {
                    
                    // general regs
                    Registers::AX => &mut self.ax,
                    Registers::CX => &mut self.cx,
                    Registers::DX => &mut self.dx,
                    Registers::BX => &mut self.bx,
                    Registers::SP => &mut self.sp,
                    Registers::BP => &mut self.bp,
                    Registers::SI => &mut self.si,
                    Registers::DI => &mut self.di,
                    
                    // segment registers
                    Registers::RCS => &mut self.memory.code_segment,
                    Registers::RDS => &mut self.memory.data_segment,
                    Registers::RSS => &mut self.memory.stack_segment,
                    Registers::RES => &mut self.memory.extra_segment,

                    _ => panic!("wrong reg value"),
                }
            },
            ModrmRegField::Sreg => {
                match reg_code {
                    Registers::CS => &mut self.memory.code_segment,
                    Registers::DS => &mut self.memory.data_segment,
                    Registers::SS => &mut self.memory.stack_segment,
                    Registers::ES => &mut self.memory.extra_segment,
                    _ => panic!("wrong sreg value")
                }
            },
            _ => panic!("can't decode `Group` as reg field")
        };
        if !word {
            return Operand::SmallRegister(reg, level);
        }
        Operand::Register(reg)
    }

    /**
     * This function returns the addressed operand decoded in ModR/M style.
     */
    pub fn get_addressing_operand(&mut self, modf: u8, rm: u8, word: bool) -> Operand {
        // if mod 3, it's a register
        if modf == 3 {
            return self.get_reg(rm, ModrmRegField::Reg, word)
        }
        // gets the dissplacement
        else {
            let dis: u16 = match modf {
                0 => 0,
                // 8 bit diss
                1 => {
                    self.get_data(1).get() as u16
                }
                // 16 bit diss
                2 => {
                    self.get_data(2).get() as u16
                }
                _ => panic!("Illegal mod field"),
            };

            // gets the address from a general register
            let address = match rm  {
                0b000 => self.bx + self.si,
                0b001 => self.bx + self.di,
                0b010 => self.bp + self.si,
                0b011 => self.bp + self.di,
                0b100 => self.si,
                0b101 => self.di,
                0b110 => {
                    if dis == 0 {
                        self.get_data(2).get() as u16
                    } else {
                        self.bp
                    }
                }
                0b111 => self.bx,
                _ => panic!("illegal R/M field"),
            };
            let address = address + dis;
            
            // create the MemoryPointer
            let len: usize = if word{2} else {1};
            let pointer =  MemoryPointer::new(len as u8,
            self.memory.get_memory_pointer(&self.segment, address));
            
            Operand::Memory(pointer)
        }
    }

    /**
     * The function returns `Data` of the instrunction.
     * @word: a boolean which expresses if the data is byte or word long.
     */
    pub fn get_data(&mut self, len: u8) -> Operand {
        let mut val = self.inst_bus.remove().unwrap() as u32;
        for i in 1..len {
            let byte = self.inst_bus.remove().unwrap()as u32;
            val += byte << i * 8;
        }
        return Operand::Data(val);
    }

    /**
     * The function gets an InctrutionFormat and returns the values of the operands
     * @param inst: InstructionFormat reference.
     * @return: tuple which 2 values (dst operand, src operand)
     */
    pub fn read(&mut self, inst: &InstructionFormat) -> (Operand, Operand) {
        let (mut dst, mut src) = match &inst.operand_type {
            //#     Data
            &OperandType::Data(reg) => {
                let dst = self.get_reg(reg, ModrmRegField::Reg, inst.word);
                let src = self.get_data(
                    if inst.word {2} else {1}
                );
                match inst.instrution_type {
                    InstructionType::Procedure =>(dst, src),
                    _ => {
                        if inst.direction { (src, dst)}
                        else { (dst, src)}
                    }
                }
            }
            //#     Address
            &OperandType::Address(reg) => {
                let dst = self.get_reg(reg, ModrmRegField::Reg, inst.word);
                let src = self.get_addressing_operand(0, VARIABLE, inst.word);
                if inst.direction { (src, dst) }
                else { (dst, src) }
            }
            //#     ModR/M
            &OperandType::ModRM(regfd, data) => {
                // get all the fields
                let mod_rm = self.inst_bus.remove().unwrap();
                let modf = (mod_rm & 0b11000000) >> 6;
                let regf = (mod_rm & 0b00111000) >> 3;
                let rmf = mod_rm & 0b00000111;
                
                // get operands value
                let mut dst = self.get_addressing_operand(modf, rmf, inst.word);
                let mut src = match regfd {
                    ModrmRegField::Group(group) => {
                        if group == 0 {Operand::None}
                        else if group == 1 {Operand::Data(1)}
                        else if group == 2 {Operand::Data(self.cx as u32 & 0xFF)}
                        else {Operand::None}
                    },
                    _ => self.get_reg(regf, regfd, inst.word)
                    // let group = match  refgd { ModrmRegField::Group(group) => group, _ => 0};
                    // if group == 1 {Operand::Data(1)}
                    // else if group == 1  {Operand::Data(self.cx & 0x00FF)}
                    // else {Operand::None}
                };
                
                // if direction, switch operands value
                // direction true - reg is destination. else reg is source.
                if inst.direction {
                    self.reference = regf as u16;
                    let t = dst;
                    dst = src;
                    src = t;
                }

                // if data, load data in to the second (source) operand
                if data {
                    src = self.get_data(
                        if inst.word {2} else {1}
                    );
                }
                (dst, src)
            }
            //#     Registers
            &OperandType::Registers(reg1, reg2) => {
                let dst = self.get_reg(reg1,ModrmRegField::Reg, inst.word);
                let src = match reg2 {
                    Some(reg) =>self.get_reg(reg,ModrmRegField::Reg, inst.word),
                    None => Operand::None,
                };
                if inst.direction { (src, dst)}
                else { (dst, src)}
            }
            OperandType::None => (Operand::None,Operand::None),
        };

        // special case for different instruction types
        match &inst.instrution_type {
            // if program flow, dst supposd to be the IP register
            &InstructionType::ProgramFlow(far) => {
                dst = Operand::Register(&mut self.ip);
                if far {
                    let val = (self.get_data(2).get() << 16) + src.get();
                    src = Operand::Data(val);

                }
            },
            InstructionType::Stack => {
                // PUSH
                if inst.direction {
                    let stack_ref = Operand::Memory(MemoryPointer::new(
                        2,
                        self.memory.get_memory_pointer(&Segment::SS, self.sp -2)));
                    dst = stack_ref; 
                }
                // POP
                else { 
                    let stack_ref = Operand::Memory(MemoryPointer::new(
                        2,
                        self.memory.get_memory_pointer(&Segment::SS, self.sp)));
                    src = stack_ref; 
                }
            },
            InstructionType::Procedure  => {
                dst = Operand::Register(&mut self.ip);
                // RET
                if !inst.direction {
                    // some cases pop out a number of words
                    self.sp += src.get() as u16 * 2;
                    let stack_ref = Operand::Memory(MemoryPointer::new(
                        2,
                        self.memory.get_memory_pointer(&Segment::SS, self.sp)));
                    src = stack_ref; 
                }
            },
            InstructionType::Math => {
                dst = Operand::Data(((self.dx as u32) << 16 ) + self.ax as u32);
                self.flags.res1 = inst.word;
            },
            _ => ()
        }
        (dst, src)
    }

    /**
     * Helper funciton which reads bytes from the code segment and loads it
     * into the instruction_bus
     * @param count: amount of bytes to load
     */
    pub fn read_code_segment(&mut self, count: u8) {
        for _i in 0..count {
            let code = self.memory[(self.memory.code_segment, self.ip)];
            self.inst_bus.add(code);
            self.ip += 1;
        }
    }
}
