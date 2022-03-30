
use super::*;

/**
 * This function gets a code and a Group `InstructionFormat` and assign new execute
 * closure to the instruction.
 * @param code: a 3 bits code representing the instruction.
 * @param inst: mutable reference to the InstructionFormat.
 */
pub fn group3(code: u8, inst: &mut InstructionFormat){
    inst.execute = match code {
        // TEST
        0 => Some(Box::new(|dst, src, flags| dst & src)),
        
        // NOT
        2 => Some(Box::new(|dst, src, flags| !dst)),
                
        // NEG
        3 => Some(Box::new(|dst, src, flags| { 
            ((dst as i32) * -1) as u32
        })),
                
        // MUL
        4 => Some(Box::new(|dst, src, flags| { dst * src})),
        
        // IMUL
        5 => Some(Box::new(|dst, src, flags| {
            (dst as i32 * src as i32) as u32
        })),
        
        // DIV
        6 => Some(Box::new(|dst, src, flags| { 
            let div = dst / src;
            if flags.res1 {
                div + ((dst % src) << 16) as u32
            }
            else {
                div + ((dst % src) << 8) as u32
            }
        })),
        
        // IDIV
        7 => Some(Box::new(|dst, src, flags| { 
            let div = (dst as i16 / src as i16) as i32;
            if flags.res1 {
                (div + (((dst as i16 % src as i16) as i32 + 1) << 16)) as u32
            }
            else {
                (div + (((dst as i16 % src as i16) as i32 + 1) << 8)) as u32
            }
        })),
        _ => panic!("error tranlsating bonus bytes to instruciton")
    };

    // some setting chagnes
    if code == 0 {
        inst.write = false;
        inst.operand_type = OperandType::ModRM(ModrmRegField::Group(0), true);
    }
    else {
        inst.write = true;
        inst.operand_type = OperandType::ModRM(ModrmRegField::Group(0), false);
    }
    if code < 4 {
        inst.instrution_type = InstructionType::General;
        inst.direction = false;
    } 
    else {
        inst.instrution_type = InstructionType::Math;
        inst.direction = true;
    };
}

/**
 * Sets group2 instructions
 * @param instructions_map: mutable reference to the vector containing the isntructions.
 */
pub fn group3_instructions(instructions_map: &mut Vec<(u8, InstructionFormat)>) {
    instructions_map.push(
        //MARK: 0xF6    GRP3 Eb
        (0xF6, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Group(0), false),
            instrution_type: InstructionType::Group(3),
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
    instructions_map.push(
        //MARK: 0xF7    GRP3 Ew
        (0xF7, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Group(0), false),
            instrution_type: InstructionType::Group(3),
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        })
    );
}