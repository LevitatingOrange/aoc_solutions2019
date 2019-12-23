use num_enum::TryFromPrimitive;
use std::fmt;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum Opcode {
    Add = 1,
    Mul = 2,
    Halt = 99
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Add => write!(f, "Addition"),
            Opcode::Mul => write!(f, "Multiplication"),
            Opcode::Halt => write!(f, "Halt"),
        }
    }
}
