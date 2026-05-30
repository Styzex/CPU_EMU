use crate::{assembler::*, cpu::*};
use std::{range, time::Instant};

mod assembler;
mod cpu;
mod instructions;

fn main() {
    let mut cpu = CPU::new();
    //cpu.enable_debug();

    let program = ass_bin("code.s");
    mload(program, &mut cpu);
    readable();

    println!("Running in DEBUG MODE!!!");
    loop {
        let c = cpu.debug_run(); // Debug run function for the P1831 Processor with error handling and logs
        match c {
            CPUStatus::Halt => break,
            CPUStatus::Error(e) => {
                println!("Error: {}", e);
                break;
            }
            CPUStatus::Running => {}
        }
    }

    println!("Running in RELEASE MODE!!!");
    loop {
        let c = cpu.release_run();// Debug run function for the P1831 Processor with error handling and logs
        match c {
            0x01 => break,
            _ => {}
        }
    }
    
}
