use pretty_env_logger;
use log::{info, error, debug};
use runfiles::Runfiles;
use std::fs::read_to_string;
use intcode_computer::error::*;
use intcode_computer::util::string_to_program;
use intcode_computer::virtual_machine::{VirtualMachine, VMState};


fn solution_1(input: &str) -> Result<i32> {
    info!("Executing intcode program \"{}\"", input);
    let mut program = string_to_program(input);
    let mut vm = VirtualMachine::new(program)?;

    vm.input(1)?;

    let mut code = 0;
    
    while (vm.run()? != VMState::Halted) {
        code = vm.output()?;
        info!("Got diagnostics code 0");
    }
    Ok(code)
}

fn solution_2(input: &str) -> Result<i32> {
    info!("Executing intcode program \"{}\"", input);
    let mut program = string_to_program(input);
    let mut vm = VirtualMachine::new(program)?;

    vm.input(5)?;

    let mut code = 0;
    
    while (vm.run()? != VMState::Halted) {
        code = vm.output()?;
        info!("Got diagnostics code 0");
    }
    Ok(code)
}

fn main() {
    pretty_env_logger::init();

    let r = Runfiles::create().unwrap();
    let path = r.rlocation("aoc_solutions/util/input_05");

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