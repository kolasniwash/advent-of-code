use std::fs;

#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    x_start: i32,
    y_start: i32,
    x_end: i32,
    y_end: i32,
}

impl Instruction {
    fn new(action: Action, x_start: i32, y_start: i32, x_end: i32, y_end: i32) -> Instruction {
        Instruction {
            action,
            x_start,
            y_start,
            x_end,
            y_end,
        }
    }
}

fn parse_integers(locs: &str) -> (i32, i32) {
    let split_locs: Vec<&str> = locs.split(",").collect();
    let x_loc: i32 = if let Some(x) = split_locs.first() {
        x.parse::<i32>().expect("Failed to parse integer")
    } else {
        0
    };
    let y_loc: i32 = if let Some(y) = split_locs.last() {
        y.parse::<i32>().expect("Failed to parse integer")
    } else {
        0
    };
    (x_loc, y_loc)
}

fn parse_instructions(instructions: String) -> Vec<Instruction> {
    let mut parsed_instructions: Vec<Instruction> = Vec::new();

    for instruction in instructions.split("\n") {
        println!("{}", instruction);

        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let mut locs_idx = 1;
        let mut action_idx = 0;
        if parts.len() > 4 {
            locs_idx = 2;
            action_idx = 1;
        }

        let action: Action = if let Some(part) = parts.get(action_idx) {
            match *part {
                "on" => Action::On,
                "off" => Action::Off,
                "toggle" => Action::Toggle,
                _ => panic!("Invalid input: action type"),
            }
        } else {
            Action::On
        };

        let xy_start: (i32, i32) = if let Some(locs) = parts.get(locs_idx) {
            parse_integers(locs)
        } else {
            (0, 0)
        };

        let xy_end: (i32, i32) = if let Some(locs) = parts.last() {
            parse_integers(locs)
        } else {
            (0, 0)
        };

        parsed_instructions.push(Instruction {
            action,
            x_start: xy_start.0,
            y_start: xy_start.1,
            x_end: xy_end.0,
            y_end: xy_end.1,
        })
    }
    parsed_instructions
}

fn part_1_solution(
    mut lights: Vec<Vec<i32>>,
    parsed_instructions: Vec<Instruction>,
) -> Vec<Vec<i32>> {
    for parsed_instruction in parsed_instructions.iter() {
        for row in parsed_instruction.x_start..parsed_instruction.x_end + 1 {
            for col in parsed_instruction.y_start..parsed_instruction.y_end + 1 {
                match parsed_instruction.action {
                    Action::On => lights[row as usize][col as usize] = 1,
                    Action::Off => lights[row as usize][col as usize] = 0,
                    Action::Toggle => {
                        if 0 == lights[row as usize][col as usize] {
                            lights[row as usize][col as usize] = 1
                        } else if 1 == lights[row as usize][col as usize] {
                            lights[row as usize][col as usize] = 0
                        }
                    }
                }
            }
        }
    }
    lights
}

fn part_2_solution(
    mut lights: Vec<Vec<i32>>,
    parsed_instructions: Vec<Instruction>,
) -> Vec<Vec<i32>> {
    for parsed_instruction in parsed_instructions.iter() {
        for row in parsed_instruction.x_start..parsed_instruction.x_end + 1 {
            for col in parsed_instruction.y_start..parsed_instruction.y_end + 1 {
                match parsed_instruction.action {
                    Action::On => lights[row as usize][col as usize] += 1,
                    // Action::On => lights.get(),
                    Action::Off => {
                        if 0 < lights[row as usize][col as usize] {
                            lights[row as usize][col as usize] -= 1
                        }
                    }
                    Action::Toggle => lights[row as usize][col as usize] += 2,
                }
            }
        }
    }
    lights
}

fn main() {
    let instructions = fs::read_to_string(
        "/Users/nshaw/Code/kolasniwash/advent-of-code/rust/aoc2015/day-6/src/input.txt",
    )
    .expect("File not found");

    let parsed_instructions: Vec<Instruction> = parse_instructions(instructions);

    let mut lights: Vec<Vec<i32>> = Vec::with_capacity(1000);
    for _ in 0..1000 {
        lights.push(vec![0; 1000]);
    }

    // lights = part_1_solution(lights, parsed_instructions);
    lights = part_2_solution(lights, parsed_instructions);

    let total_on: i32 = lights.iter().map(|x| x.iter().sum::<i32>()).sum();

    println!("Total lights on: {}", total_on)
}
