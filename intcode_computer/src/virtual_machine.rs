use crate::opcode::Opcode;
use crate::error::*;
use std::convert::TryFrom;
use std::ops::{Add, Mul};
use log::info;

const MEMORY_SIZE: usize = 1 << 32;

#[derive(Debug, Eq, PartialEq)]
pub enum VMStatus {
    Running,
    Halted
}

pub struct VirtualMachine {
    // TODO: I originally wanted to have a byte memory for space efficiency.
    // Sadly, that makes operand parsing non trivial so I opted for u32's for
    // now
    memory: [u32; MEMORY_SIZE],
    pc: usize,
    status: VMStatus
}

impl VirtualMachine {
    pub fn run (&mut self) -> Result<()> {
        while self.status == VMStatus::Running {
            self.step()?;
        }
        Ok(())
    }
    pub fn step(&mut self) -> Result<()> {

        if self.status == VMStatus::Halted {
            return Err(VMError::MachineHalted)
        }

        let opcode = Opcode::try_from(self.memory[self.pc])?;
        info!("Step: `{:?}`", opcode);

        match opcode {
            Opcode::Add => {
                self.apply2(Add::add);
            },
            Opcode::Mul => {
                self.apply2(Mul::mul);
            },
            Opcode::Halt => {
                self.status = VMStatus::Halted;
            }
        };
        Ok(())
    }

    pub fn current_opcode(self) -> Result<Opcode> {
        let opcode = Opcode::try_from(self.memory[self.pc])?;
        Ok(opcode)
    }

    pub fn mem(&self, index: usize) -> &u32 {
        &self.memory[index]
    }

    pub fn mem_mut(&mut self, index: usize) -> &mut u32 {
        &mut self.memory[index]
    }

    fn apply2(&mut self, f:  fn(u32, u32) -> u32) {
        let lhs = self.pc + 1;
        let rhs = self.pc + 2;
        let out = self.pc + 3;
        self.memory[out] = f(self.memory[lhs], self.memory[rhs]);
        self.pc += 3;
    } 
}