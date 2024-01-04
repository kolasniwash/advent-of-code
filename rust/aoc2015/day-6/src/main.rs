use std::fs;

#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    x_start: i32,
    y_start: i32,
    x_end: i32,
    y_end: i32
}

impl Instruction {
    fn new(action: Action, x_start: i32, y_start: i32, x_end: i32, y_end: i32) -> Instruction {
        Instruction{ action, x_start, y_start, x_end, y_end }
    }
}

fn parse_instruction(instruction: &str) -> Instruction{
    let instruction_parts:Vec<&str> = instruction.split(",").collect();

    let first_part: Vec<&str> = instruction_parts[0].split("turn").map(|x| x.trim()).collect();
    let first_part_split: Vec<&str> = first_part[first_part.len()-1].split(" ").collect();

    let action: Action = match first_part_split[0]{
        "on" => {Action::On},
        "off" => {Action::Off},
        "toggle" => {Action::Toggle},
        _ => {Action::On}
    };

    let x_start: i32 = first_part_split[1].parse().expect("Failed to convert to int");

    let second_part: Vec<&str> = instruction_parts[1].split(" ").collect();

    let y_start: i32 = second_part[0].parse().expect("Failed to convert to int");
    let x_end: i32 = second_part[second_part.len()-1].parse().expect("Failed to convert to int");

    let y_end:i32 = instruction_parts[instruction_parts.len()-1].parse().expect("Failed to convert to int");

    println!("{:?} x:{} y:{}  x_end:{} y_end:{}", action, x_start, y_start, x_end, y_end);
    Instruction{
        action,
        x_start,
        y_start,
        x_end,
        y_end
    }

}

fn main(){
    let instructions = fs::read_to_string("/Users/nshaw/Code/kolasniwash/advent-of-code/rust/aoc2015/day-6/src/input.txt")
        .expect("File not found");

    let mut lights: Vec<Vec<i32>> = Vec::with_capacity(1000);
    for _ in 0..1000 {
        lights.push(vec![0; 1000]);
    }
    // println!("{:?}", lights);

    for instruction in instructions.split("\n") {
        println!("{}", instruction);
        let parsed_instruction: Instruction = parse_instruction(instruction);

        for row in parsed_instruction.x_start..parsed_instruction.x_end+1 {
            for col in parsed_instruction.y_start..parsed_instruction.y_end+1{
                match parsed_instruction.action {
                    Action::On => { lights[row as usize][col as usize] = 1}
                    Action::Off => { lights[row as usize][col as usize] = 0}
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
    let mut total_on = 0;
    for row in 0..999{
        for col in 0..999{
            total_on += lights[row][col]
        }
    }
    // let total_on: i32 = lights.into_iter().map(|row| row.into_iter().sum::<u32>().collect()).sum();
    println!("Total lights on: {}", total_on)
}