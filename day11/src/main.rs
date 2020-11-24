use pretty_env_logger;
use log::{info, error, debug};
use runfiles::Runfiles;
use std::fs::read_to_string;
use intcode_computer::error::*;
use intcode_computer::util::string_to_program;
use intcode_computer::opcode::Opcode;
use std::convert::From;
use intcode_computer::virtual_machine::{VirtualMachine, VMState};
use intcode_computer::memory::MemoryValueType;
use std::iter::repeat_with;

#[derive(Debug, Eq, PartialEq, Clone)]
enum TileColor {
    Unchanged,
    Black,
    White,
}

impl TileColor {
    fn val(self) -> MemoryValueType {
        match self {
            TileColor::Unchanged => 0,
            TileColor::Black => 0,
            TileColor::White => 1,
        }
    }
}

impl From<MemoryValueType> for TileColor {
    fn from(val: MemoryValueType) -> Self {
        if val == 0 {
            TileColor::Black
        } else if val == 1 {
            TileColor::White
        } else {
            panic!("Unkown tilecolor!")
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down
}

struct Robot {
    brain: VirtualMachine,
    current_direction: Direction,
    current_position: (isize, isize),
    hull: Vec<Vec<TileColor>>,
}

const INITIAL_SIZE: usize = 200;

impl Robot {
    fn new(program: &[MemoryValueType]) -> Result<Self> {
        let hull = repeat_with(|| {
           let mut v = Vec::new();
           v.resize(INITIAL_SIZE, TileColor::Unchanged);
           v
        }).take(INITIAL_SIZE).collect();
        Ok(Robot {
            brain: VirtualMachine::new(program)?,
            current_direction: Direction::Up,
            current_position: (INITIAL_SIZE / 2, INITIAL_SIZE / 2),
            hull
        })
    }

    fn paint(&mut self) -> Result<()> {
        self.brain.input(self.hull[self.current_position.0][self.current_position.1].val())?;
        while self.brain.run()? != VMState::Halted {
            let color = TileColor::from(self.brain.output()?);
            self.hull[self.current_position.0][self.current_position.1] = color;
            self.brain.run()?;
            let dir = self.brain.output()?;

            self.current_direction = if dir == 0 {
                match self.current_direction {
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                }
            } else if dir == 1{
                match self.current_direction {
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                }
            } else {
                panic!("Unkown turn signal");
            };
            self.move();
            self.brain.input(self.hull[self.current_position.0][self.current_position.1].val())?;
        }
        Ok(())
    }

    fn move(&mut self) {
        self.current_position.0 += match self.current_direction {
            Direction::Left  => -1,
            Direction::Right => 1,
            Direction::Up    => 0,
            Direction::Down  => 0
        };
        self.current_position.1 += match self.current_direction {
            Direction::Left  => 0,
            Direction::Right => 0,
            Direction::Up    => 1,
            Direction::Down  => -1
        };
        if self.current_position.0 <= 0 {
            panic!("Board to small in x");
        }
        if self.current_position.0 >= INITIAL_SIZE -  {
            panic!("Board to small in x");
        }
        if self.current_position.1 <= 0 {
            panic!("Board to small in y");
        }
        if self.current_position.1 >= INITIAL_SIZE -  {
            panic!("Board to small in y");
        }
    }
}


fn solution_1(program: &[MemoryValueType]) -> Result<MemoryValueType> {
    let mut robot = Robot::new(program)?;
    robot.paint()?;

    let mut changed = 0;
    for xs in robot.hull {
        for y in xs {
            if y != TileColor::Unchanged {
                changed += 1;
            }
        }
    }
    Ok(changed)
    // let mut vm = VirtualMachine::new(program)?;

    // vm.input(1)?;

    // let mut code = 0;
    
    // while (vm.run()? != VMState::Halted) {
    //     code = vm.output()?;
    //     info!("Got {}", code);
        
    //     //info!("Got Non working op: {}", Opcode::try_from(code as u8)?);
    // }
    // Ok(code)
}

// fn solution_2(program: &[MemoryValueType]) -> Result<MemoryValueType> {
//     let mut vm = VirtualMachine::new(program)?;

//     vm.input(2)?;

//     let mut code = 0;
    
//     while (vm.run()? != VMState::Halted) {
//         code = vm.output()?;
//         info!("Got {}", code);
//     }
//     Ok(code)
// }

fn main() {
    pretty_env_logger::init();

    let r = Runfiles::create().unwrap();
    let path = r.rlocation("aoc_solutions/util/input_11");

    let input = read_to_string(path).unwrap();
    info!("Executing intcode program \"{}\"", input);
    let program = string_to_program(&input);

    match solution_1(&program) {
        Ok(val) => println!("Solution 1: {}", val),
        Err(err) => error!("Could not execute first program: {}", err),
    }

    // match solution_2(&program) {
    //     Ok(val) => println!("Solution 2: {}", val),
    //     Err(err) => error!("Could not execute second program: {}", err),
    // }

}