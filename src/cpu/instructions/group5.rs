
use super::*;

/**
 * This function gets a code and a Group `InstructionFormat` and assign new execute
 * closure to the instruction.
 * @param code: a 3 bits code representing the instruction.
 * @param inst: mutable reference to the InstructionFormat.
 */
pub fn group5(code: u8, inst: &mut InstructionFormat){
    inst.execute = match code {
        // INC
        0 => Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),

        // DEC
        1 => Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),

        // CALL 
        2 => Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),

        // CALL Mp
        3 => todo!("Group5 code 3 `CALL Mp`"),

        // JMP
        4 => {
            inst.instrution_type = InstructionType::ProgramFlow(false);
            Some(Box::new(|dst, src, flags| src))
        },

        // JMP Mp
        5 => {
            inst.instrution_type = InstructionType::ProgramFlow(true);
            Some(Box::new(|dst, src, flags| src))
        },

        // PUSH
        6 => {
            inst.instrution_type = InstructionType::Stack;
            Some(Box::new(|dst, src, flags| src))
        },
        
        _ => panic!("error tranlsating bonus bytes to instruciton")
    };
    inst.direction = if code == 6 {true} else {false};
    
}

/**
 * Sets group5 instructions
 * @param instructions_map: mutable reference to the vector containing the isntructions.
 */
pub fn group5_instructions(instructions_map: &mut Vec<(u8, InstructionFormat)>) {
    instructions_map.push(
        //MARK: 0xFF    GRP5 Iw
        (0xFF, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Group(0), false),
            instrution_type: InstructionType::Group(5),
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
}