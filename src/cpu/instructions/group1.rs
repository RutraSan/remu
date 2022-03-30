use super::*;

/**
 * This function gets a code and a Group `InstructionFormat` and assign new execute
 * closure to the instruction.
 * @param code: a 3 bits code representing the instruction.
 * @param inst: mutable reference to the InstructionFormat.
 */
pub fn group1(code: u8, inst: &mut InstructionFormat){
    inst.execute = match code {
        // ADD
        0 => Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),

        // OR
        1 => Some(Box::new(|dst, src, flags| dst | src)),
        
        // ADC
        2 => Some(Box::new(|dst, src, flags| dst.wrapping_add(
                src.wrapping_add(flags.carry as u32)))),
                
        // SSB
        3 => Some(Box::new(|dst, src, flags| dst.wrapping_sub(
                src.wrapping_sub(flags.carry as u32)))),
                
        // AND
        4 => Some(Box::new(|dst, src, flags| dst & src)),
        
        // SUB
        5 => Some(Box::new(|dst, src, flags| dst.wrapping_sub(src))),
        
        // XOR
        6 => Some(Box::new(|dst, src, flags| dst ^ src)),
        
        // CMP
        7 => Some(Box::new(|dst, src, flags| {
                let (res, overflow) = dst.overflowing_sub(src);
                flags.carry = dst as u32 + src as u32 | 0x10000 == 1;
                flags.parity =res.count_ones() % 2 == 0;
                flags.auxilarity = dst >> 4 == res >> 4;
                flags.zero = res == 0;
                flags.sign = res >= 0x8000;
                flags.overflow = overflow;
                0 })),
        _ => panic!("error tranlsating bonus bytes to instruciton")
    };
    inst.write = if code == 7 {false} else {true};

}

/**
 * Sets group1 instructions.
 * @param instructions_map: mutable reference to the vector containing the isntructions.
 */
pub fn group1_instructions(instructions_map: &mut Vec<(u8, InstructionFormat)>) {
    instructions_map.push(
        //MARK: 0x80    GRP1 Eb Ib
        (0x80, InstructionFormat{
                operand_type: OperandType::ModRM(ModrmRegField::Group(0), true),
                instrution_type: InstructionType::Group(1),
                word: false,
                direction: false,
                write: true,
                execute: None,
                decode: None,
        })
    );
    instructions_map.push(
        //MARK: 0x81    GRP1 Ew Iw
        (0x81, InstructionFormat{
                operand_type: OperandType::ModRM(ModrmRegField::Group(0), true),
                instrution_type: InstructionType::Group(1),
                word: true,
                direction: false,
                write: true,
                execute: None,
                decode: None,
        })
    );   
    instructions_map.push(
        //MARK: 0x82    GRP1 Eb Ib
        (0x82, InstructionFormat{
                operand_type: OperandType::ModRM(ModrmRegField::Group(0), true),
                instrution_type: InstructionType::Group(1),
                word: false,
                direction: false,
                write: true,
                execute: None,
                decode: None,
        })
    );   
    instructions_map.push(
        //MARK: 0x83    GRP1 Ew Ib
        (0x83, InstructionFormat{
                operand_type: OperandType::ModRM(ModrmRegField::Group(0), true),
                instrution_type: InstructionType::Group(1),
                word: false,
                direction: false,
                write: true,
                execute: None,
                decode: None,
        }),
    );       
}