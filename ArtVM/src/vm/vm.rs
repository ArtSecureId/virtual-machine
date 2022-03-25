use log::{error, info};
use crate::vm::instructions::OpCode;

#[derive(Debug)]
pub struct ArtVM {
    registers: [i32; 32],
    program: Vec<u8>,
    pc: usize
}

impl ArtVM {
    pub fn new() -> Self {
        ArtVM {
            registers: [0; 32],
            program: vec![],
            pc: 0
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                println!("Exiting Program =>");
                break;
            }

            match self.decode_opcode() {
                OpCode::HLT => {
                    info!("HALT ArtVM");
                    return
                },
                OpCode::LOAD => {
                  let register = self.next_8_bits() as usize;
                  let value = self.next_16_bits();
                  self.registers[register] = value as i32;
                },
                OpCode::ADD => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 + register2;
                },
                OpCode::SUB => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 - register2;
                },
                OpCode::MUL => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 * register2;
                },
                OpCode::DIV => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];

                }
                _ => {
                    error!("Unrecognized instruction");
                    return
                }
            }

        }
    }

    fn decode_opcode(&mut self) -> OpCode {
        let opcode = OpCode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let val = self.program[self.pc];
        self.pc += 1;
        return val;
    }

    fn next_16_bits(&mut self) -> u16 {
        let val = ((self.program[self.pc] as u16) << 8) | self.program[self.pc+1] as u16;
        self.pc += 2;
        return val;
    }
}

#[cfg(test)]
mod VMTest {
    use crate::vm::vm::ArtVM;

    #[test]
    fn create_new_vm() {
        let artvm = ArtVM::new();
        assert_eq!(artvm.pc, 0);
        assert_eq!(artvm.registers[0], 0);
    }

    #[test]
    fn test_vm_igl() {
        let mut artvm = ArtVM::new();
        let program = vec![230,0,0,0];
        artvm.program = program;
        artvm.run();
        assert_eq!(artvm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut vm = ArtVM::new();
        let ins = vec![2,0,1,244];
        vm.program = ins;
        vm.run();
        assert_eq!(vm.registers[0], 500);
    }
}