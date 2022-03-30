use super::*;
use libc::getchar;

pub struct Keyboard {
    ports_range: Range<usize>,
    buffer: i32,
}

impl Keyboard {
    // Creates a new instance of Lightbulb
    pub fn new() -> Self {
        Self {
            ports_range: 0x1000..0x1002,
            buffer: 0,
        }
    }
}

impl Hardware for Keyboard {
    fn init(&self, memory: &mut MemorySegments) -> () {
        let interrupt_vector = vec![0x00, 0x16, 0x00, 0x00];
        memory.write(&interrupt_vector, (0, 0x58));
        memory.write(&get_program("src/hardware/keyboard.asm"), (0x1600, 0));
    }  

    fn run_hardware(&mut self, ports: &mut Vec<u8>) -> () {
        // get keystroke
        if ports[0x1000] == 0 {
            unsafe {
                self.buffer = getchar();
            }
        }
    }

    fn is_port_related(&self, port: &u16) -> bool {
        let p = *port as usize;
        self.ports_range.contains(&p)
    }

    fn ui(&self, ui: &mut Ui) {
        ui.label("THIS IS DEBBUGING WINDOWS");
        ui.label(format!("{}", self.buffer));
    }

    fn name(&self) -> &str {
        "Keyboard"
    }
}