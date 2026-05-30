use crate::{assembler::*, cpu::*};
use std::time::Instant;

mod assembler;
mod cpu;
mod instructions;

fn main() {
    let mut cpu = CPU::new();
    //cpu.enable_debug();

    //ass_to_bin("code.s".to_string());
    //load_into_memory("code.bin".to_string(), &mut cpu);
    let program = ass_bin("code.s");
    mload(program, &mut cpu);
    readable();

    /*
    loop {
        let c = cpu.run(); // Debug run function for the P1831 Processor with error handling and logs
        match c {
            CPUStatus::Halt => break,
            CPUStatus::Error(e) => {
                println!("Error: {}", e);
                break;
            }
            CPUStatus::Running => {}
        }
    }
    */

    let mut instructions: f64 = 0.0;
    let start = Instant::now();
    for _ in 0u64..=(u32::MAX as u64 * 3) {
        cpu.fast_run(); // Optimized run function for the P1831 Processor
        instructions += 1.0;
    }
    let duration = start.elapsed();

    println!("Total time: {}", duration.as_secs_f64());
    println!("Total instructions: {}", instructions);
    println!(
        "{} GHz",
        instructions / duration.as_secs_f64() / 1000000000.0
    )
}
