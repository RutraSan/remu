#![allow(unused_must_use)]
use std::io::Write;
use console::Term;
use queues::*;

use crate::cpu::{memory_unit::memory_segments::Segment, helperModules};

use super::cpu::CPU;

/**
 * The Debugger is the struct which runs the program on his own, which allows
 * for a control over the program.
 */
pub struct Debugger {
    // if true, the Debugger will print each running instruction.
    print_instruction: bool,
    //if true, this means that the program is halted.
    code_finished: bool,
}

impl Debugger {

    /**
     * The new() function which returns a new object of `Self`
     * @return: new instance of `Self`
     */
    pub fn new() -> Self {
        Self{
            print_instruction: false,
            code_finished: false,
        }
    }

    /**
     * This is the main function for running the program in debug mode.
     * @param cpu: a mutable refernce to the cpu.
     */
    pub fn run_emulator(&mut self, cpu: &mut CPU) {
        // this is used instead of a default print functions for a better customization.
        let mut term = Term::stdout();

        term.set_title("Debug Mode");
        term.clear_screen();
        
        // Default print
        term.write_line(">>> Running in Debug mode");
        term.write_line(">>> Try 'help'");
        

        // Running the console
        loop {
            term.write(b">>> ");

            // get the input and split to parameters
            let input = term.read_line().expect("error");
            let split = input.split(' ').collect::<Vec<&str>>();
            let command = split[0];
            
            // run requested commmand
            let res: Result<String, String> = match command {
                _ if command == "help" => Ok("not implemented yet".to_string()),
                _ if command == "run" => self.run(cpu, split),
                _ if command == "sop" => self.showop(split),
                _ if command == "echo" => self.echo(cpu, split),
                _ if command == "restart" => unimplemented!(),
                _ if command == "exit" => break,
                _ if command == "" => Ok(format!("")),
                _ => Ok("Unkown Command".to_string()),
            };

            // print the result of the runned command
            match res {
                Result::Ok(msg) => term.write_line(&msg).unwrap(),
                Result::Err(msg) => term.write_line(&msg).unwrap(),
            }
        }
    
    }

    /**
     *# run [num_of_lines] command.
     * @param cpu: a mutable reference to the cpu.
     * @param parameters: Vec containing the parameters of the command.
     */
    fn run(&mut self, cpu: &mut CPU, parameters: Vec<&str>) -> Result<String, String> {
        // check that command is valid
        if parameters.len() != 2 {
            return Err("bad arguments".to_string())
        }

        // variables
        let all = if parameters[1] == "all" {true} else {false}; 
        let mut count: i32 = match parameters[1].parse() {
            Result::Err(_) => {
                if !all {return Err("wrong argument [count]".to_string())}
                else {0}
            },
            Result::Ok(val) => val,
        };
        let c = count;

        // main loop running the program
        loop {
            // stop running
            if !all && count <= 0 {
                break
            }
            // print instruction
            if self.print_instruction {
                let ip = cpu.memory_unit.ip;
                cpu.load_instruction();
                while cpu.memory_unit.inst_bus.size() > 0 {
                    print!{"0x{:02X?}\t", cpu.memory_unit.inst_bus.remove().unwrap()}
                }
                println!();
                cpu.memory_unit.ip = ip;
            }

            let res = cpu.run_next_instruction();
            // Halted
            if res == Ok(0xF4 as u8)  {
                self.code_finished = true;
                return Ok(format!("program halted"))
            }
            if res.is_err() {
                return Err(res.unwrap_err())
            }

            count -= 1;
        }
        return Ok(format!("{} lines runned succesfuly", c));
    }

    /**
     *# echo reg
     * Print value of a registers.
     *# echo [seg]:[add] -l [len]
     * prints memory location.
     * @param cpu: a mutable reference to the cpu.
     * @param parameters: Vec containing the parameters of the command.
     */
    pub fn echo(&mut self, cpu: &mut CPU, parameters: Vec<&str>) -> Result<String, String> {
        if parameters.len() < 2 {
            return Err("bad arguments".to_string())
        }
        let param1 = parameters[1].to_lowercase();
        let val = match param1 {
            // registers
            _ if param1 == "ax" => cpu.memory_unit.ax,
            _ if param1 == "bx" => cpu.memory_unit.bx,
            _ if param1 == "cx" => cpu.memory_unit.cx,
            _ if param1 == "dx" => cpu.memory_unit.dx,
            _ if param1 == "ip" => cpu.memory_unit.ip,
            _ if param1 == "sp" => cpu.memory_unit.sp,
            _ if param1 == "flags" => cpu.memory_unit.flags.as_word(),
            _ if param1 == "freg" => cpu.memory_unit.flags.as_word(),
            _ if param1 == "es" => cpu.memory_unit.memory.extra_segment,
            _ if param1 == "cs" => cpu.memory_unit.memory.code_segment,

            // print the contents of the stack
            _ if param1 == "stack" => {
                let mut i: u16 = 0xFFFE;
                let mut memory_pointer = cpu.memory_unit.memory.get_memory_pointer(&Segment::SS, i);
                while i >= cpu.memory_unit.sp{
                    let data = unsafe {*memory_pointer as u16 + ((*memory_pointer.add(1) as u16) << 8)};
                    println!("0x{:04X?}\t0x{:04X?}",i, data);
                    memory_pointer = unsafe{memory_pointer.sub(2)};
                    i -= 2;
                }
                return Ok(format!(""))
            }

            // memory
            _ if param1.contains(":") => {
                // check that arguments are correct
                let args = parameters[1].split(":").collect::<Vec<&str>>();
                if args.len() != 2 {
                    return Err(format!("bad argument '{}'", parameters[1]))
                }
                // check segemnt
                let segment = args[0].to_lowercase();
                let segment = match Segment::from(&segment) {
                    Result::Ok(val) => val,
                    Result::Err(_) => return Err(format!("bad `segmet` argument '{}'", segment))
                };

                // check address
                let address = match helperModules::string_to_number(args[1]) {
                    Result::Ok(val) => val as u16,
                    Result::Err(_) => {
                        // accept ip as address
                        if args[1].eq("ip") {
                            cpu.memory_unit.ip
                        }
                        else{
                            return Err(format!("bad `address` argument '{}'", args[1]))
                        }
                    }
                };
                // check for the length
                let mut len = 1;
                if parameters.len() == 4 && parameters[2] == "-l" {
                    let len_res = parameters[3].parse::<u16>();
                    if len_res.is_err() {
                        return Err(format!("bad `length` argument '{}'", args[3]))
                    }
                    len = len_res.unwrap();
                    if len > 0x100 {
                        return Err(format!("length can't be more then {}", 0x100))
                    }
                }
                // print memory column
                print!("\t");
                let columns = if len > 16 {16} else {len};
                for i in 0..columns {
                    print!("{:02X?}\t", i);
                }
                // print memory
                for i in 0..len {
                    // print memory row
                    if i % 16 == 0 {
                        print!("\n{:02X?}\t", i);
                    }
                    let data = unsafe{*cpu.memory_unit.memory.get_memory_pointer(&segment, address + i)};
                    print!("0x{:02X?}\t", data);
                }
                return Ok(format!(""))                
            }
            _ => return Err(format!("bad argument '{}'", parameters[1]))
        };

        Ok(format!("{:#X}", val))
    }

    /**
     *# sop [optional bool]
     * The function chagnes the show_opcode parameter.
     * @param parameters: Vec containing the parameters of the command.
     */
    pub fn showop(&mut self, parameters: Vec<&str>) -> Result<String, String> {
        if parameters.len() > 2 {
            return Err("bad arguments".to_string())
        }
        let val = if parameters.len() == 1 {
            !self.print_instruction
        } else {
            match parameters[1] {
                _ if parameters[1] == "true" => true,
                _ if parameters[1] == "false" => false,
                _ => return Err("[val] should be true or false".to_string()),
            }
        };
        self.print_instruction = val;
        Ok(format!("print_opcode changed to {}", val))
    }
}