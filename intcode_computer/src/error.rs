use thiserror::Error;
use crate::opcode::{Opcode, ParameterMode};

use std::convert::TryFrom;


pub type Result<T> = std::result::Result<T, VMError>;

type OpcodeConversionError = <Opcode as TryFrom<u8>>::Error;
type ParameterModeConversionError = <ParameterMode as TryFrom<u8>>::Error;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Unknown opcode")]
    UnkownOpcode(#[from] OpcodeConversionError),
    #[error("Memory error")]
    MemoryError(#[from] MemoryError),
    #[error("Unknown parameter mode")]
    UnkownParameterMode(#[from] ParameterModeConversionError),
    #[error("Machine has been halted")]
    MachineHalted,
    #[error("Machine is blocked")]
    MachineBlocked,
    #[error("Machine has input that has not been processed")]
    InputAlreadyPopulated,
    #[error("Machine has no output to take")]
    NoOutput,
    #[error("Position Operand is negative")]
    NegativeAddress,
    #[error("Destination Operand is immediate")]
    ImmediateDestination,
    
    // TODO: Check for out of bounds access?
    // #[error("Tried to access location outside defined memory")]
    // OutOfBounds,
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

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Address {address} was not aligned to page size {page_size}")]
    NotAligned {
        address: usize,
        page_size: usize
    }
}