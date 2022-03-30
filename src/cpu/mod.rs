#![allow(dead_code)]
#![allow(unused_must_use)]

use helperModules::*;
use instructions::*;
use memory_unit::{MemoryUnit};
use memory_unit::memory_segments::{MemoryPointer, Segment};

use std::{collections::HashMap};
use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver}};
use queues::*;

pub mod helperModules;
pub mod memory_unit;
mod instructions;

/**
 * cpu sturct
 */
pub struct CPU {
    pub memory_unit: MemoryUnit,
    pub opcodes: HashMap<u8, InstructionFormat>,
    ports: Arc<Mutex<Vec<u8>>>,
    update_port: (Sender<(u16, bool)>, Receiver<u16>),
    update_group: (bool, u8)
}

impl CPU {
    // contructor
    pub fn new(ports: Arc<Mutex<Vec<u8>>>, update_port: (Sender<(u16, bool)>, Receiver<u16>)) -> Self {
        let mut cpu = Self {
            memory_unit: MemoryUnit::new(),
            opcodes: HashMap::new(),
            ports: ports,
            update_port: update_port,
            update_group: (false, 0),
        };
        // initialize memory components
        cpu.memory_unit.flags.set(2);

        // IP is initialized with the code 0xFFF0.
        // and CS is initialized with 0xF000. The physical address is 0xFFFF0.
        // The upper 16 bytes contain an initalization code for the cpu.
        // usaully contaning a JMP instruction to an actual initalization program.
        cpu.memory_unit.ip = 0xFFF0;
        cpu.memory_unit.memory.code_segment = 0xF000;
        cpu
    }

    /**
     * This function creates a new CPU and initalizes it.
     * @param program: reference to a vector containing the program.
     * @param init_program: Optional. first 16 bytes starting up the CPU.
     * @param location: Optional. location of users program.
     */
    pub fn init(program: &Vec<u8>,
        ports: Arc<Mutex<Vec<u8>>>,
        update_port: (Sender<(u16, bool)>, Receiver<u16>), 
        init_program: Option<&Vec<u8>>, 
        location: Option<&Vec<u8>>,) -> Self {
        let mut cpu = Self::new(ports, update_port);

        // get isntructionFormat map
        cpu.opcodes = instructions::instructions_decode()
            .into_iter()
            .collect();

        // choose init program
        let ini: Vec<u8> = match location {
            Some(add) => {
                let mut v = [0xea].to_vec();
                v.append(&mut add.to_vec());
                v
            },
            None => vec![0xea, 0x00, 0x01, 0xef, 0xde],
        };
        // load the program
        cpu.memory_unit.memory.write(match init_program {
            Some(program) => program,
            None => &ini
        }, 
        (cpu.memory_unit.memory.code_segment, cpu.memory_unit.ip));
        cpu.memory_unit.memory.write(program, (0xdeef, 0x100));

        cpu
        
    }

    /**
     * This function runs the CPU.
     */
    pub fn run_program(&mut self) -> Result<(), String> {
        loop {
            let res = self.run_next_instruction();
            // Halted
            if res == Ok(0xF4 as u8)  {
                return Ok(())
            }
            // if Error
            if res.is_err() {
                return Err(res.unwrap_err())
            }
        }
    }

    /**
     * Runs the next intruction in the code.
     */
    pub fn run_next_instruction(&mut self) -> Result<u8, String>{
        // load and get instruction
        self.load_instruction();
        
        // get instructionFormat
        let opcode = self.memory_unit.inst_bus.remove().unwrap();
        let inst = self.opcodes.get(&opcode).unwrap();

        // reading the memory and get operands
        let (mut dst, src) = self.memory_unit.read(inst);

        //# EXECUTION UNIT

        // getting copy of freg
        let mut flags = self.memory_unit.flags;
        
        // execute the algorithem of the isnstruction
        let res = match &inst.execute {
            Some(exec) => exec(dst.get(), src.get(), &mut flags),
            None => src.get()
        };

        // write the result
        if inst.write { 
            match &inst.instrution_type {
                InstructionType::Stack => {
                    dst.set(res as u32);
                    // PUSH
                    if inst.direction { self.memory_unit.sp -= 2; }
                    // POP
                    else { self.memory_unit.sp += 2; }
                },
                InstructionType::Procedure => {
                    // CALL
                    if inst.direction {
                        // push IP
                        self.memory_unit.sp -=2;
                        let mut stack_ref = MemoryPointer::new(
                            2,
                            self.memory_unit.memory.get_memory_pointer(&Segment::SS, self.memory_unit.sp));
                        // sets the ip    
                        stack_ref.set(dst.get());
                    }
                    // RET
                    else {
                        self.memory_unit.sp +=2;
                    }
                    dst.set(res);
                },
                InstructionType::Math => {
                    self.memory_unit.ax = res as u16;
                    if inst.word {
                        self.memory_unit.dx = (res >> 16) as u16;
                    }
                },
                InstructionType::Interrupt => {
                    // IRET
                    if inst.direction {
                        // pop cs:ip
                        let stack_ref = MemoryPointer::new(
                            4,
                            self.memory_unit.memory.get_memory_pointer(&Segment::SS, self.memory_unit.sp));
                        let cs_ip = stack_ref.get();
                        self.memory_unit.ip = cs_ip as u16;
                        self.memory_unit.memory.code_segment = (cs_ip >> 16) as u16;
                        self.memory_unit.sp += 4;
                        
                        // pop freg
                        let stack_ref = MemoryPointer::new(
                            2,
                            self.memory_unit.memory.get_memory_pointer(&Segment::SS, self.memory_unit.sp));
                        let freg = stack_ref.get();
                        self.memory_unit.flags.set(freg as u16);
                        self.memory_unit.sp += 2;
                    }
                    // INT
                    else {
                         // pushes freg   
                        self.memory_unit.sp -=2;
                        let mut stack_ref = MemoryPointer::new(
                            2,
                            self.memory_unit.memory.get_memory_pointer(&Segment::SS, self.memory_unit.sp));
                        stack_ref.set(self.memory_unit.flags.as_word() as u32);
                        
                        // pushes cs:ip
                        self.memory_unit.sp -=4;
                        let mut stack_ref = MemoryPointer::new(
                            4,
                            self.memory_unit.memory.get_memory_pointer(&Segment::SS, self.memory_unit.sp));
                        let mut cs_ip = self.memory_unit.memory.code_segment as u32;
                        cs_ip = (cs_ip << 16) + self.memory_unit.ip as u32;
                        stack_ref.set(cs_ip);

                        // clear IF and TF
                        self.memory_unit.flags.interrupt =false;
                        self.memory_unit.flags.trap =false;
                        
                        // get address from IVT
                        let interrupt_vector = MemoryPointer::new(
                            4,
                            self.memory_unit.memory.get_memory_pointer(&Segment::IVT, src.get() as u16 * 4));
                        let interrupt_address = interrupt_vector.get();

                        // set new cs:ip
                        self.memory_unit.memory.code_segment = interrupt_address as u16;
                        self.memory_unit.ip = (interrupt_address >> 16) as u16;
                    }
                }
                &InstructionType::ProgramFlow(far) => {
                    // check if far jump
                    if far {
                        self.memory_unit.memory.code_segment = (res >> 16) as u16;
                    }
                    dst.set(res);
                }
                // assign to port
                &InstructionType::IO => {
                    let mut ports = self.ports.lock().unwrap();
                    // OUT
                    if inst.direction {
                        ports[dst.get() as usize] = src.get() as u8;
                        // assign next if word
                        if inst.word {
                            ports[(dst.get()+1) as usize] = (src.get() >> 8) as u8;
                        }
                        self.update_port.0.send((dst.get() as u16, inst.word));
                        drop(ports);
                        self.update_port.1.recv();
                    }
                    // IN
                    else {
                        let mut val:u32 = ports[src.get() as usize] as u32;
                        if inst.word {
                            val += (ports[(src.get()+1) as usize] as u32) << 8;
                        }
                        dst.set(val);
                    }
                }
                _ => dst.set(res)
            }
        }
        // assign new flags value
        self.memory_unit.flags = flags;

        // the instruction might be an updated Group instruction.
        // should update it back to being GroupInstruction.
        if self.update_group != (false, 0) {
            let (_, group_number) = self.update_group;
            let inst = self.opcodes.get_mut(&opcode).unwrap();
            self.update_group = (false, 0);
            inst.instrution_type = InstructionType::Group(group_number);
        }
        Ok(opcode)
    }

    /**
     * This function loads the instruction into the inst_bus of memory_unit.
     */
    pub fn load_instruction(&mut self) {
        // make sure inst_bus is clear
        while self.memory_unit.inst_bus.size() > 0 {
            self.memory_unit.inst_bus.remove();
        }
        let code_pointer = self.memory_unit.memory.get_memory_pointer(
            &memory_unit::memory_segments::Segment::CS, self.memory_unit.ip);

        // read instruction opcode
        self.memory_unit.read_code_segment(1);

        let opcode = self.memory_unit.inst_bus.peek().unwrap();
        let mut inst = self.opcodes.get_mut(&opcode).unwrap();

        // check for prefix or Group
        match inst.instrution_type {
            InstructionType::Prefix => todo!("implement prefixes"),
            // Group
            InstructionType::Group(group_number) => {
                let group_code = (unsafe{*code_pointer.add(1)}& 0b00111000 )>> 3;
                instructions::group_decode(group_number % 10, group_code, inst);
                self.update_group = (true, group_number);
            },
            InstructionType::SegmentOverride => {
                // overrdie the segment
                let segment = match inst.operand_type {
                    OperandType::Registers(dst, _) => dst,
                    _ => panic!("error doing segment override")
                };
                self.memory_unit.segment = Segment::from_code(segment).unwrap();

                // load the actual instruction
                self.memory_unit.inst_bus.remove();
                self.memory_unit.read_code_segment(1);
                let opcode = self.memory_unit.inst_bus.peek().unwrap();
                inst = self.opcodes.get_mut(&opcode).unwrap();
            }
            _ => (),
        }

        // load rest of the instruction
        match inst.operand_type {
            OperandType::Address(_) => self.memory_unit.read_code_segment(2),
            OperandType::Data(_) => self.memory_unit.read_code_segment(if inst.word {2}
                else {1}),
            OperandType::ModRM(_, data) => {
                // getting the modRM byte
                self.memory_unit.read_code_segment(1);
                self.memory_unit.inst_bus.remove();
                let modrm = self.memory_unit.inst_bus.remove().unwrap();
                self.memory_unit.ip -= 2;

                // getting the instruction and modRM back
                self.memory_unit.read_code_segment(2);
                
                // dissplacement
                self.memory_unit.read_code_segment(
                    if modrm > 0x3f && modrm < 0x80 {1}
                        else if modrm > 0x7f && modrm < 0xC0 {2}
                        else if modrm < 0x40 && modrm & 0b00000111 == 0b110 {2}
                        else {0}
                );
                if data {
                    self.memory_unit.read_code_segment(if inst.word {2} else {1});
                }
            },
            OperandType::None => (),
            OperandType::Registers(_,_) => (),
        }

        if inst.instrution_type == InstructionType::ProgramFlow(true) {
            self.memory_unit.read_code_segment(2);
        }
    }
}
