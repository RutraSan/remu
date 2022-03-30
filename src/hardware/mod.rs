use std::ops::Range;
use std::sync::{Arc, Mutex};
use std::thread;

use eframe::egui;
use eframe::egui::Ui;

use crate::cpu::memory_unit::memory_segments::MemorySegments;
use crate::program_loader::get_program;

pub mod lightbulb;
pub mod keyboard;

pub trait Hardware: Send {

    /**
     * Inits the hardware and lets him initialize it's interrupt
     * code and program.
     * @param memory: a reference to the MemorySegments struct
     *      of the cpu, letting the hardware to write it's program.
     */
    fn init(&self, memory: &mut MemorySegments) -> ();

    /**
     * Runs the hardware. That is, the way to simulate a 'hardware' in action.
     * The function must be called in to a new thread with the ports parameter given.
     * @param ports: Arc of Mutex to a vec[u8] representing the ports.
     * @return: doesn't return anything, but if called in a seperate thread, the 
     *      thread returns a Result.
     */
    fn run_hardware(&mut self, ports: &mut Vec<u8>) -> ();

    fn is_port_related(&self, port: &u16) -> bool;

    /**
     * Adds a ui content on to a new `Window`.
     * If not implamented, nothing happens.
     * @param ui: refernece to the Ui
     */
    fn ui(&self, ui: &mut Ui) {
        ()
    }

    /**
     * Returns the name of the hardware.
     */
    fn name(&self) -> &str{
        "Hardware"
    }

}