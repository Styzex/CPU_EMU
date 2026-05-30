use std::fs;

use crate::{cpu::*, instructions::*};

pub fn ass_bin(path: &str) -> String {
    let contents = fs::read_to_string(path).unwrap();
    let mut binstructions: Vec<u8> = Vec::new();
    for l in contents.lines() {
        for i in l.split_whitespace() {
            match i {
                "JMP" => binstructions.push(JMP),
                "JZ" => binstructions.push(JZ),
                "JNZ" => binstructions.push(JNZ),
                "LOD" => binstructions.push(LOD),
                "STR" => binstructions.push(STR),
                "MOV" => binstructions.push(MOV),
                "ADD" => binstructions.push(ADD),
                "SUB" => binstructions.push(SUB),
                "HLT" => binstructions.push(HLT),
                "R1" => binstructions.push(R1),
                "R2" => binstructions.push(R2),
                _ => {
                    //println!("{}", i); // Just some debugging
                    binstructions.push(u8::from_str_radix(i.trim_start_matches("0x"), 16).unwrap())
                }
            }
        }
    }
    let n_path = path.trim_end_matches(".s").to_string() + ".bin";
    fs::write(&n_path, &binstructions).unwrap();
    return path.trim_end_matches(".s").to_string() + ".bin";
}

pub fn mload(path: String, cpu: &mut CPU) {
    let program = fs::read(path).unwrap();
    for i in 0..program.len() {
        cpu.mem[i + OFFSET].value = program[i];
    }
}

pub fn readable() {
    let bytes = fs::read("code.bin").unwrap();
    let hex = bytes
        .iter()
        .map(|b| format!("0x{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ");
    fs::write("code.hex", hex).unwrap();
}
