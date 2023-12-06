use std::fs;

fn floor_up_down(item: char) -> i32 {
    if '(' == item {
        return 1;
    } else if ')' == item {
        return -1;
    } else {
        return 0
    }
}


fn find_final_floor(input: String) -> i32{
    let mut count:i32 = 0;
    for item in input.chars(){
        count += floor_up_down(item);
    }
    count
}

fn find_fist_negative(input: String) -> i32 {
    let mut idx:i32 = 0;
    let mut count:i32 = 0;
    for item in input.chars(){
        count += floor_up_down(item);
        idx += 1;
        if count < 0 {
            return idx;
        }
    }
    return 0
}

fn main() {
    let input = fs::read_to_string("/Users/nshaw/Code/kolasniwash/advent-of-code/rust/aoc2015/day-1/src/input.txt")
        .expect("Failed to read file");

    let count = find_final_floor(input.clone());
    let idx = find_fist_negative(input);

    println!("{ }", count);
    println!("{ }", idx);



}
