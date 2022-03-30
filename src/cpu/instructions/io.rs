use super::*;
/**
 * Sets io instructions
 * @param instructions_map: mutable reference to the vector containing the isntructions.
 */
pub fn io_instructions(instructions_map: &mut Vec<(u8, InstructionFormat)>) {
    //MARK: 0xE4    IN AL Ib
    instructions_map.push(
        (0xE4, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::IO,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
    //MARK: 0xE5    IN AX Ib
    instructions_map.push(
        (0xE5, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::IO,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
     //MARK: 0xE6    OUT Ib AL
     instructions_map.push(
        (0xE6, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::IO,
            word: false,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        })
    );
    //MARK: 0xE7    OUT Ib AX
    instructions_map.push(
        (0xE7, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::IO,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        })
    );
    //MARK: 0xEC    IN AL DX
    instructions_map.push(
        (0xEC, InstructionFormat{
            operand_type: OperandType::Registers(Registers::AL, Some(Registers::DX)),
            instrution_type: InstructionType::IO,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
    //MARK: 0xED    IN AX DX
    instructions_map.push(
        (0xED, InstructionFormat{
            operand_type: OperandType::Registers(Registers::AX, Some(Registers::DX)),
            instrution_type: InstructionType::IO,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
     //MARK: 0xEE    OUT DX AL
     instructions_map.push(
        (0xEE, InstructionFormat{
            operand_type: OperandType::Registers(Registers::AL, Some(Registers::DX)),
            instrution_type: InstructionType::IO,
            word: false,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        })
    );
    //MARK: 0xEF    OUT DX AX
    instructions_map.push(
        (0xEF, InstructionFormat{
            operand_type: OperandType::Registers(Registers::AX, Some(Registers::DX)),
            instrution_type: InstructionType::IO,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        })
    );
}