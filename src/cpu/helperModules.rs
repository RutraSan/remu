#![allow(non_snake_case)]
#![allow(dead_code)]

// indexing general usage regs
pub const HIGH_BYTE: usize = 0;
pub const LOW_BYTE: usize = 1;
pub const VARIABLE: u8 = 0b110;

// Registers
pub mod Registers {
    // word registers
    pub const AX: u8 = 0;
    pub const CX: u8 = 1;
    pub const DX: u8 = 2;
    pub const BX: u8 = 3;
    pub const SP: u8 = 4;
    pub const BP: u8 = 5;
    pub const SI: u8 = 6;
    pub const DI: u8 = 7;

    // byte registers
    pub const AL: u8 = 0;
    pub const CL: u8 = 1;
    pub const DL: u8 = 2;
    pub const BL: u8 = 3;
    pub const AH: u8 = 4;
    pub const CH: u8 = 5;
    pub const DH: u8 = 6;
    pub const BH: u8 = 7;

    // SERG 
    pub const ES: u8 = 0;
    pub const CS: u8 = 1;
    pub const SS: u8 = 2;
    pub const DS: u8 = 3;

    // segment regs for operand type Registers 
    pub const RES: u8 = 10;
    pub const RCS: u8 = 11;
    pub const RSS: u8 = 12;
    pub const RDS: u8 = 13;
}

/**
 * This struct represents the flags register.
 */
#[derive(Default, Clone, Copy)]
pub struct Flags {
    pub carry: bool,
    pub res1: bool, // used for telling wether the DIV is word or byte
    pub parity: bool,
    res2: bool,
    pub auxilarity: bool,
    res3: bool,
    pub zero: bool,
    pub sign: bool,
    pub trap: bool,
    pub interrupt: bool,
    pub direction: bool,
    pub overflow: bool,
    pub io_privilege_low: bool,
    pub io_privilege_high: bool,
    pub nested_task: bool,
    res4: bool,
}

impl Flags {

    /**
     * The function returns the flag as a word.
     * @return: a word value of the flag.
     */
    pub fn as_word(&self) -> u16 {
        let mut word:u16 = 0;
        if self.carry {word ^= 0b0000000000000001}
        if self.res1 {word ^= 0b0000000000000010}
        if self.parity {word ^= 0b0000000000000100}
        if self.res2 {word ^= 0b0000000000001000}
        if self.auxilarity {word ^= 0b0000000000010000}
        if self.res3 {word ^= 0b0000000000100000}
        if self.zero {word ^= 0b0000000001000000}
        if self.sign {word ^= 0b0000000010000000}
        if self.trap {word ^= 0b0000000100000000}
        if self.interrupt {word ^= 0b0000001000000000}
        if self.direction {word ^= 0b0000010000000000}
        if self.overflow {word ^= 0b0000100000000000}
        if self.io_privilege_low {word ^= 0b0001000000000000}
        if self.io_privilege_high {word ^= 0b0010000000000000}
        if self.nested_task {word ^= 0b0100000000000000}
        if self.res4 {word ^= 0b1000000000000000}
        word
    }

    /**
     * The function sets flags value
     * @param val: a word value
     */
    pub fn set(&mut self, val: u16) {
        self.carry =  if val & 1 == 1 {true} else {false};
        self.res1 =  if val >> 1 & 1 == 1 {true} else {false};
        self.parity =  if val >> 2 & 1 == 1 {true} else {false};
        self.res2 =  if val >> 3 & 1 == 1 {true} else {false};
        self.auxilarity =  if val >> 4 & 1 == 1 {true} else {false};
        self.res3 =  if val >> 5 & 1 == 1 {true} else {false};
        self.zero =  if val >> 6 & 1 == 1 {true} else {false};
        self.sign =  if val >> 7 & 1 == 1 {true} else {false};
        self.trap =  if val >> 8 & 1 == 1 {true} else {false};
        self.interrupt =  if val >> 9 & 1 == 1 {true} else {false};
        self.direction =  if val >> 10 & 1 == 1 {true} else {false};
        self.overflow =  if val >> 11 & 1 == 1 {true} else {false};
        self.io_privilege_low =  if val >> 12 & 1 == 1 {true} else {false};
        self.io_privilege_high =  if val >> 13 & 1 == 1 {true} else {false};
        self.nested_task =  if val >> 14 & 1 == 1 {true} else {false};
        self.res4 =  if val >> 15 & 1 == 1 {true} else {false};

    }
}

pub enum OperandType {
    //ModrmRegField: ecrypts the reg field. bool: data.
    ModRM(ModrmRegField, bool),
    //u8: register
    Data(u8),
    // u8: register
    Address(u8),
    // u8: dst register. Option<u8>: optional src register
    Registers(u8, Option<u8>),
    None,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ModrmRegField {
    Reg,
    Sreg,
    Group(u8),
}

/**
 * This function converts a given string to number from any base.
 * @param string: the string to parse
 * @return: the resulted number.
 */
pub fn string_to_number(string: &str) -> Result<u32, String> {
    // decimal
    match string.parse::<u32>() {
        Ok(number) => return Ok(number), 
        Err(_) => ()
    }
    let numstring = &string[0..string.len() -1];
    // hex base
    if string.ends_with("h") {
        match u32::from_str_radix(numstring, 16) {
            Ok(number) => return Ok(number), 
            Err(_) => return Err("bad hex number format".to_string())
        }
    }
    // octal base
    else if string.ends_with("o") {
        match u32::from_str_radix(numstring, 2) {
            Ok(number) => return Ok(number), 
            Err(_) => return Err("bad octal number format".to_string())
        }
    }
    // binary base
    else if string.ends_with("b") {
        match u32::from_str_radix(numstring, 2) {
            Ok(number) => return Ok(number), 
            Err(_) => return Err("bad binary number format".to_string())
        }
    }
    return Err("bad number format".to_string())
}