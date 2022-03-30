
use super::*;

/**
 * This function gets a code and a Group `InstructionFormat` and assign new execute
 * closure to the instruction.
 * @param code: a 3 bits code representing the instruction.
 * @param inst: mutable reference to the InstructionFormat.
 */
pub fn group4(code: u8, inst: &mut InstructionFormat){
    inst.execute = match code {
        // INC
        0 => Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),

        // DEC
        1 => Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),
        
        _ => panic!("error tranlsating bonus bytes to instruciton")
    };
}

/**
 * Sets group4 instructions
 * @param instructions_map: mutable reference to the vector containing the isntructions.
 */
pub fn group4_instructions(instructions_map: &mut Vec<(u8, InstructionFormat)>) {
    instructions_map.push(
        //MARK: 0xFE    GRP4 Eb
        (0xFE, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Group(0), false),
            instrution_type: InstructionType::Group(4),
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
}