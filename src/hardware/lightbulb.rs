use super::*;

pub struct Lightbulb {
    ports_range: Range<usize>,
    power: bool,
    color: f32,
}

impl Lightbulb {
    // Creates a new instance of Lightbulb
    pub fn new() -> Self {
        Self {
            ports_range: 0xbb..0xbc,
            power: false,
            color: 0.0,
        }
    }
}

impl Hardware for Lightbulb {
    fn init(&self, memory: &mut MemorySegments) -> () {
        let interrupt_vector = vec![0x00, 0x33, 0x00, 0x00];
        memory.write(&interrupt_vector, (0, 0xCC));
        memory.write(&get_program("src/hardware/lightbulb.asm"), (0x3300, 0));
    }  

    fn run_hardware(&mut self, ports: &mut Vec<u8>) -> () {
        if ports[0xbb] == 0 {
            ports[0xbc] = 0;
            self.power = false;
        }
        else {
            if ports[0xbc] == 0 {
                ports[0xbc] = 255;
            }
            self.power = true;
        }
        self.color = ports[0xbc] as f32;
    }

    fn is_port_related(&self, port: &u16) -> bool {
        let p = *port as usize;
        self.ports_range.contains(&p)
    }

    fn ui(&self, ui: &mut Ui) {
        if self.power {
            ui.add(egui::Label::new(egui::RichText::new("    ")
                .background_color(egui::Rgba::from_rgb(self.color, self.color, self.color))));
        }
        else {
            ui.add(egui::Label::new(egui::RichText::new("    ")
                .background_color(egui::Rgba::BLACK)));
        }
    }

    fn name(&self) -> &str {
        "Lightbulb"
    }
}