#![allow(dead_code)]

use std::{
    ops::{Index, IndexMut},
    ptr::copy,
};

use crate::cpu::helperModules::Registers;

const SEGMENT_SIZE: u32 = 0xFFFF;
const MEMORY_SIZE: usize = 0x100000;
/**
 * This enum helps access the needed segment of the memory.
 */
#[derive(Debug)]
pub enum Segment {
    CS,
    SS,
    ES,
    DS,
    IVT,
}
impl Segment {
    pub fn get_segment(&self, mem: &MemorySegments) -> u16{
        match &self {
            Self::CS => mem.code_segment,
            Self::SS => mem.stack_segment,
            Self::ES => mem.extra_segment,
            Self::DS => mem.data_segment,
            Self::IVT => 0,
        }
    }

    pub fn from(string: &str) -> Result<Self, ()>{
        let string = string.to_lowercase();
        match string {
            _ if string == "ds" => Ok(Self::DS),
            _ if string == "cs" => Ok(Self::CS),
            _ if string == "ss" => Ok(Self::SS),
            _ if string == "es" => Ok(Self::ES),
            _ => Err(())
        }
    }

    pub fn from_code(segment: u8) -> Result<Self, ()>{
        match segment {
            Registers::DS => Ok(Self::DS),
            Registers::CS => Ok(Self::CS),
            Registers::SS => Ok(Self::SS),
            Registers::ES => Ok(Self::ES),
            _ => Err(())
        }
    }
}

/**
 * This struct holds a pointer to memory location and needed length to read/write.
 */
pub struct MemoryPointer {
    len: u8,
    ptr: *mut u8,
}
impl MemoryPointer {
    // Constructor
    pub fn new(len: u8, ptr: *mut u8) -> Self {
        Self {
            len: len,
            ptr: ptr,
        }
    }

    //Setter
    pub fn set(&mut self, val:u32) {
        let mut val = val;
        for i in 0..self.len as usize {
            unsafe { *self.ptr.add(i) = val as u8;}
            val >>= 8;
        }
    }

    // Getter
    pub fn get(&self) -> u32 {
        let mut val: u32 = 0;
        for i in (0..self.len).rev()  {
            val <<= 8;
            unsafe {val += *self.ptr.add(i as usize) as u32;}
        }
        val
    }
}

pub struct MemorySegments {
    /**
     * This is a private field which represents the memory of the cpu,
     * but the `[]` operator is overloaded and can be accessed with it.
     */
    memory: Vec<u8>,

    // Segment registers
    pub code_segment: u16,
    pub data_segment: u16,
    pub stack_segment: u16,
    pub extra_segment: u16,
}
impl MemorySegments {
    //Con
    pub fn new() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE],
            code_segment: 0x0000,
            data_segment: 0x0000,
            stack_segment: 0x0000,
            extra_segment: 0x0000,
        }
    }

    /**
     * This function gets a vector reference conatining data to write into
     * segment:address pair.
     * @param data: a vector reference conatining a data to write into the memory.
     * @param add: a pair of segment:address. basically the location of memory
     *             to write into.
     */
    pub fn write(&mut self, data: &Vec<u8>, add: (u16, u16)) {
        let address = get_physical_address(add) as usize;
        unsafe {
            copy(data.as_ptr(), self.memory.as_mut_ptr().add(address), data.len());
        }
    }

    /**
     * This function returns a pointer to a location in the memory.
     * @param seg: the segment from which to take the pointer.
     * @param add: the address to take.
     */
    pub fn get_memory_pointer(&mut self, seg: &Segment, add: u16) -> *mut u8 {
        let address: usize = get_physical_address((seg.get_segment(self), add)) as usize;
        let p:*mut u8 = &mut self.memory[address];
        return p
    }
}

//# `[]` operator overloading for easier memory access
impl Index<(u16, u16)> for MemorySegments {
    type Output = u8;
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        &self.memory[get_physical_address(index) as usize]
    }
}

impl IndexMut<(u16, u16)> for MemorySegments {
    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        &mut self.memory[get_physical_address(index) as usize]
    }
}

/**
 * This function helps to calculate the physical address of the memory with the segment and 
 * address given as parameters.
 * @param addresses: tupple which contains the segment as first item and the address as second.
 * @ret: the physical address as a u32.
 */
fn get_physical_address(addresses: (u16, u16)) -> u32 {
    let (segment, address) = addresses;
    segment as u32 * 10 + address as u32
}