mod opcodes;

use opcodes::Opcodes;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::process;

struct Cpu {
    reg: [u16; 8],
    memory: [u16; 32768],
    pc: u16,
    op: Opcodes,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: [0u16; 8],
            memory: [0u16; 32768],
            pc: 0,
            op: Opcodes::OpNoop,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.emulate_instruction();
        }
    }

    fn read_memory(&mut self) -> u16 {
        match self.pc {
            0..=32768 => {
                let val = self.memory[self.pc as usize];
                self.pc += 1;
                val
            }
            _ => panic!(),
        }
    }

    fn emulate_instruction(&mut self) -> io::Result<()> {
        let instr = self.read_memory();
        let op = Opcodes::from_u16(instr);
        match op {
            Opcodes::OpHalt => process::exit(1),
            Opcodes::OpSet => todo!(),
            Opcodes::OpPush => todo!(),
            Opcodes::OpPop => todo!(),
            Opcodes::OpEq => todo!(),
            Opcodes::OpGt => todo!(),
            Opcodes::OpJmp => {
                self.pc = self.read_memory();
            }
            Opcodes::OpJt => {
                let a = self.read_memory();
                let b = self.read_memory();
                if a != 0 {
                    self.pc = b;
                }
            }
            Opcodes::OpJf => {
                let a = self.read_memory();
                let b = self.read_memory();
                if a == 0 {
                    self.pc = b;
                }
            }
            Opcodes::OpAdd => todo!(),
            Opcodes::OpMult => todo!(),
            Opcodes::OpMod => todo!(),
            Opcodes::OpAnd => todo!(),
            Opcodes::OpOr => todo!(),
            Opcodes::OpNot => todo!(),
            Opcodes::OpRmem => todo!(),
            Opcodes::OpWmem => todo!(),
            Opcodes::OpCall => todo!(),
            Opcodes::OpRet => todo!(),
            Opcodes::OpOut => {
                let code = self.read_memory().to_le_bytes();
                let str = match std::str::from_utf8(&code) {
                    Ok(val) => val,
                    Err(_) => "",
                };
                print!("{}", str);
            }
            Opcodes::OpIn => todo!(),
            Opcodes::OpNoop => {}
        }

        Ok(())
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

pub fn read_file(path: String) -> io::Result<[u16; 32768]> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    let mut memory: [u16; 32768] = [0u16; 32768];

    reader.read_to_end(&mut buffer)?;

    let mut index = 0;
    for i in (0..buffer.len()).step_by(2) {
        memory[index] = u16::from_le_bytes([buffer[i], buffer[i + 1]]);
        index += 1;
    }

    Ok(memory)
}

fn main() -> io::Result<()> {
    let mut cpu: Cpu = Default::default();
    cpu.memory = read_file(String::from("./res/challenge.bin"))?;
    cpu.run();

    Ok(())
}
