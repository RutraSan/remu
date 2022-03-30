extern crate queues;
extern crate console;
extern crate keyboard_query;
extern crate device_query;

use crate::emulator::Emulator;
// use keyboard_query::{DeviceQuery, DeviceState};
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

mod cpu;
mod computer;
mod program_loader;
mod emulator;
pub mod hardware;

#[allow(dead_code)]
mod assembler;
mod debugger;

fn main() {
    // let device_state = DeviceState::new();  
    // loop {
    //     let keys: Vec<Keycode> = device_state.query_keymap();
    //     if keys.len() > 0 {
    //         println!("{:?}", keys);
    //     }
    // }


    let mut app = Emulator::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}