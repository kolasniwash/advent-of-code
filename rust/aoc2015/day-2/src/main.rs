use std::fs;

fn calculate_sides(l: &str, w: &str, h: &str) -> Vec<i32> {
    let l: i32 = l.clone().parse().unwrap();
    let w: i32 = w.clone().parse().unwrap();
    let h: i32 = h.clone().parse().unwrap();
    return vec![l * w, w * h, h * l];
}

fn calculate_side_areas(l: i32, w: i32, h: i32) -> Vec<i32> {
    return vec![l * w, w * h, h * l];
}

fn calculate_ribbon_length(s1: i32, s2: i32, s3: i32) -> i32 {
    return 2 * s1 + 2 * s2 + s1 * s2 * s3;
}

fn main() {
    let input = fs::read_to_string(
        "/Users/nshaw/Code/kolasniwash/advent-of-code/rust/aoc2015/day-2/src/input.txt",
    )
    .expect("File not found");
    let boxes_dims: Vec<&str> = input.split("\n").collect();

    let mut total_wrapping = 0;
    let mut total_ribbon = 0;

    for box_dims in boxes_dims.iter() {
        let dims: Vec<&str> = box_dims.split("x").collect();
        let mut dims_int: Vec<i32> = dims.iter().map(|x| x.parse().unwrap()).collect();
        let side_areas: Vec<i32> = calculate_side_areas(dims_int[0], dims_int[1], dims_int[2]);
        let min_side = side_areas.iter().min().unwrap();
        let box_wrapping: i32 = side_areas.iter().map(|x| 2 * x).sum::<i32>() + *min_side;

        dims_int.sort();
        let ribbon_length: i32 = calculate_ribbon_length(dims_int[0], dims_int[1], dims_int[2]);

        total_wrapping += box_wrapping;
        total_ribbon += ribbon_length;
    }
    println!("Hello Day 2");
    println!("{:?}", total_wrapping);
    println!("{:?}", total_ribbon);
}
