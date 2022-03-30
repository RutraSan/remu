use crate::computer::{Computer, HardwareList};
use crate::hardware::{Hardware, lightbulb::Lightbulb, keyboard::Keyboard};

use eframe::egui::Widget;
use eframe::{epi::App, egui, 
    egui::{CentralPanel, Window, Vec2, TopBottomPanel, CtxRef, Ui}};
use queues::IsQueue;

use std::{sync::{Arc, Mutex}, path::PathBuf};
use std::thread;

enum State {
    ProgramLoader,
    Running,
    Debugger,
}

#[derive(PartialEq, Clone, Copy)]
enum RegInfo {
    NONE,
    AX,
    BX,
    CX,
    DX,
    SP,
    BP,
    SI,
    DI,
    IP,
    CS,
    DS,
    SS,
    ES,
}

pub struct Emulator{
    pub computer: Option<Computer>,
    program_path: PathBuf,
    open_err: bool,
    state: State,

    halted: bool,
    connected_hardware: HardwareList,

    // Debugging options
    running: bool,
    follow_add: bool,
    reg_info: RegInfo,
    add: u16,
    seg: u16,
    add_txt: String,
    seg_txt: String,
    inst_len: u16,
}

impl Emulator{
    pub fn new() -> Self {
        Self {
            computer: None,
            program_path: PathBuf::new(),
            open_err: false,
            state: State::ProgramLoader,
            halted: false,
            connected_hardware: HardwareList::new(),
            running: false,
            follow_add: false,
            reg_info: RegInfo::NONE,
            add: 0,
            seg: 0,
            add_txt: String::with_capacity(4),
            seg_txt: String::with_capacity(4),
            inst_len: 0,
        }
    }

    /**
     * Program Loader Menu.
     * This is the menu where the user can choose the program to run and as well
     * initialize the Computer with hardware and more.
     */
    fn program_loader_menu(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            // get program path from explorer.
            ui.horizontal(|ui| {
                if ui.button("open program").clicked() {
                    #[cfg(windows)]
                    match wfd::open_dialog(Default::default()) {
                        Ok(dialog_result) => {
                            self.program_path = dialog_result.selected_file_path;
                            self.open_err = false;
                        },
                        Err(__) => (),
                    }
                }
                ui.label( self.program_path.to_str().unwrap());
            });
            if self.open_err {
                ui.label("file doesn't exist");
            }
            // if ui.button("run program").clicked() {
            //     self.computer = Some(Computer::new(
            //         self.program_path.to_str().unwrap()
            //     ));
            //     self.state = State::Running;
            // }
            if ui.button("run with debugger").clicked() {
                if self.program_path.exists() {
                    self.computer = Some(Computer::new(
                        self.program_path.to_str().unwrap(),
                        self.connected_hardware,
                    ));
                    let cpu = &mut self.computer.as_mut().unwrap().cpu;
                    self.add = cpu.memory_unit.ip;
                    self.seg = cpu.memory_unit.memory.code_segment;
                    self.seg_txt = format!("{:04X}", self.seg);
                    self.add = cpu.memory_unit.ip;
                    self.add_txt = format!("{:04X}", self.add);
                    self.state = State::Debugger;
                }
                else {
                    self.open_err = true;
                }
            }
            ui.checkbox(&mut self.connected_hardware.lightbulb, "Lightbulb");
            // ui.checkbox(&mut self.connected_hardware.keyboard, "Keyboard");
        });
    }

    /**
     * Menu for a program running without a debugger
     */
    fn running(&mut self, ui: &mut Ui) {
        ui.label("program running");
        let computer = &mut self.computer.as_mut().unwrap();
        computer.run();
    }

    /**
     * Menu for a program running with a Debugger
     */
    fn debugger(&mut self, ui: &mut Ui, ctx: &CtxRef) {
        let mut run = self.running;
        let mut follow_add = self.follow_add;
        let mut reg_info = self.reg_info;
        let mut seg = self.seg_txt.clone();
        let mut add = self.add_txt.clone();
        let mut update_seg = false;
        let mut update_add = false;
        let enabled = !self.halted;
        let mut inst_len = self.inst_len;

        // Menu
        ui.horizontal(|ui| {
            use egui::Button;
            if ui.add_enabled(enabled, Button::new("single step")).clicked() {
                run = true;
            }
            if ui.add_enabled(enabled, Button::new("run")).clicked() {
                self.running = true;
            }
            if ui.add_enabled(enabled, Button::new("stop")). clicked() {
                self.running = false;
            }
            if !enabled {
                ui.label("program halted");
                self.running = false;
                run = false;
            }
        });
        // get ref to CPU
        let cpu = &mut self.computer.as_mut().unwrap().cpu;
        // run the cpu virtualy
        if run {
            let res = cpu.run_next_instruction();
            if res == Ok(0xF4 as u8) {
                self.halted = true;
            }
            // get length of the instruction (used later for color it in memory view)
            inst_len = 0;
            let ip = cpu.memory_unit.ip;
            cpu.load_instruction();
            inst_len = cpu.memory_unit.ip - ip;
            cpu.memory_unit.inst_bus = queues::Buffer::new(6);
            cpu.memory_unit.ip = ip;
        }
        let cpu = &self.computer.as_ref().unwrap().cpu;
        ui.separator();
        ui.horizontal(|ui| {
            //# REGISTORS DATA
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(10.0, 5.0);
                ui.set_max_width(100.0);
                if ui.button(format!("AX: {:04X}", cpu.memory_unit.ax)).clicked() {
                    reg_info = RegInfo::AX;
                }
                if ui.button(format!("BX: {:04X}", cpu.memory_unit.bx)).clicked() {
                    reg_info = RegInfo::BX;
                }
                if ui.button(format!("CX: {:04X}", cpu.memory_unit.cx)).clicked() {
                    reg_info = RegInfo::CX;
                }
                if ui.button(format!("DX: {:04X}", cpu.memory_unit.dx)).clicked() {
                    reg_info = RegInfo::DX;
                }
                if ui.button(format!("SP: {:04X}", cpu.memory_unit.sp)).clicked() {
                    reg_info = RegInfo::SP;
                }
                if ui.button(format!("BP: {:04X}", cpu.memory_unit.bp)).clicked() {
                    reg_info = RegInfo::BP;
                }
                if ui.button(format!("SI: {:04X}", cpu.memory_unit.si)).clicked() {
                    reg_info = RegInfo::SI;
                }
                if ui.button(format!("DI: {:04X}", cpu.memory_unit.di)).clicked() {
                    reg_info = RegInfo::DI;
                }
                if ui.button(format!("IP: {:04X}", cpu.memory_unit.ip)).clicked() {
                    reg_info = RegInfo::IP;
                }
                ui.separator();
                if ui.button(format!("CS: {:04X}", cpu.memory_unit.memory.code_segment)).clicked() {
                    reg_info = RegInfo::CS;
                }
                if ui.button(format!("DS: {:04X}", cpu.memory_unit.memory.data_segment)).clicked() {
                    reg_info = RegInfo::DS;
                }
                if ui.button(format!("SS: {:04X}", cpu.memory_unit.memory.stack_segment)).clicked() {
                    reg_info = RegInfo::SS;
                }
                if ui.button(format!("ES: {:04X}", cpu.memory_unit.memory.extra_segment)).clicked() {
                    reg_info = RegInfo::ES;
                }
                ui.button(format!("FR: {:04X}", cpu.memory_unit.flags.as_word()));
                ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
            });
            ui.add(egui::Separator::default().vertical());

            //# SELECTED REGISTOR
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui: &mut Ui| {
                // ui.set_max_width(500.0);
                ui.set_min_size(Vec2::new(270., 500.));
                ui.set_max_size(Vec2::new(270., 500.));
                let reg_val = match reg_info {
                    RegInfo::AX => ("AX", cpu.memory_unit.ax),
                    RegInfo::BX => ("BX", cpu.memory_unit.bx),
                    RegInfo::CX => ("CX", cpu.memory_unit.cx),
                    RegInfo::DX => ("DX", cpu.memory_unit.dx),
                    RegInfo::SP => ("SP", cpu.memory_unit.sp),
                    RegInfo::BP => ("BP", cpu.memory_unit.bp),
                    RegInfo::SI => ("SI", cpu.memory_unit.si),
                    RegInfo::DI => ("DI", cpu.memory_unit.di),
                    RegInfo::IP => ("IP", cpu.memory_unit.ip),
                    RegInfo::CS => ("CS", cpu.memory_unit.memory.code_segment),
                    RegInfo::DS => ("DS", cpu.memory_unit.memory.data_segment),
                    RegInfo::SS => ("SS", cpu.memory_unit.memory.stack_segment),
                    RegInfo::ES => ("ES", cpu.memory_unit.memory.extra_segment),
                    RegInfo::NONE => ("Select a register to get detailed information", 0),
                };
                // ui.label(reg_val.0);
                ui.label(egui::RichText::new(reg_val.0).underline());
                ui.label(format!("Bin: {:08b} {:08b}", reg_val.1 >> 8, reg_val.1 & 0x00FF));
                ui.label(format!("Hex: {:04X}", reg_val.1));
                ui.label(format!("Sig: {:05}", reg_val.1 as i16));
                ui.label(format!("Uns: {:05}", reg_val.1));
            });
            ui.add(egui::Separator::default().vertical());

            //# PRINT MEMORY
            ui.vertical(|ui| {
                // address to print
                let mut print_seg = self.seg;
                let mut print_add = self.add;
                // check if we were supposed to follow
                if follow_add {
                    print_seg = cpu.memory_unit.memory.code_segment;
                    print_add = cpu.memory_unit.ip;
                }

                // count how many bytes of the instruction were colored
                let mut count_inst = 0;
                // segment
                ui.horizontal(|ui| {
                    let seg_editor = ui.add(egui::TextEdit::singleline(&mut seg)
                        .desired_width(45.));
                    ui.add_space(10.);
                    let add_editor = ui.add(egui::TextEdit::singleline(&mut add)
                        .desired_width(45.));
                    ui.add_space(10.);
                    if ui.button("follow").clicked() {
                        follow_add =  !follow_add;
                    }

                    // check if seg_editor lsot focus, then change the segment value
                    if seg_editor.lost_focus() {
                       update_seg = check_number_is_hex(&mut seg);
                    }
                    if add_editor.lost_focus() {
                        update_add = check_number_is_hex(&mut add);
                    }
                    if follow_add {
                        update_seg = true;
                        seg = format!("{:04X}", print_seg);
                        update_add = true;
                        add = format!("{:04X}", print_add);
                    }
                });
            
                // print memory at seg:add
                for i in 0..16 {
                    let val = cpu.memory_unit.memory[(print_seg, print_add)];
                    let arr = [val];
                    let asci = match  std::str::from_utf8(&arr) {
                        Ok(v) => v,
                        Err(_) => "",
                    };
                    // print the memory
                    let mut mem = egui::RichText::new(format!("{:04X} : {:04X}\t{:02X}  {2:03}  {3}",print_seg, print_add, val, asci));
                    
                    // color the current instruction
                    if (print_seg == cpu.memory_unit.memory.code_segment &&
                       print_add == cpu.memory_unit.ip )||
                       (count_inst > 0 && count_inst < inst_len) {
                        mem = mem.background_color(egui::Rgba::from_rgb(255., 60., 0.))
                            .color(egui::Rgba::from_rgb(0., 0., 0.))
                            .strong();
                        count_inst += 1;
                    }
                    if ui.add(egui::Label::new(mem).sense(egui::Sense::hover())).hovered() {
                        let s = ctx.input().scroll_delta;
                        let mut add_num = i32::from_str_radix(&add, 16).unwrap();
                        add_num -= ((s.y / 50.) as i32);
                        if add_num < 0 || add_num != add_num & 0xffff{
                            seg = format!("{:04X}", i32::from_str_radix(&seg, 16).unwrap() + (-s.y / s.y.abs()) as i32);
                            update_seg = true;
                        }
                        // ui.label(format!("{}", (s.y / 50.)));
                        add = format!("{:04X}", add_num as u16);
                        update_add = true;
                        follow_add = false;
                        
                    }
                    if print_add == 0xffff {
                        print_seg += 1;
                        print_add = 0;
                    }
                    else {print_add += 1;}
                }
                ui.label(format!("inst_len: {}", inst_len));
            });
        });

        //# UPDATE DEBBUGING INFORMATION
        self.reg_info = reg_info;
        self.inst_len = inst_len;
        self.follow_add = follow_add;
        
        // update segment and address
        self.seg_txt = seg.clone();
        self.add_txt = add.clone();
        if update_seg {
            let val = u16::from_str_radix(&seg, 16).unwrap();
            self.seg = val;
            self.seg_txt = format!("{:04X}", val);
        }
        if update_add {
            let val = u16::from_str_radix(&add, 16).unwrap();
            self.add = val;
            self.add_txt = format!("{:04X}", val);
        }
    }

    fn show_hardware(&mut self, ctx: &CtxRef) {
        match self.computer.as_mut() {
            None => (),
            Some(computer) => {
                let hardware = computer.connected_hardware.lock().unwrap();
                for i in 0..hardware.len() {
                    Window::new(hardware[i].name()).show(ctx, |ui| {
                        hardware[i].ui(ui);
                    });
                }
            }
        }
    }

    /**
     * Function for configuring the fonts of the program.
     */
    fn config_fonts(&self, ctx: &egui::CtxRef) {
        let mut fonts = egui::FontDefinitions::default();

        // set RobotoMono font
        let mut roboto_mono = egui::FontData::from_static(include_bytes!("../fonts/RobotoMono.ttf"));
        roboto_mono.index = 4;
        fonts.font_data.insert("RobotoMono".to_owned(),
            roboto_mono);
        
        fonts.fonts_for_family.get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "RobotoMono".to_owned());
        
        // set sizes for TextStyles
        // Button
        fonts.family_and_size.insert(
            egui::TextStyle::Button,
            (egui::FontFamily::Monospace, 24.0)
        );
        // Body
        fonts.family_and_size.insert(
            egui::TextStyle::Body,
            (egui::FontFamily::Monospace, 24.0)
        );
        // Heading
        fonts.family_and_size.insert(
            egui::TextStyle::Heading,
            (egui::FontFamily::Monospace, 32.0)
        );
        
        ctx.set_fonts(fonts);
    }
}

impl App for Emulator {
    fn setup(&mut self, 
        ctx: &egui::CtxRef, 
        _frame: &eframe::epi::Frame, 
        _storage: Option<&dyn eframe::epi::Storage>
    ) {
        self.config_fonts(ctx);
    }
    fn update(&mut self, ctx: &CtxRef, frame: &eframe::epi::Frame) {
        TopBottomPanel::top("top").show(ctx, |ui| {
            ui.vertical( |ui| {
                ui.heading("Emulator");
                ui.add_space(10.0);
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            match &self.state {
                State::ProgramLoader => self.program_loader_menu(ui),
                State::Running => self.running(ui),
                State::Debugger => self.debugger(ui, ctx),
            }
        });
        self.show_hardware(ctx);
    }

    fn name(&self) -> &str {
        "Emulator"
    }
}

fn check_number_is_hex(num: &mut String) -> bool {
    num.truncate(4);
    if num.len() == 0 {
        *num = "0".to_string();
    }
    match u16::from_str_radix(num, 16) {
        Ok(val) => true,
        Err(_) => false,
    }
}