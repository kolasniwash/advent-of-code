use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Location{
    x: i32,
    y: i32
}

impl Location {
    fn new(x: i32, y: i32) -> Location {
        Location{ x, y }
    }
}

fn update_location(loc: Location, step: char) -> Location {
    let x = loc.x;
    let y = loc.y;
    if step == '^' { Location { x, y: y + 1 } }
    else if step == 'v' { Location{ x, y: y - 1 } }
    else if step == '>' { Location{x: x + 1, y } }
    else if step == '<' { Location{ x: x - 1, y } }
    else { Location { x, y } }
}


fn main(){
    let input = fs::read_to_string("/Users/nshaw/Code/kolasniwash/advent-of-code/rust/aoc2015/day-3/src/input.txt")
        .expect("File not found");
    let steps:Vec<char> = input.chars().collect();

    let num_santas:u32 = 1;
    let start_loc = Location::new(0,0);
    let mut current_santa_loc:HashMap<u32, Location> = HashMap::new();
    let mut unique_santa_locations:HashMap<Location, u32> = HashMap::new();

    for santa_num in 0..num_santas {
        current_santa_loc.insert(santa_num, start_loc);
    }

    let mut index = 0;

    while index < steps.len() {
        for santa in 0..num_santas{

            let step:char = steps[index];
            match current_santa_loc.get(&santa){
                Some(&loc) => {
                    let new_loc = update_location(loc, step);
                    current_santa_loc.insert(santa, new_loc);
                    unique_santa_locations.insert(new_loc, 1);
                },
                _ => println!("Santa value out of range.")
            }

            index += 1;
        }
    }

    println!("Number of unique santa locations: {:?}", unique_santa_locations.len())


}