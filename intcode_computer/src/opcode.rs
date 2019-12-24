use num_enum::TryFromPrimitive;
use std::fmt;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Opcode {
     Add =  1,
     Mul =  2,
      In =  3,
     Out =  4,
     JNZ =  5,
      JZ =  6,
      LT =  7,
      EQ =  8,
    Halt = 99
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,   
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Add => write!(f, "Addition"),
            Opcode::Mul => write!(f, "Multiplication"),
            Opcode::In => write!(f, "Input"),
            Opcode::Out => write!(f, "Output"),           
            Opcode::JNZ => write!(f, "Jump if true"),
            Opcode::JZ => write!(f, "Jump if false"),            
            Opcode::LT => write!(f, "Less than"),
            Opcode::EQ => write!(f, "Equals"),
            Opcode::Halt => write!(f, "Halt"),
        }
    }
}
