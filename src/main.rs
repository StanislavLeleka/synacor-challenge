mod opcodes;

use std::fs::File;
use std::io::{self, Read, BufReader};
use std::process;
use opcodes::Opcodes;

struct Cpu {
    reg: [u16; 8],
    memory: [u16; 32768],
    pc: u16,
    op: Opcodes
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { reg: [0u16; 8], memory: [0u16; 32768], pc: 0, op: Opcodes::OpNoop }
    }

    pub fn run(&mut self) {
        loop {
            self.emulate_instruction();
        }
    }

    fn read_memory(&self, address: usize) -> u16 {
        match address {
            0 ..=32768 => self.memory[address],
            _ => panic!()
        }
    }

    fn emulate_instruction(&mut self) -> io::Result<()>  {
        let instr = self.read_memory(self.pc as usize);
        let op = Opcodes::from_u16(instr);
        match op {
            Opcodes::OpHalt => process::exit(1),
            Opcodes::OpSet => todo!(),
            Opcodes::OpPush => todo!(),
            Opcodes::OpPop => todo!(),
            Opcodes::OpEq => todo!(),
            Opcodes::OpGt => todo!(),
            Opcodes::OpJmp => todo!(),
            Opcodes::OpJt => todo!(),
            Opcodes::OpJf => todo!(),
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
                self.pc += 1;
                let code = self.read_memory(self.pc as usize).to_le_bytes();
                let str = match std::str::from_utf8(&code){
                    Ok(val) => val,
                    Err(_) => "",
                };
                print!("{}", str);
                self.pc += 1;
            },
            Opcodes::OpIn => todo!(),
            Opcodes::OpNoop => { 
                self.pc += 1
            },
        }
        
        Ok(())
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

pub fn read_file(path: String)-> io::Result<[u16; 32768]> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    let mut memory: [u16; 32768] = [0u16; 32768];

    reader.read_to_end(&mut buffer)?;

    let mut index = 0;
    for i in (0..buffer.len()).step_by(2) {
        memory[index] = u16::from_le_bytes([buffer[i], buffer[i+1]]);
        index += 1;
    }

    Ok(memory)
}

fn main() -> io::Result<()>  {
    let mut cpu: Cpu = Default::default();
    cpu.memory = read_file(String::from("./res/challenge.bin"))?;
    cpu.run();

    Ok(())
}
