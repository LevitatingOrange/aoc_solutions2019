use pretty_env_logger;
use log::{info, error, debug};
use runfiles::Runfiles;
use std::fs::read_to_string;
use intcode_computer::error::*;
use intcode_computer::util::string_to_program;
use intcode_computer::virtual_machine::VirtualMachine;
use intcode_computer::memory::MemoryValueType;


fn solution_1(program: &[MemoryValueType]) -> Result<MemoryValueType> {
    let mut vm = VirtualMachine::new(program)?;

    vm.run()?;
    Ok(vm[0])
}

fn solution_2(program: &[MemoryValueType]) -> Result<MemoryValueType> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut vm = VirtualMachine::new(program)?;
            vm[1] = noun;
            vm[2] = verb;
            vm.run()?;

            if vm[0] == 19690720 {
                return Ok(100 * noun + verb)
            }
        }
    }
    // TODO: lazy
    unreachable!();
}

fn main() {
    pretty_env_logger::init();

    let r = Runfiles::create().unwrap();
    let path = r.rlocation("aoc_solutions/util/input_02");

    let input = read_to_string(path).unwrap();
    info!("Executing intcode program \"{}\"", input);
    let mut program = string_to_program(&input);

    match solution_1(&program) {
        Ok(val) => println!("Solution 1: {}", val),
        Err(err) => error!("Could not execute first program: {}", err),
    }

    match solution_2(&program) {
        Ok(val) => println!("Solution 2: {}", val),
        Err(err) => error!("Could not execute second program: {}", err),
    }

}