use crate::cpu::CPU;
use crate::program_loader::get_program;
use crate::hardware::{Hardware, lightbulb::Lightbulb, keyboard::Keyboard};
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::sync::{Arc, Mutex, mpsc::channel};
/**
 * struct with boolian members representing if a certain hardware is connected
 * to the computer or not.
 */
#[derive(Clone, Copy)]
pub struct HardwareList {
    pub lightbulb: bool,
    pub keyboard: bool,
}

impl HardwareList {
    pub fn new() -> Self {
        Self {
            lightbulb: false,
            keyboard: false,
        }
    }

    pub fn get_hardwares_vector(&self) -> Vec<Box<dyn Hardware>> {
        let mut hardware_vec:Vec<Box<dyn Hardware>> = Vec::new();
        
        if self.lightbulb {
            hardware_vec.push(Box::new(Lightbulb::new()));
        }
        if self.keyboard {
            hardware_vec.push(Box::new(Keyboard::new()));
        }
        hardware_vec
    }
}

pub struct Computer {
    pub cpu: CPU,
    pub connected_hardware: Arc<Mutex<Vec<Box<dyn Hardware>>>>,
    ports: Arc<Mutex<Vec<u8>>>,
}

impl Computer {
    /**
     * Creates a new Computer object and returns it.
     * @param program_path: the path to the program to load.
     */
    pub fn new(program_path: &str, hardware: HardwareList) -> Self {
        use std::ops::DerefMut;
        let ports = Arc::new(Mutex::new(vec![0; 0xFFFF]));
        let hardware = Arc::new(Mutex::new(hardware.get_hardwares_vector()));
        let (cpu_sender, handler_receiver) = channel::<(u16, bool)>();
        let (handler_sender, cpu_receiver) = channel::<u16>();
        let mut computer = Self {
            cpu: CPU::init(&get_program(&program_path),
            ports.clone(),
            (cpu_sender, cpu_receiver),
            None,
            None),
            connected_hardware: hardware.clone(),
            ports: ports.clone()
        };
        // init each hardware on memory
        let mut mhardware = hardware.lock().unwrap();
        for i in 0..mhardware.len() {
            mhardware[i].deref_mut().init(&mut computer.cpu.memory_unit.memory);
        }
        drop(mhardware);

        thread::spawn(move || {
            loop {
                match handler_receiver.try_recv() {
                    Err(err) => match err {
                        TryRecvError::Empty => (),
                        TryRecvError::Disconnected => break,
                    },
                    Ok(port) =>  {
                        let mut hardwares = hardware.lock().unwrap();
                        for i in 0..hardwares.len() {
                            if hardwares[i].is_port_related(&port.0) {
                                hardwares[i].run_hardware(ports.lock().unwrap().deref_mut());
                            }
                        }
                        handler_sender.send(10u16);
                    },
                }
            }
        });
        computer
    }

    pub fn run(&mut self) {
        // let cpu = Fragile::new(&mut self.cpu);
        let hardware = self.connected_hardware.clone();
        thread::spawn(move || {
            assert!(true);
        });
    }
}