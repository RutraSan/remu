#![allow(dead_code)]
use std::process::Command;
use std::fs;

/**
 * This function is responsible for loading the program.
 * @param file_path: path to the file to open.
 * @return: reference to the contents of the file.
 * TODO: the function should get both binary files and .asm files.
*/
pub fn get_program(file_path: &str) -> Vec<u8> {

    if file_path.ends_with(".asm") {
        let nasm_command = format!("nasm -f bin {} -o C:\\temp\\emu", file_path); 
        
        // the command to run
        Command::new("cmd")
        .args(["/C", &nasm_command])
        .output()
        .expect("failed to execute process");
        
        // read data from new file
        let data = fs::read("C:\\temp\\emu").expect("Unable to compile");
        return data
    }
    else{
        let data = fs::read(file_path).expect("Unable to read file");
        return data
    } 
}