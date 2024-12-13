use std::cmp::PartialEq;
use std::fs;
use std::mem::drop;
use itertools::Itertools;

fn split_lines_to_nested_vec(lines: String) -> Vec<Vec<i32>>{

    let num_lines: Vec<&str> = lines.split("\n").collect();
    let mut outer: Vec<Vec<i32>> = Vec::new();
    for numbers in num_lines.into_iter(){
        let mut inner: Vec<i32> = Vec::new();
        for number in numbers.split_whitespace(){
            if let Ok(number) = number.parse() {
                inner.push(number);
            }
        }
        outer.push(inner);
    }
    outer
}


#[derive(Eq, PartialEq, PartialOrd)]
enum ReportSafety {
    IsSafe,
    IsUnsafe
}

enum MeasureMovement{
    Up,
    Down,
    Invalid
}



fn read_meter_movement(first: &i32, second: &i32) -> i32{
    let diff = second - first;
    if diff.eq(&0) {
        return 0;
    } else if diff.le(&3) & diff.ge(&0) {
        return 3;
    } else if diff.ge(& -3) & diff.le(&0){
        return -3;
    } else {
        return 0;
    }
}

fn read_all_meter_readings(elements: &Vec<i32>) -> Vec<i32>{
    let mut readings: Vec<i32> = Vec::new();
    for (first, second) in elements.clone().into_iter().tuple_windows(){
        let reading = read_meter_movement(&first, &second);
        readings.push(reading);
    }
    readings
}

fn report_checker(elements: &Vec<i32>) -> ReportSafety{

    let readings = read_all_meter_readings(&elements);
    drop(elements);

    let all_des = readings.iter().all(|x| x.eq(&-3));
    let all_asc = readings.iter().all(|x| x.eq(&3));

    match all_des || all_asc{
        true => {
            // println!("elements: {:?}, readings: {:?}, safe", elements, readings);
            return ReportSafety::IsSafe;
        }
        false => {
            // println!("elements: {:?}, readings: {:?}, unsafe", elements, readings);
            return ReportSafety::IsUnsafe;
        }
    }
}

fn problem_damper(elements: &Vec<i32>) -> ReportSafety{
    println!("elements: {:?}", elements);

    let array_len = elements.len();
    let mut dampend_results = Vec::new();
    for i in 0..array_len{
        let mut sub_elements = elements.clone();
        sub_elements.remove(i);
        let safety = report_checker(&sub_elements);

        dampend_results.push(safety);
    }

    let dampened_count = dampend_results.into_iter().filter(|x| x == &ReportSafety::IsSafe).count();
    println!("Damp count {:?}", dampened_count);
    if dampened_count == 0 {
        return ReportSafety::IsUnsafe;
    };
    ReportSafety::IsSafe

}

fn solution_part_1(_lines: Vec<Vec<i32>>){
    let mut count = 0;
    for item in _lines.into_iter(){
        match report_checker(&item) {
            ReportSafety::IsSafe => {count +=1 }
            ReportSafety::IsUnsafe => {}
        }
    }

    println!("Solution part 1: {:?}", count)
}

fn solution_part_2(_lines: Vec<Vec<i32>>){
    println!("Total Lines Input: {:?}", _lines.len());
    let mut count = 0;
    for item in _lines.into_iter(){
        match report_checker(&item) {
            ReportSafety::IsSafe => {count +=1 }
            ReportSafety::IsUnsafe => {
                match problem_damper(&item){
                    ReportSafety::IsSafe => { count +=1 }
                    ReportSafety::IsUnsafe => {}
                }
            }
        }
    }

    println!("Solution part 2: {:?}", count)
}

fn main() {
    let input = fs::read_to_string(
        "/Users/nshaw/Code/kolasniwash/advent-of-code/rust/aoc2024/day-2/src/input.txt",
    )
        .expect("Failed to read file");

    let lines = split_lines_to_nested_vec(input);

    // solution_part_1(lines.clone());

    solution_part_2(lines.clone());
}