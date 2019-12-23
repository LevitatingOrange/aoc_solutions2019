use thiserror::Error;
use crate::opcode::Opcode;

use std::convert::TryFrom;


pub type Result<T> = std::result::Result<T, VMError>;

type OpcodeConversionError = <Opcode as TryFrom<u32>>::Error;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Unknown opcode")]
    UnkownOpcode(#[from] OpcodeConversionError),
    #[error("Machine has been halted")]
    MachineHalted,
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("unknown data store error")]
    // Unknown,
}