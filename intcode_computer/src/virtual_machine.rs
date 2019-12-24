use crate::opcode::{Opcode, ParameterMode};
use crate::error::*;
use std::convert::TryFrom;
use std::ops::{Add, Mul};
use log::{info, debug};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VMState {
    Paused,
    Blocked,
    Running,
    Halted
}

pub struct VirtualMachine {
    // TODO: I originally wanted to have a byte memory for space efficiency.
    // Sadly, that makes operand parsing non trivial so I opted for i32's for
    // now
    memory: Vec<i32>,
    pc: usize,
    state: VMState,
    input_register: Option<i32>,
    output_register: Option<i32>
}

impl VirtualMachine {
    pub fn new(program: Vec<i32>) -> Result<VirtualMachine> {
        Ok(VirtualMachine {
            memory: program,
            pc: 0,
            state: VMState::Paused,
            input_register: None,
            output_register: None
        })
    }

    fn opcode(&self) -> Result<Opcode> {
        let opcode = Opcode::try_from((self.memory[self.pc] % 100) as u8)?;
        Ok(opcode)
    }

    fn parameter_modes(&self) -> Result<(ParameterMode, ParameterMode, ParameterMode)> {
        let fst = ParameterMode::try_from(((self.memory[self.pc] /   100) % 10) as u8)?;
        let snd = ParameterMode::try_from(((self.memory[self.pc] /  1000) % 10) as u8)?;
        let thd = ParameterMode::try_from(((self.memory[self.pc] / 10000) % 10) as u8)?;
        Ok((fst, snd, thd))
    }

    pub fn run(&mut self) -> Result<VMState> {
        let saved_pc = self.pc;
        self.state = VMState::Running;
        loop {
            match self.state {
                VMState::Running => (),
                state @ VMState::Blocked => if saved_pc == self.pc {
                    // We did not step through a single instruction. This means
                    // that a blocked VM has been started without providing
                    // input or taking the output. This is probably a logical
                    // error in the calling code, so we return an error here.
                    return Err(VMError::MachineBlocked)
                } else {
                    return Ok(state)
                },
                state => return Ok(state)
            };
            self.step()?;
        }
    }

    #[must_use]
    pub fn input(&mut self, val: i32) -> Result<()> {
        if self.input_register.is_some() {
            return Err(VMError::InputAlreadyPopulated);
        }
        self.input_register = Some(val);
        Ok(())
    }

    #[must_use]
    pub fn output(&mut self) -> Result<i32> {
        let val = self.output_register.ok_or(VMError::NoOutput)?;
        self.output_register = None;
        self.pc += 2;
        return Ok(val)
    }


    pub fn step(&mut self) -> Result<()> {
        if self.state == VMState::Halted {
            return Err(VMError::MachineHalted)
        }

        if self.state == VMState::Blocked {
            return Err(VMError::MachineBlocked)
        }

        // if self.pc >= MEMORY_SIZE {
        //     return Err(VMError::MemorySize)
        // }

        let opcode = self.opcode()?;
        info!("Step: `{}`", opcode);

        match opcode {
            Opcode::Add => self.apply2(Add::add)?,
            Opcode::Mul => self.apply2(Mul::mul)?,

            Opcode::In => {
                if self.parameter_modes()?.2 == ParameterMode::Immediate {
                    return Err(VMError::ImmediateDestination);
                }
                if let Some(val) = self.input_register {
                    // Take value out of input register 
                    let out = self.memory[self.pc + 1] as usize;
                    self.memory[out] = val;
                    self.input_register = None;
                    self.pc += 2;
                } else {
                    // No Value there, block
                    self.state = VMState::Blocked;
                }
            }
            Opcode::Out => {
                if let Some(_) = self.input_register {
                    // Output value still there, block
                    self.state = VMState::Blocked;
                } else {
                    // Set output value
                    self.output_register = Some(self.param(1, self.parameter_modes()?.0)?);
                    self.state = VMState::Blocked;
                }
            }
            Opcode::JNZ => self.jmp_condition(|x| x != 0)?,
            Opcode::JZ => self.jmp_condition(|x| x == 0)?,

            Opcode::LT => self.apply2(|x,y| if x  < y {1} else {0})?,
            Opcode::EQ => self.apply2(|x,y| if x == y {1} else {0})?,

            Opcode::Halt => {
                self.state = VMState::Halted;
            }
        };
        Ok(())
    }

    pub fn mem(&self) -> & [i32] {
        &self.memory[..]
    }

    pub fn mem_mut(&mut self) -> &mut [i32] {
        &mut self.memory[..]
    }

    fn jmp_condition(&mut self, cond: fn(i32) -> bool) -> Result<()> {
        if cond(self.param(1, self.parameter_modes()?.0)?) {
            let new_pc = self.param(2, self.parameter_modes()?.1)?;
            if new_pc < 0 {
                return Err(VMError::NegativeAddress);
            }
            self.pc = new_pc as usize;
        } else {
            self.pc += 3;
        }
        Ok(())
    }

    fn param(&mut self, offset: usize, mode: ParameterMode) -> Result<i32> {
        Ok(match mode {
            ParameterMode::Position => {
                if self.memory[self.pc + offset] < 0 {
                    return Err(VMError::NegativeAddress)
                }
                self.memory[self.memory[self.pc + offset] as usize]
            },
            ParameterMode::Immediate => {
                self.memory[self.pc + offset]

            }
        })
        
    }
    fn apply2(&mut self, f:  fn(i32, i32) -> i32) -> Result<()> {
        let (lhs_mode, rhs_mode, out_mode) = self.parameter_modes()?;
        if out_mode == ParameterMode::Immediate {
            return Err(VMError::ImmediateDestination);
        }
        if self.memory[self.pc + 3] < 0 {
            return Err(VMError::NegativeAddress);
        }

        let lhs = self.param(1, lhs_mode)?;
        let rhs = self.param(2, rhs_mode)?;
        let out = self.memory[self.pc + 3] as usize;

        self.memory[out] = f(lhs, rhs);
        self.pc += 4;

        Ok(())
    } 
}