#![allow(unused_variables)]
#![allow(unused_assignments)]

use super::helperModules::*;

pub mod group1;
pub mod group2;
pub mod group3;
pub mod group4;
pub mod group5;
pub mod seg_override;
pub mod io;

#[derive(PartialEq)]
pub enum InstructionType {
    General,
    Group(u8),
    Stack,
    // true - far jump
    ProgramFlow(bool),
    Prefix,
    Procedure,
    Math,
    Interrupt,
    SegmentOverride,
    IO,
}
// #[derive(Default)]
pub struct InstructionFormat {
    pub operand_type: OperandType,
    pub instrution_type: InstructionType,
    pub word: bool,
    pub direction: bool,
    pub write: bool,
    pub execute: Option<Box<dyn Fn(u32, u32, &mut Flags) -> u32>>,
    pub decode: Option<Vec<u8>>
}

/**
 * The function gets a group number and the bonus 3 bits opcode, with whom he updates
 * the InstructionFormat.
 * @param group_number: number of the group.
 * @param group_code: the bonus 3 bit opcode.
 * @param inst: mutable reference to the InstructionFormat.
 */
pub fn group_decode(group_number: u8, group_code: u8, inst: &mut InstructionFormat) {
    match group_number {
        1 => group1::group1(group_code, inst),
        2 => group2::group2(group_code, inst),
        3 => group3::group3(group_code, inst),
        4 => group4::group4(group_code, inst),
        5 => group5::group5(group_code, inst),

        _ => panic!("bad group number")
    }
}

/**
 * This function creates the InstruionFormat vector, which will then be turne into a map. 
 * @return: vecotr of tuples (u8, InstructionForamt)
 */
#[allow(unused_variables)]
pub fn instructions_decode() -> Vec<(u8, InstructionFormat)> {
    let mut instructions_map = Vec::<(u8, InstructionFormat)>::new();
    instructions_map = Vec::from([
        //MARK: 0x00    ADD Eb Gb
        (0x00, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),
            decode: None,
        }),
        //MARK: 0x01    ADD Ew Gw
        (0x01, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),
            decode: None,
        }),
        //MARK: 0x02    ADD Gb Eb
        (0x02, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),
            decode: None,
        }),
        //MARK: 0x03    ADD Gw Ew
        (0x03, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),
            decode: None,
        }),
        //MARK: 0x04    ADD AL, Ib
        (0x04, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),
            decode: None,
        }),
        //MARK: 0x05    ADD AX, Iw
        (0x05, InstructionFormat{
            operand_type: OperandType::Data(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),
            decode: None,
        }),
        //MARK: 0x06    PUSH ES
        (0x06, InstructionFormat{
            operand_type: OperandType::Registers(Registers::RES, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x07    POP ES
        (0x07, InstructionFormat{
            operand_type: OperandType::Registers(Registers::RES, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x08    OR Eb Gb
        (0x08, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst | src)),
            decode: None,
        }),
        //MARK: 0x09    OR Ew Gw
        (0x09, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst | src)),
            decode: None,
        }),
        //MARK: 0x0A    OR Gb Eb
        (0x0A, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst | src)),
            decode: None,
        }),
        //MARK: 0x0B    OR Gw Ew
        (0x0B, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst | src)),
            decode: None,
        }),
        //MARK: 0x0C    OR AL, Ib
        (0x0C, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst | src)),
            decode: None,
        }),
        //MARK: 0x0D    OR AX, Iw
        (0x0D, InstructionFormat{
            operand_type: OperandType::Data(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst | src)),
            decode: None,
        }),
        //MARK: 0x10    ADC Eb Gb
        (0x10, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(
                src.wrapping_add(
                    flags.carry as u32)))),
            decode: None,
        }),
        //MARK: 0x11    ADC Ew Gw
        (0x11, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(
                src.wrapping_add(
                    flags.carry as u32)))),
            decode: None,
        }),
        //MARK: 0x12    ADC Gb Eb
        (0x12, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(
                src.wrapping_add(
                    flags.carry as u32)))),
            decode: None,
        }),
        //MARK: 0x13    ADC Gw Ew
        (0x13, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(
                src.wrapping_add(
                    flags.carry as u32)))),
            decode: None,
        }),
        //MARK: 0x14    ADC AL, Ib
        (0x14, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(
                src.wrapping_add(
                    flags.carry as u32)))),
            decode: None,
        }),
        //MARK: 0x15    ADC AX, Iw
        (0x15, InstructionFormat{
            operand_type: OperandType::Data(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(
                src.wrapping_add(
                    flags.carry as u32)))),
            decode: None,
        }),
        //MARK: 0x18    SBB Eb Gb
        (0x018, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(
                src.wrapping_sub(flags.carry as u32)
            ))),
            decode: None,
        }),
        //MARK: 0x19    SBB Ew Gw
        (0x19, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(
                src.wrapping_sub(flags.carry as u32)
            ))),
            decode: None,
        }),
        //MARK: 0x1A    SBB Gb Eb
        (0x1A, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(
                src.wrapping_sub(flags.carry as u32)
            ))),
            decode: None,
        }),
        //MARK: 0x1B    SBB Gw Ew
        (0x1B, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(
                src.wrapping_sub(flags.carry as u32)
            ))),
            decode: None,
        }),
        //MARK: 0x1C    SBB AL, Ib
        (0x1C, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(
                src.wrapping_sub(flags.carry as u32)
            ))),
            decode: None,
        }),
        //MARK: 0x1D    SBB AX, Iw
        (0x1D, InstructionFormat{
            operand_type: OperandType::Data(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(
                src.wrapping_sub(flags.carry as u32)
            ))),
            decode: None,
        }),
        //MARK: 0x20    AND Eb Gb
        (0x20, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst & src)),
            decode: None,
        }),
        //MARK: 0x21    AND Ew Gw
        (0x21, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst & src)),
            decode: None,
        }),
        //MARK: 0x22    AND Gb Eb
        (0x22, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst & src)),
            decode: None,
        }),
        //MARK: 0x23    AND Gw Ew
        (0x23, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst & src)),
            decode: None,
        }),
        //MARK: 0x24    AND AL, Ib
        (0x24, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst & src)),
            decode: None,
        }),
        //MARK: 0x25    AND AX, Iw
        (0x25, InstructionFormat{
            operand_type: OperandType::Data(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst & src)),
            decode: None,
        }),
        //MARK: 0x28    SUB Eb Gb
        (0x28, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(src))),
            decode: None,
        }),
        //MARK: 0x29    SUB Ew Gw
        (0x29, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(src))),
            decode: None,
        }),
        //MARK: 0x2A    SUB Gb Eb
        (0x2A, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(src))),
            decode: None,
        }),
        //MARK: 0x2B    SUB Gw Ew
        (0x2B, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(src))),
            decode: None,
        }),
        //MARK: 0x2C    SUB AL, Ib
        (0x2C, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(src))),
            decode: None,
        }),
        //MARK: 0x2D    SUB AX, Iw
        (0x2D, InstructionFormat{
            operand_type: OperandType::Data(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(src))),
            decode: None,
        }),
        //MARK: 0x30    XOR Eb Gb
        (0x30, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst ^ src)),
            decode: None,
        }),
        //MARK: 0x31    XOR Ew Gw
        (0x31, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst ^ src)),
            decode: None,
        }),
        //MARK: 0x32    XOR Gb Eb
        (0x32, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst ^ src)),
            decode: None,
        }),
        //MARK: 0x33    XOR Gw Ew
        (0x33, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst ^ src)),
            decode: None,
        }),
        //MARK: 0x34    XOR AL, Ib
        (0x34, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst ^ src)),
            decode: None,
        }),
        //MARK: 0x35    XOR AX, Iw
        (0x35, InstructionFormat{
            operand_type: OperandType::Data(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst ^ src)),
            decode: None,
        }),
        //MARK: 0x38    CMP Eb Gb
        (0x38, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {
                let (res, overflow) = dst.overflowing_sub(src);
                flags.carry = dst as u32 + src as u32 | 0x10000 == 1;
                flags.parity =res.count_ones() % 2 == 0;
                flags.auxilarity = dst >> 4 == res >> 4;
                flags.zero = res == 0;
                flags.sign = res >= 0x8000;
                flags.overflow = overflow;
                0
            })),
            decode: None,
        }),
        //MARK: 0x39    CMP Ew Gw
        (0x39, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {
                let (res, overflow) = dst.overflowing_sub(src);
                flags.carry = dst as u32 + src as u32 | 0x10000 == 1;
                flags.parity =res.count_ones() % 2 == 0;
                flags.auxilarity = dst >> 4 == res >> 4;
                flags.zero = res == 0;
                flags.sign = res >= 0x8000;
                flags.overflow = overflow;
                0
            })),
            decode: None,
        }),
        //MARK: 0x3A    CMP Gb Eb
        (0x3A, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {
                let (res, overflow) = dst.overflowing_sub(src);
                flags.carry = dst as u32 + src as u32 | 0x10000 == 1;
                flags.parity =res.count_ones() % 2 == 0;
                flags.auxilarity = dst >> 4 == res >> 4;
                flags.zero = res == 0;
                flags.sign = res >= 0x8000;
                flags.overflow = overflow;
                0
            })),
            decode: None,
        }),
        //MARK: 0x3B    CMP Gw Ew
        (0x3B, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {
                let (res, overflow) = dst.overflowing_sub(src);
                flags.carry = dst as u32 + src as u32 | 0x10000 == 1;
                flags.parity =res.count_ones() % 2 == 0;
                flags.auxilarity = dst >> 4 == res >> 4;
                flags.zero = res == 0;
                flags.sign = res >= 0x8000;
                flags.overflow = overflow;
                0
            })),            decode: None,
        }),
        //MARK: 0x3C    CMP AL, Ib
        (0x3C, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {
                let (res, overflow) = dst.overflowing_sub(src);
                flags.carry = (res as i32) < 0;
                flags.parity =res.count_ones() % 2 == 0;
                flags.auxilarity = dst >> 4 == res >> 4;
                flags.zero = res == 0;
                flags.sign = res >= 0x8000;
                flags.overflow = overflow;
                0
            })),            decode: None,
        }),
        //MARK: 0x3D    CMP AX, Iw
        (0x3D, InstructionFormat{
            operand_type: OperandType::Data(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {
                let (res, overflow) = dst.overflowing_sub(src);
                flags.carry = dst as u32 + src as u32 | 0x10000 == 1;
                flags.parity =res.count_ones() % 2 == 0;
                flags.auxilarity = dst >> 4 == res >> 4;
                flags.zero = res == 0;
                flags.sign = res >= 0x8000;
                flags.overflow = overflow;
                0
            })),            decode: None,
        }),
        //MARK: 0x40    INC AX
        (0x40, InstructionFormat{
            operand_type: OperandType::Registers(Registers::AX, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),
            decode: None,
        }),
        //MARK: 0x41    INC CX
        (0x41, InstructionFormat{
            operand_type: OperandType::Registers(1, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),
            decode: None,
        }),
        //MARK: 0x42    INC DX
        (0x42, InstructionFormat{
            operand_type: OperandType::Registers(2, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),
            decode: None,
        }),
        //MARK: 0x43    INC BX
        (0x43, InstructionFormat{
            operand_type: OperandType::Registers(3, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),
            decode: None,
        }),
        //MARK: 0x44    INC SP
        (0x44, InstructionFormat{
            operand_type: OperandType::Registers(4, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),
            decode: None,
        }),
        //MARK: 0x45    INC BP
        (0x45, InstructionFormat{
            operand_type: OperandType::Registers(5, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),
            decode: None,
        }),
        //MARK: 0x46    INC SI
        (0x46, InstructionFormat{
            operand_type: OperandType::Registers(6, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),
            decode: None,
        }),
        //MARK: 0x47    INC DI
        (0x47, InstructionFormat{
            operand_type: OperandType::Registers(7, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(1))),
            decode: None,
        }),
        //MARK: 0x48    DEC AX
        (0x48, InstructionFormat{
            operand_type: OperandType::Registers(0, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),
            decode: None,
        }),
        //MARK: 0x49    DEC CX
        (0x49, InstructionFormat{
            operand_type: OperandType::Registers(1, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),
            decode: None,
        }),
        //MARK: 0x4A    DEC DX
        (0x4A, InstructionFormat{
            operand_type: OperandType::Registers(2, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),
            decode: None,
        }),
        //MARK: 0x4B    DEC BX
        (0x4B, InstructionFormat{
            operand_type: OperandType::Registers(3, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),
            decode: None,
        }),
        //MARK: 0x4C    DEC SP
        (0x4C, InstructionFormat{
            operand_type: OperandType::Registers(4, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),
            decode: None,
        }),
        //MARK: 0x4C    DEC BP
        (0x4D, InstructionFormat{
            operand_type: OperandType::Registers(5, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),
            decode: None,
        }),
        //MARK: 0x4E    DEC SI
        (0x4E, InstructionFormat{
            operand_type: OperandType::Registers(6, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),
            decode: None,
        }),
        //MARK: 0x4F    DEC DI
        (0x4F, InstructionFormat{
            operand_type: OperandType::Registers(7, None),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(1))),
            decode: None,
        }),
        //MARK: 0x50    PUSH AX
        (0x50, InstructionFormat{
            operand_type: OperandType::Registers(Registers::AX, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x51    PUSH CX
        (0x51, InstructionFormat{
            operand_type: OperandType::Registers(Registers::CX, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x52    PUSH DX
        (0x52, InstructionFormat{
            operand_type: OperandType::Registers(Registers::DX, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x53    PUSH BX
        (0x53, InstructionFormat{
            operand_type: OperandType::Registers(Registers::BX, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x54    PUSH SP
        (0x54, InstructionFormat{
            operand_type: OperandType::Registers(Registers::SP, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x55    PUSH BP
        (0x55, InstructionFormat{
            operand_type: OperandType::Registers(Registers::BP, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x56    PUSH SI
        (0x56, InstructionFormat{
            operand_type: OperandType::Registers(Registers::SI, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x57    PUSH DI
        (0x57, InstructionFormat{
            operand_type: OperandType::Registers(Registers::DI, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x58    POP AX
        (0x58, InstructionFormat{
            operand_type: OperandType::Registers(Registers::AX, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x59    POP CX
        (0x59, InstructionFormat{
            operand_type: OperandType::Registers(Registers::CX, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x5A    POP DX
        (0x5A, InstructionFormat{
            operand_type: OperandType::Registers(Registers::DX, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x5B    POP BX
        (0x5B, InstructionFormat{
            operand_type: OperandType::Registers(Registers::BX, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x5C    POP SP
        (0x5C, InstructionFormat{
            operand_type: OperandType::Registers(Registers::SP, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x5D    POP BP
        (0x5D, InstructionFormat{
            operand_type: OperandType::Registers(Registers::BP, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x5E    POP SI
        (0x5E, InstructionFormat{
            operand_type: OperandType::Registers(Registers::SI, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x5F    POP DI
        (0x5F, InstructionFormat{
            operand_type: OperandType::Registers(Registers::DI, None),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x68    PUSH Iw
        (0x68, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x6A    PUSH Ib
        (0x6A, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::Stack,
            word: false,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0X70    JO
        (0x70, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if flags.overflow {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X71    JNO
        (0x71, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if !flags.overflow {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X72    JB
        (0x72, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if flags.carry {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X73    JAE
        (0x73, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if !flags.carry {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X74    JE
        (0x74, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if flags.zero {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X75    JNE / JNZ
        (0x75, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if !flags.zero {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X76    JBE
        (0x76, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if flags.zero || flags.carry {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X77    JA
        (0x77, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if !flags.zero && !flags.carry {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X78    JS
        (0x78, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if flags.sign {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X79    JNS
        (0x79, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if !flags.sign {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X7A    JPE / JP
        (0x7A, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if flags.parity {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X7B    JPO
        (0x7B, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if !flags.parity {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X7C    JL
        (0x7C, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if flags.sign != flags.overflow {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X7D    JGE
        (0x7D, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if flags.sign == flags.overflow {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X7E    JLE
        (0x7E, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if flags.zero || flags.sign != flags.overflow {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0X7E    JNLE
        (0x7F, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags|{
                if !flags.zero && flags.sign == flags.overflow {((dst as i16).wrapping_add(src as i8 as i16)) as u32}
                else {dst}
            })),
            decode: None,
        }),
        //MARK: 0x84    TEST Eb Gb
        (0x84, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| dst & src)),
            decode: None,
        }),
        //MARK: 0x85    TEST Ew Gw
        (0x85, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| dst & src)),
            decode: None,
        }),
        //TODO: XHCG
        //MARK: 0x88    MOV Eb Gb
        (0x88, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x89    MOV Ew Gw
        (0x89, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x8A    MOV Gb Eb
        (0x8A, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x8B    MOV Gw Ew
        (0x8B, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x8C    MOV Ew Sw
        (0x8C, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Sreg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x8E    MOV Sw Ew
        (0x8E, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Sreg, false),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x8F    POP Ew
        (0x8F, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, false),
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0x90    NOP
        (0x90, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: None,
            decode: None,
        }),
        //MARK: 0x9C    PUSHFֵ
        (0x9C, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::Stack,
            word: true,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| flags.as_word() as u32)),
            decode: None,
        }),
        //MARK: 0x9D    POPF
        (0x9D, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::Stack,
            word: true,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {flags.set(src as u16); 0})),
            decode: None,
        }),
        //MARK: 0xA0    MOV AL, Ob
        (0xA0, InstructionFormat{
            operand_type: OperandType::Address(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xA1    MOV AX, Ow
        (0xA1, InstructionFormat{
            operand_type: OperandType::Address(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xA2    MOV Ob, AL
        (0xA2, InstructionFormat{
            operand_type: OperandType::Address(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xA3    MOV Ow, AX
        (0xA3, InstructionFormat{
            operand_type: OperandType::Address(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xA8    TEST AL, Ib
        (0xA8, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(src))),
            decode: None,
        }),
        //MARK: 0xA9    TEST AX, Iw
        (0xA9, InstructionFormat{
            operand_type: OperandType::Data(Registers::AX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_sub(src))),
            decode: None,
        }),
        //MARK: 0xB0    MOV AL, Ib
        (0xB0, InstructionFormat{
            operand_type: OperandType::Data(Registers::AL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xB1    MOV CL, Ib
        (0xB1, InstructionFormat{
            operand_type: OperandType::Data(Registers::CL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xB2    MOV BL, Ib
        (0xB2, InstructionFormat{
            operand_type: OperandType::Data(Registers::BL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xB3    MOV DL, Ib
        (0xB3, InstructionFormat{
            operand_type: OperandType::Data(Registers::DL),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xB4    MOV AH, Ib
        (0xB4, InstructionFormat{
            operand_type: OperandType::Data(Registers::AH),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xB5    MOV CH, Ib
        (0xB5, InstructionFormat{
            operand_type: OperandType::Data(Registers::CH),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xB6    MOV DH, Ib
        (0xB6, InstructionFormat{
            operand_type: OperandType::Data(Registers::DH),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xB7    MOV BH, Ib
        (0xB7, InstructionFormat{
            operand_type: OperandType::Data(Registers::BH),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xB8    MOV AX, Iw
        (0xB8, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xB9    MOV CX, Iw
        (0xB9, InstructionFormat{
            operand_type: OperandType::Data(Registers::CX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xBA    MOV DX, Iw
        (0xBA, InstructionFormat{
            operand_type: OperandType::Data(Registers::DX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xBB    MOV BX, Iw
        (0xBB, InstructionFormat{
            operand_type: OperandType::Data(Registers::BX),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xBC    MOV SP, Iw
        (0xBC, InstructionFormat{
            operand_type: OperandType::Data(Registers::SP),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xBD    MOV BP, Iw
        (0xBD, InstructionFormat{
            operand_type: OperandType::Data(Registers::BP),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xBE    MOV SI, Iw
        (0xBE, InstructionFormat{
            operand_type: OperandType::Data(Registers::SI),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0xBF    MOV DI, Iw
        (0xBF, InstructionFormat{
            operand_type: OperandType::Data(Registers::DI),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0XC2   RET Iw
        (0xC2, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::Procedure,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| src)),
            decode: None,
        }),
        //MARK: 0XC3   RET
        (0xC3, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::Procedure,
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| src)),
            decode: None,
        }),
        //MARK: 0xC6    MOV Eb, Ib
        (0xC6, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, true),
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0XC7    MOV Ew, Iw
        (0xC7, InstructionFormat{
            operand_type: OperandType::ModRM(ModrmRegField::Reg, true),
            instrution_type: InstructionType::General,
            word: true,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0XCD    INT Ib
        (0xCD, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::Interrupt,
            word: false,
            direction: false,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0XCF    IRET Ib
        (0xCF, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::Interrupt,
            word: false,
            direction: true,
            write: true,
            execute: None,
            decode: None,
        }),
        //MARK: 0XE8    CALL Jw
        (0xE8, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::Procedure,
            word: true,
            direction: true,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),
            decode: None,
        }),
        //MARK: 0XE9    JMP Cw near
        (0xE9, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| dst.wrapping_add(src))),
            decode: None,
        }),
        //MARK: 0XEA    JMP far
        (0xEA, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(true),
            word: true,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| src)),
            decode: None,
        }),
        //MARK: 0XEB    JMP short
        (0xEB, InstructionFormat{
            operand_type: OperandType::Data(0),
            instrution_type: InstructionType::ProgramFlow(false),
            word: false,
            direction: false,
            write: true,
            execute: Some(Box::new(|dst, src, flags| ((dst as i16).wrapping_add(src as i8 as i16)) as u32)),
            decode: None,
        }),
        //MARK: 0XF4    HLT
        (0xF4, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: None,
            decode: None,
        }),
        //MARK: 0XF8    CLC
        (0xF8, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {flags.carry = false; 0})),
            decode: None,
        }),
        //MARK: 0XF9    STC
        (0xF9, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {flags.carry = true; 0})),
            decode: None,
        }),
        //MARK: 0XFA    CLI
        (0xFA, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {flags.interrupt = false; 0})),
            decode: None,
        }),
        //MARK: 0XFB    STI
        (0xFB, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {flags.interrupt = true; 0})),
            decode: None,
        }),
        //MARK: 0XFC    CLD
        (0xFC, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {flags.direction = false; 0})),
            decode: None,
        }),
        //MARK: 0XFD    STD
        (0xFD, InstructionFormat{
            operand_type: OperandType::None,
            instrution_type: InstructionType::General,
            word: false,
            direction: false,
            write: false,
            execute: Some(Box::new(|dst, src, flags| {flags.direction = true; 0})),
            decode: None,
        }),
    ]);
    group1::group1_instructions(&mut instructions_map);
    group2::group2_instructions(&mut instructions_map);
    group3::group3_instructions(&mut instructions_map);
    group4::group4_instructions(&mut instructions_map);
    group5::group5_instructions(&mut instructions_map);
    seg_override::segment_override_instructions(&mut instructions_map);
    io::io_instructions(&mut instructions_map);
    instructions_map
}
