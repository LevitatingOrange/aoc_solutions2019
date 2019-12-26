use crate::opcode::{Opcode, ParameterMode};
use crate::error::*;
use crate::memory::{Memory, MemoryValueType};
use std::convert::TryFrom;
use std::ops::{Add, Mul};
use log::{debug};
use std::fmt::{Display, Debug};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VMState {
    Paused,
    Blocked,
    Running,
    Halted
}

pub struct VirtualMachine {
    // TODO: I originally wanted to have a byte memory for space efficiency.
    // Sadly, that makes operand parsing non trivial so I opted for MemoryValueType's for
    // now
    memory: Memory,
    pc: usize,
    relative_base: usize,
    state: VMState,
    input_register: Option<MemoryValueType>,
    output_register: Option<MemoryValueType>
}

impl VirtualMachine {
    pub fn new(program: &[MemoryValueType]) -> Result<VirtualMachine> {
        let mut mem = Memory::new();
        mem.insert_contiguous(0, program);
        Ok(VirtualMachine {
            memory: mem,
            pc: 0,
            relative_base: 0,
            state: VMState::Paused,
            input_register: None,
            output_register: None
        })
    }

    fn opcode(&self) -> Result<Opcode> {
        let opcode = Opcode::try_from((self.memory[self.pc] % 100) as u8)?;
        Ok(opcode)
    }

    fn parameter_modes(&self) -> Result<[ParameterMode; 3]> {
        let fst = ParameterMode::try_from(((self.memory[self.pc] /   100) % 10) as u8)?;
        let snd = ParameterMode::try_from(((self.memory[self.pc] /  1000) % 10) as u8)?;
        let thd = ParameterMode::try_from(((self.memory[self.pc] / 10000) % 10) as u8)?;
        Ok([fst, snd, thd])
    }

    pub fn run(&mut self) -> Result<VMState> {
        let saved_pc = self.pc;
        self.state = VMState::Running;
        loop {
            match self.state {
                VMState::Running => (),
                // this just causes problems
                // state @ VMState::Blocked => if saved_pc == self.pc {
                //     // We did not step through a single instruction. This means
                //     // that a blocked VM has been started without providing
                //     // input or taking the output. This is probably a logical
                //     // error in the calling code, so we return an error here.
                //     return Err(VMError::MachineBlocked)
                // } else {
                //     return Ok(state)
                // },
                state => return Ok(state)
            };
            self.step()?;
        }
    }

    #[must_use]
    pub fn input(&mut self, val: MemoryValueType) -> Result<()> {
        if self.input_register.is_some() {
            return Err(VMError::InputAlreadyPopulated);
        }
        self.input_register = Some(val);
        Ok(())
    }

    #[must_use]
    pub fn output(&mut self) -> Result<MemoryValueType> {
        let val = self.output_register.take().ok_or(VMError::NoOutput)?;
        return Ok(val)
    }


    pub fn step(&mut self) -> Result<()> {
        if self.state == VMState::Halted {
            return Err(VMError::MachineHalted)
        }

        if self.state == VMState::Blocked {
            return Err(VMError::MachineBlocked)
        }

        if let Some(_) = self.output_register {
            return Err(VMError::MachineBlocked);
        }

        // if self.pc >= MEMORY_SIZE {
        //     return Err(VMError::MemorySize)
        // }

        let opcode = self.opcode()?;
        debug!("Step at {}: `{}` with parameter modes {:?}", self.pc, opcode, self.parameter_modes()?);

        match opcode {
            Opcode::Add => self.apply2(Add::add)?,
            Opcode::Mul => self.apply2(Mul::mul)?,

            Opcode::In => {
                if self.parameter_modes()?[0] == ParameterMode::Immediate {
                    return Err(VMError::ImmediateDestination);
                }
                if let Some(val) = self.input_register {
                    let in_address = self.param_address(0)?;
                    self.memory[in_address] = val;
                    self.input_register = None;
                    self.pc += 2;
                } else {
                    // No Value there, block
                    self.state = VMState::Blocked;
                }
            }
            Opcode::Out => {
                self.output_register = Some(self.param(0)?);
                self.state = VMState::Blocked;
                self.pc += 2;
            }
            Opcode::JNZ => self.jmp_condition(|x| x != 0)?,
            Opcode::JZ => self.jmp_condition(|x| x == 0)?,

            Opcode::LT => self.apply2(|x,y| if x  < y {1} else {0})?,
            Opcode::EQ => self.apply2(|x,y| if x == y {1} else {0})?,
            Opcode::RBO => {
                let address = self.param(0)? as isize;
                let new_base = self.relative_base as isize + address;
                if new_base < 0 {
                    return Err(VMError::NegativeAddress);
                }
                self.relative_base = new_base as usize;
                self.pc += 2;
            },

            Opcode::Halt => {
                self.state = VMState::Halted;
            }
        };
        Ok(())
    }

    fn jmp_condition(&mut self, cond: fn(MemoryValueType) -> bool) -> Result<()> {
        if cond(self.param(0)?) {
            let new_pc = self.param(1)?;
            if new_pc < 0 {
                return Err(VMError::NegativeAddress);
            }
            self.pc = new_pc as usize;
        } else {
            self.pc += 3;
        }
        Ok(())
    }

    fn param_address(&self, offset: usize) -> Result<usize> {
        let address = match self.parameter_modes()?[offset] {
            ParameterMode::Position => {
                self.memory[self.pc + offset + 1] as isize
            },
            ParameterMode::Immediate => {
                (self.pc + offset + 1) as isize
            },
            ParameterMode::Relative => {
                self.relative_base as isize + (self.memory[self.pc + offset + 1] as isize)
            }
        };

        if address < 0 {
            return Err(VMError::NegativeAddress)
        }
        Ok(address as usize)
        
    }

    fn param(&self, offset: usize) -> Result<MemoryValueType> {
        Ok(self.memory[self.param_address(offset)?])
    }

    fn apply2(&mut self, f:  fn(MemoryValueType, MemoryValueType) -> MemoryValueType) -> Result<()> {
        if self.parameter_modes()?[2] == ParameterMode::Immediate {
            return Err(VMError::ImmediateDestination);
        }
        let out_address = self.param_address(2)?;
        self.memory[out_address] = f(self.param(0)?, self.param(1)?);
        self.pc += 4;

        Ok(())
    } 
}

impl Index<usize> for VirtualMachine {
    type Output = MemoryValueType;

    fn index(&self, address: usize) -> &Self::Output {
        &self.memory[address]
    }
} 

impl IndexMut<usize> for VirtualMachine {
    fn index_mut(&mut self, address: usize) -> &mut Self::Output {
        &mut self.memory[address]
    }
}