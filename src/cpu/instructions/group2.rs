
use super::*;

/**
 * This function gets a code and a Group `InstructionFormat` and assign new execute
 * closure to the instruction.
 * @param code: a 3 bits code representing the instruction.
 * @param inst: mutable reference to the InstructionFormat.
 */
pub fn group2(code: u8, inst: &mut InstructionFormat){
    inst.execute = match code {
        // ROL
        0 => Some(Box::new(|dst, src, flags| dst.rotate_left(src as u32))),

        // ROR
        1 => Some(Box::new(|dst, src, flags| dst.rotate_right(src as u32))),
        
        // RCL
        2 => Some(Box::new(|dst, src, flags| {let carry = flags.carry;
            dst.rotate_left(src as u32);
            if carry {dst.wrapping_add(1);}
            dst})),
                
        // RCR
        3 => Some(Box::new(|dst, src, flags| {let carry = flags.carry;
            dst.rotate_right(src as u32);
            if carry {dst.wrapping_add(1)}
            else {dst}})),
                
        // SHL
        4 => Some(Box::new(|dst, src, flags| dst << src)),
        
        // SHR
        5 => Some(Box::new(|dst, src, flags| dst >> src)),
        
        // SAR
        7 => Some(Box::new(|dst, src, flags| (dst as i32 >> src) as u32)),
        _ => panic!("error tranlsating bonus bytes to instruciton")
    };
}

/**
 * Sets group2 instructions
 * @param instructions_map: mutable reference to the vector containing the isntructions.
 */
pub fn group2_instructions(instructions_map: &mut Vec<(u8, InstructionFormat)>) {
    instructions_map.push(
        //MARK: 0xD0    GRP2 Eb 1
        (0xD0, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Group(1), false),
            instrution_type: InstructionType::Group(2),
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
    instructions_map.push(
        //MARK: 0xD1    GRP2 Ew 1
        (0xD1, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Group(1), false),
            instrution_type: InstructionType::Group(2),
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
    instructions_map.push(
        //MARK: 0xD2    GRP2 Eb CL
        (0xD2, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Group(2), false),
            instrution_type: InstructionType::Group(2),
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
    instructions_map.push(
        //MARK: 0xD3    GRP2 Ew CL
        (0xD3, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Group(2), false),
            instrution_type: InstructionType::Group(2),
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    )
}