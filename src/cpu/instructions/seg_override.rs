use super::*;

/**
 * Sets segment override prefixes
 * @param instructions_map: mutable reference to the vector containing the isntructions.
 */
pub fn segment_override_instructions(instructions_map: &mut Vec<(u8, InstructionFormat)>) {
    instructions_map.push(
        //MARK: 0x26    ES:
        (0x26, InstructionFormat{
            operand_type: OperandType::Registers(Registers::ES, None),
            instrution_type: InstructionType::SegmentOverride,
            word: false,
            direction: false,
            write: false,
            execute: None,
            decode: None,
        }),
    );
    instructions_map.push(
        //MARK: 0x2E    CS:
        (0x2E, InstructionFormat{
            operand_type: OperandType::Registers(Registers::CS, None),
            instrution_type: InstructionType::SegmentOverride,
            word: false,
            direction: false,
            write: false,
            execute: None,
            decode: None,
        }),
    );
    instructions_map.push(
        //MARK: 0x36    SS:
        (0x36, InstructionFormat{
            operand_type: OperandType::Registers(Registers::SS, None),
            instrution_type: InstructionType::SegmentOverride,
            word: false,
            direction: false,
            write: false,
            execute: None,
            decode: None,
        }),
    );
    instructions_map.push(
        //MARK: 0x3E    DS:
        (0x3E, InstructionFormat{
            operand_type: OperandType::Registers(Registers::DS, None),
            instrution_type: InstructionType::SegmentOverride,
            word: false,
            direction: false,
            write: false,
            execute: None,
            decode: None,
        }),
    );
}