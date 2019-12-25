use pretty_env_logger;
use log::{info, error, debug};
use runfiles::Runfiles;
use std::fs::read_to_string;
use intcode_computer::error::*;
use intcode_computer::util::string_to_program;
use intcode_computer::virtual_machine::{VirtualMachine, VMState};
use itertools::Itertools;

// fn run_amplifiers(program: &[i32], phase_settings: &[i32]) -> Result<i32> {
//     let mut input_signal = 0;
//     for phase_setting in phase_settings {
//         let mut vm = VirtualMachine::new(program.to_vec())?;
//         vm.input(*phase_setting)?;
//         vm.run()?;
//         vm.input(input_signal)?;
//         vm.run()?;
//         input_signal = vm.output()?;
//     }
//     Ok(input_signal)
// }

fn run_amplifiers(program: &[i32], phase_settings: &[i32]) -> Result<i32> {
    let mut amplifiers = Vec::with_capacity(phase_settings.len());

    for phase_setting in phase_settings {
        let mut amplifier = VirtualMachine::new(program.to_vec())?;
        amplifier.input(*phase_setting)?;
        // let machine process first input
        amplifier.run()?;
        amplifiers.push(amplifier);
    }

    let mut input_signal = 0;
    let mut state = VMState::Running;
    loop {
        for amplifier in &mut amplifiers {
            amplifier.input(input_signal)?;
            state = amplifier.run()?;
            if state == VMState::Halted {
                continue;
            }
            input_signal = amplifier.output()?;
        }
        if state == VMState::Halted {
            return Ok(input_signal)
        }
    }
}

fn solution_1(input: &str) -> Result<i32> {
    info!("Executing intcode program \"{}\"", input);
    let mut program = string_to_program(input);

    let mut max = 0;
     
    for phase_settings in (0..=4).permutations(5) {
        let new = run_amplifiers(&program, &phase_settings[0..5])?;
        if new > max {
            max = new;
        }
    }

    Ok(max)
}

fn solution_2(input: &str) -> Result<i32> {
    info!("Executing intcode program \"{}\"", input);
    let mut program = string_to_program(input);

    let mut max = 0;
     
    for phase_settings in (5..=9).permutations(5) {
        let new = run_amplifiers(&program, &phase_settings[0..5])?;
        if new > max {
            max = new;
        }
    }

    Ok(max)
}

// fn solution_2(input: &str) -> Result<i32> {
//     info!("Executing intcode program \"{}\"", input);
//     let mut program = string_to_program(input);
//     let mut vm = VirtualMachine::new(program)?;

//     vm.input(5)?;

//     let mut code = 0;
    
//     while (vm.run()? != VMState::Halted) {
//         code = vm.output()?;
//         info!("Got diagnostics code 0");
//     }
//     Ok(code)
// }

fn main() {
    pretty_env_logger::init();

    let r = Runfiles::create().unwrap();
    let path = r.rlocation("aoc_solutions/util/input_07");

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