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

fn parse_instructions(instructions: String) -> Vec<Instruction> {
    let mut parsed_instructions: Vec<Instruction> = Vec::new();
    for instruction in instructions.split("\n") {
        let instruction_parts: Vec<&str> = instruction.split(",").collect();

        let first_part: Vec<&str> = instruction_parts[0]
            .split("turn")
            .map(|x| x.trim())
            .collect();
        let first_part_split: Vec<&str> = first_part[first_part.len() - 1].split(" ").collect();

        let action: Action = match first_part_split[0] {
            "on" => Action::On,
            "off" => Action::Off,
            "toggle" => Action::Toggle,
            _ => Action::On,
        };

        let x_start: i32 = first_part_split[1]
            .parse()
            .expect("Failed to convert to int");

        let second_part: Vec<&str> = instruction_parts[1].split(" ").collect();

        let y_start: i32 = second_part[0].parse().expect("Failed to convert to int");
        let x_end: i32 = second_part[second_part.len() - 1]
            .parse()
            .expect("Failed to convert to int");

        let y_end: i32 = instruction_parts[instruction_parts.len() - 1]
            .parse()
            .expect("Failed to convert to int");

        println!(
            "{:?} x:{} y:{}  x_end:{} y_end:{}",
            action, x_start, y_start, x_end, y_end
        );
        parsed_instructions.push(Instruction {
            action,
            x_start,
            y_start,
            x_end,
            y_end,
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
    // lights = lights.iter().map(|x| x.push(vec![0; 1000])).collect();
    for _ in 0..1000 {
        lights.push(vec![0; 1000]);
    }

    // lights = part_1_solution(lights, parsed_instructions);
    lights = part_2_solution(lights, parsed_instructions);

    let total_on: i32 = lights.iter().map(|x| x.iter().sum::<i32>()).sum();

    println!("Total lights on: {}", total_on)
}
