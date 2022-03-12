use log::{error, info};
use crate::vm::instructions::OpCode;

#[derive(Debug)]
pub struct AsVM {
    registers: [i32; 32],
    program: Vec<u8>,
    pc: usize
}

impl AsVM {
    pub fn new() -> Self {
        AsVM {
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
                    info!("HALT AsVM");
                    return
                },
                OpCode::LOAD => {
                  let register = self.next_8_bits() as usize;
                  let value = self.next_16_bits();
                  self.registers[register] = value as i32;
                },
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
    use crate::vm::vm::AsVM;

    #[test]
    fn create_new_vm() {
        let asvm = AsVM::new();
        assert_eq!(asvm.pc, 0);
        assert_eq!(asvm.registers[0], 0);
    }

    #[test]
    fn test_vm_igl() {
        let mut asvm = AsVM::new();
        let program = vec![230,0,0,0];
        asvm.program = program;
        asvm.run();
        assert_eq!(asvm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut vm = AsVM::new();
        let ins = vec![2,0,1,244];
        vm.program = ins;
        vm.run();
        assert_eq!(vm.registers[0], 500);
    }
}