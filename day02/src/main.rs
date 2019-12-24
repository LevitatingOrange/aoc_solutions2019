use pretty_env_logger;
use log::{info, error, debug};
use runfiles::Runfiles;
use std::fs::read_to_string;
use intcode_computer::error::*;
use intcode_computer::util::string_to_program;
use intcode_computer::virtual_machine::VirtualMachine;


fn solution_1(input: &str) -> Result<i32> {
    info!("Executing intcode program \"{}\"", input);
    let mut program = string_to_program(input);
    program[1] = 12;
    program[2] = 2;
    let mut vm = VirtualMachine::new(program)?;
    debug!("Memory before run: \"{:?}\"", vm.mem());
    vm.run()?;
    debug!("Memory after run: \"{:?}\"", vm.mem());
    Ok(vm.mem()[0])
}

fn solution_2(input: &str) -> Result<i32> {
    info!("Executing intcode program \"{}\"", input);
    let program = string_to_program(input);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut current_program = program.to_vec();
            current_program[1] = noun;
            current_program[2] = verb;
            let mut vm = VirtualMachine::new(current_program)?;

            vm.run()?;

            if vm.mem()[0] == 19690720 {
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

    match solution_1(&input) {
        Ok(val) => println!("Solution 1: {}", val),
        Err(err) => error!("Could not execute first program: {}", err),
    }

    match solution_2(&input) {
        Ok(val) => println!("Solution 2: {}", val),
        Err(err) => error!("Could not execute second program: {}", err),
    }

}