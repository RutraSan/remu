#![allow(unused_imports)]
use crate::cpu::*;
use crate::cpu::helperModules::ModrmRegField;
use crate::cpu::memory_unit::Operand;
use crate::cpu::memory_unit::memory_segments::*;

mod cpu;
mod hardware;
mod program_loader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mempointer() {
        let mut mem = MemorySegments::new();
        let seg = Segment::DS;
        let p = mem.get_memory_pointer(&seg, 0);
        let mut memp = MemoryPointer::new(2, p);
        memp.set(0xABCD);
        //assert!(mem.get_data(&seg, 0, true) == 0xABCD);
    }

    #[test]
    fn operand_test() {
        let mut mem = memory_unit::MemoryUnit::new();
        let mut reg = mem.get_reg(0, ModrmRegField::Reg, true);
        let mut reg2 = mem.get_reg(1, ModrmRegField::Reg, true);        
        reg.set(20);
        reg2.set(5);
        assert!(mem.ax == 20);
        assert!(mem.cx == 5);
    }
}