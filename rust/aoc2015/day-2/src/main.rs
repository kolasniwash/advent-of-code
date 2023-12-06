use std::fs;

fn calculate_sides(l: &str, w: &str, h: &str) -> Vec<i32>{
    let l:i32 = l.clone().parse().unwrap();
    let w:i32 = w.clone().parse().unwrap();
    let h:i32 = h.clone().parse().unwrap();
    return vec!(l*w, w*h, h*l);
}

fn main(){
    let input = fs::read_to_string("/Users/nshaw/Code/kolasniwash/advent-of-code/rust/aoc2015/day-2/src/input.txt")
        .expect("File not found");
    let boxes_dims: Vec<&str> = input.split("\n").collect();


    let mut total_wrapping = 0;

    for box_dims in boxes_dims.iter(){
        let dims: Vec<&str> = box_dims.split("x").collect();

        let side_areas:Vec<i32> = calculate_sides(dims[0], dims[1], dims[2]);
        let min_side = side_areas.iter().min().unwrap();

        let box_wrapping: i32 = side_areas.iter().map(|x| 2*x).sum::<i32>() + *min_side;
        total_wrapping += box_wrapping;
    }
    println!("Hello Day 2");
    println!("{:?}", total_wrapping);
}