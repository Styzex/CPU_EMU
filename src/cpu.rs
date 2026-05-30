use crate::instructions::*;

#[allow(dead_code)]
pub enum CPUStatus {
    Halt,
    Error(String),
    Running,
}

fn invalid_register_err(name: &str) -> CPUStatus {
    CPUStatus::Error(format!("{}: Invalid register", name))
}

#[derive(Clone, Copy)]
pub struct Block {
    pub value: u8,
    pub free: bool,
}
impl Block {
    pub fn new() -> Block {
        Block {
            value: 0u8,
            free: true,
        }
    }
}

#[allow(unused)]
pub struct CPU {
    pub mem: [Block; 64],
    r1: Block,
    r2: Block,
    pc: u8,
    debug: bool,
    i: u32,
    pub hlt: u8,
}

#[allow(unused)]
impl CPU {
    pub fn new() -> CPU {
        CPU {
            mem: [Block::new(); 64],
            r1: Block::new(),
            r2: Block::new(),
            pc: 0u8,
            debug: false,
            i: 0u32,
            hlt: 0u8,
        }
    }

    fn get_operand(&self, i: u8) -> u8 {
        return self.mem[(self.pc + i) as usize].value;
    }

    pub fn enable_debug(&mut self) {
        self.debug = true;
    }

    pub fn get_executed_instructions(&self) -> u32 {
        return self.i;
    }

    pub fn run(&mut self) -> CPUStatus {
        if (self.pc as usize) + 1 >= self.mem.len() {
            return self.halt();
        }

        let opcode = self.mem[(self.pc) as usize].value;
        if opcode != 0x00 && self.debug == true {
            println!(
                "Debug: PC={}, Opcode=0x{:02X}, R1={}, R2={}",
                self.pc, opcode, self.r1.value, self.r2.value
            );
            println!("Executed instrucions: {}", self.i);
        }

        match opcode {
            JMP => {
                let operand = self.get_operand(1);
                let status = self.jmp(operand as usize);
                self.i += 1;
                return status;
            }
            JZ => {
                let operand1 = self.get_operand(1);
                let operand2 = self.get_operand(2);
                let status = self.jz(operand1, operand2 as usize);
                self.i += 1;
                return status;
            }
            JNZ => {
                let operand1 = self.get_operand(1);
                let operand2 = self.get_operand(2);
                let status = self.jnz(operand1, operand2 as usize);
                self.i += 1;
                return status;
            }
            LOD => {
                let operand1 = self.get_operand(1);
                let operand2 = self.get_operand(2);
                let status = self.load(operand1, operand2 as usize);
                self.pc += 3;
                self.i += 1;
                return status;
            }
            STR => {
                let operand1 = self.get_operand(1);
                let operand2 = self.get_operand(2);
                let status = self.store(operand1, operand2 as usize);
                self.pc += 3;
                self.i += 1;
                return status;
            }
            MOV => {
                let operand1 = self.get_operand(1);
                let operand2 = self.get_operand(2);
                let status = self.mov(operand1, operand2);
                self.pc += 3;
                self.i += 1;
                return status;
            }
            ADD => {
                let operand1 = self.get_operand(1);
                let operand2 = self.get_operand(2);
                let status = self.add(operand1, operand2);
                self.pc += 3;
                self.i += 1;
                return status;
            }
            SUB => {
                let operand1 = self.get_operand(1);
                let operand2 = self.get_operand(2);
                let status = self.subtract(operand1, operand2);
                self.pc += 3;
                self.i += 1;
                return status;
            }
            HLT => self.halt(),
            _ => {
                self.pc += 1;
                self.i += 1;
                CPUStatus::Running
            }
        }
    }

    fn jmp(&mut self, addr: usize) -> CPUStatus {
        if addr >= self.mem.len() {
            return CPUStatus::Error(String::from("JMP: Invalid jump address"));
        }
        self.pc = addr as u8;
        println!("JMP: memory[{}] = {}", addr, self.mem[addr].value);
        return CPUStatus::Running;
    }

    fn jz(&mut self, reg: u8, addr: usize) -> CPUStatus {
        if addr >= self.mem.len() {
            return CPUStatus::Error(String::from("JZ: Invalid jump address"));
        }
        match reg {
            R1 => {
                if self.r1.value <= 0 {
                    self.pc = addr as u8;
                    println!(
                        "JZ: r1 = {} memory[{}] = {}",
                        self.r1.value, addr, self.mem[addr].value
                    );
                } else {
                    self.pc += 3;
                }
            }
            R2 => {
                if self.r2.value <= 0 {
                    self.pc = addr as u8;
                    println!(
                        "JZ: r1 = {} memory[{}] = {}",
                        self.r1.value, addr, self.mem[addr].value
                    );
                } else {
                    self.pc += 3;
                }
            }
            _ => return CPUStatus::Running,
        }
        return CPUStatus::Running;
    }

    fn jnz(&mut self, reg: u8, addr: usize) -> CPUStatus {
        if addr >= self.mem.len() {
            return CPUStatus::Error(String::from("JNZ: Invalid jump address"));
        }
        match reg {
            R1 => {
                if self.r1.value != 0 {
                    self.pc = addr as u8;
                    println!(
                        "JNZ: r1 = {} memory[{}] = {}",
                        self.r1.value, addr, self.mem[addr].value
                    );
                } else {
                    self.pc += 3;
                }
            }
            R2 => {
                if self.r2.value != 0 {
                    self.pc = addr as u8;
                    println!(
                        "JNZ: r2 = {} memory[{}] = {}",
                        self.r2.value, addr, self.mem[addr].value
                    );
                } else {
                    self.pc += 3;
                }
            }
            _ => return invalid_register_err("JNZ"),
        }
        return CPUStatus::Running;
    }

    fn load(&mut self, reg: u8, addr: usize) -> CPUStatus {
        if addr >= self.mem.len() {
            return CPUStatus::Error(String::from("LOD: Memory address out of bounds"));
        }
        if reg == R1 {
            self.r1.value = self.mem[addr].value;
            self.mem[addr].free = true;
            println!("LOD: memory[{}] -> r1 = {}", addr, self.r1.value);
            return CPUStatus::Running;
        } else if reg == R2 {
            self.r2.value = self.mem[addr].value;
            self.mem[addr].free = true;
            println!("LOD: memory[{}] -> r2 = {}", addr, self.r2.value);
            return CPUStatus::Running;
        } else {
            return invalid_register_err("LOD");
        }
    }

    fn store(&mut self, reg: u8, addr: usize) -> CPUStatus {
        if addr >= self.mem.len() {
            return CPUStatus::Error(String::from("LOD: Memory address out of bounds"));
        }
        if reg == R1 {
            self.mem[addr].value = self.r1.value;
            self.mem[addr].free = false;
            println!("STR: r1 -> memory[{}] = {}", addr, self.mem[addr].value);
            return CPUStatus::Running;
        } else if reg == R2 {
            self.mem[addr].value = self.r2.value;
            self.mem[addr].free = false;
            println!("STR: r2 -> memory[{}] = {}", addr, self.mem[addr].value);
            return CPUStatus::Running;
        } else {
            return invalid_register_err("STR");
        }
    }

    fn mov(&mut self, reg: u8, val: u8) -> CPUStatus {
        if reg == R1 {
            self.r1.value = val;
            println!("MOV: {} -> R1", val);
            return CPUStatus::Running;
        } else if reg == R2 {
            self.r2.value = val;
            println!("MOV: {} -> R2", val);
            return CPUStatus::Running;
        } else {
            return invalid_register_err("MOV");
        }
    }

    fn subtract(&mut self, reg1: u8, reg2: u8) -> CPUStatus {
        match reg1 {
            R1 => {
                let pre = self.r1.value;
                let (result, overflow) = self.r1.value.overflowing_sub(self.r2.value);
                if result == pre && self.r2.value != 0 {
                    return CPUStatus::Error(String::from("SUB: Failed"));
                } else if overflow {
                    return CPUStatus::Error(String::from("SUB: Overflow"));
                }
                self.r1.value = result;
                println!("SUB: r1 - r2 -> r1 = {}", self.r1.value);
                return CPUStatus::Running;
            }
            R2 => {
                let pre = self.r2.value;
                let (result, overflow) = self.r2.value.overflowing_sub(self.r1.value);
                if result == pre && self.r1.value != 0 {
                    return CPUStatus::Error(String::from("SUB: Failed"));
                } else if overflow {
                    return CPUStatus::Error(String::from("SUB: Overflow"));
                }
                self.r2.value = result;
                println!("SUB: r2 - r1 -> r2 = {}", self.r2.value);
                return CPUStatus::Running;
            }
            _ => return invalid_register_err("SUB"),
        }
    }

    fn add(&mut self, reg1: u8, reg2: u8) -> CPUStatus {
        if reg1 == R1 {
            let pre = self.r1.value;
            let (result, overflow) = self.r1.value.overflowing_add(self.r2.value);
            if result == pre && self.r2.value != 0 {
                return CPUStatus::Error(String::from("ADD: Failed"));
            } else if overflow {
                return CPUStatus::Error(String::from("ADD: Overflow"));
            }
            self.r1.value = result;
            println!("ADD: r1 + r2 -> r1 = {}", self.r1.value);
            return CPUStatus::Running;
        } else if reg1 == R2 {
            let pre = self.r2.value;
            let (result, overflow) = self.r2.value.overflowing_add(self.r1.value);
            if result == pre && self.r1.value != 0 {
                return CPUStatus::Error(String::from("ADD: Failed"));
            } else if overflow {
                return CPUStatus::Error(String::from("ADD: Overflow"));
            }
            self.r2.value = result;
            println!("ADD: r2 + r1 -> r2 = {}", self.r2.value);
            return CPUStatus::Running;
        } else {
            return self.halt();
        }
    }

    fn halt(&self) -> CPUStatus {
        println!("HLT");
        return CPUStatus::Halt;
    }

    pub fn fast_run(&mut self) {
        let opcode = self.mem[(self.pc) as usize].value;
        let operand1 = self.get_operand(1);
        let operand2 = self.get_operand(2);

        match opcode {
            JMP => {
                self.fast_jmp(operand1);
                self.i += 1;
            }
            JZ => {
                self.fast_jz(operand1, operand2);
                self.i += 1;
            }
            JNZ => {
                self.fast_jnz(operand1, operand2);
                self.i += 1;
            }
            LOD => {
                self.fast_load(operand1, operand2);
                self.pc += 3;
                self.i += 1;
            }
            STR => {
                self.fast_store(operand1, operand2);
                self.pc += 3;
                self.i += 1;
            }
            MOV => {
                self.fast_mov(operand1, operand2);
                self.pc += 3;
                self.i += 1;
            }
            ADD => {
                self.fast_add(operand1, operand2);
                self.pc += 3;
                self.i += 1;
            }
            SUB => {
                self.fast_subtract(operand1, operand2);
                self.pc += 3;
                self.i += 1;
            }
            HLT => self.fast_halt(),
            _ => {
                self.pc += 1;
                self.i += 1;
            }
        }
    }

    fn fast_jmp(&mut self, addr: u8) {
        self.pc = addr;
    }

    fn fast_jz(&mut self, reg: u8, addr: u8) {
        match reg {
            R1 => {
                if self.r1.value == 0 {
                    self.pc = addr;
                } else {
                    self.pc += 3;
                }
            }
            R2 => {
                if self.r2.value == 0 {
                    self.pc = addr;
                } else {
                    self.pc += 3;
                }
            }
            _ => {}
        }
    }

    fn fast_jnz(&mut self, reg: u8, addr: u8) {
        match reg {
            R1 => {
                if self.r1.value != 0 {
                    self.pc = addr;
                } else {
                    self.pc += 3;
                }
            }
            R2 => {
                if self.r2.value != 0 {
                    self.pc = addr;
                } else {
                    self.pc += 3;
                }
            }
            _ => {}
        }
    }

    fn fast_load(&mut self, reg: u8, addr: u8) {
        match reg {
            R1 => {
                self.r1.value = self.mem[addr as usize].value;
            }
            R2 => {
                self.r2.value = self.mem[addr as usize].value;
            }
            _ => {}
        }
    }

    fn fast_store(&mut self, reg: u8, addr: u8) {
        match reg {
            R1 => {
                self.mem[addr as usize].value = self.r1.value;
            }
            R2 => {
                self.mem[addr as usize].value = self.r2.value;
            }
            _ => {}
        }
    }

    fn fast_mov(&mut self, reg: u8, val: u8) {
        match reg {
            R1 => {
                self.r1.value = val;
            }
            R2 => {
                self.r2.value = val;
            }
            _ => {}
        }
    }

    fn fast_subtract(&mut self, reg1: u8, reg2: u8) {
        match reg1 {
            R1 => {
                self.r1.value -= self.r2.value;
            }
            R2 => {
                self.r2.value -= self.r1.value;
            }
            _ => {}
        }
    }

    fn fast_add(&mut self, reg1: u8, reg2: u8) {
        match reg1 {
            R1 => {
                self.r1.value += self.r2.value;
            }
            R2 => {
                self.r2.value += self.r1.value;
            }
            _ => {}
        }
    }

    fn fast_halt(&mut self) {
        self.hlt = 1;
    }
}
