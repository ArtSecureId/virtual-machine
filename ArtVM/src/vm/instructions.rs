use crate::vm::instructions::OpCode::{HLT, IGL};

#[derive(Debug, PartialEq)]
pub enum OpCode {
    HLT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPB,
    JMPF,
    EQ,
    JEQ,
    JNEQ,
}

impl From<u8> for OpCode {
    fn from(opcode: u8) -> Self {
        match opcode {
           0 => OpCode::HLT,
           2 => OpCode::LOAD,

           _ => OpCode::IGL
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: OpCode
}

impl Instruction {
    pub fn new(opcode: OpCode) -> Self {
        Instruction {
            opcode
        }
    }
}

#[cfg(test)]
mod InstructionTest {

    use super::*;

    #[test]
    fn test_create_halt() {
        let opcode = OpCode::HLT;
        assert_eq!(opcode, OpCode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(OpCode::HLT);
        assert_eq!(instruction.opcode, OpCode::HLT);
    }
}