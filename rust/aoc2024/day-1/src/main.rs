use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

fn split_number_list(list: String) -> (Vec<i32>, Vec<i32>) {
    let list: Vec<_> = list.split("\n").collect();
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    for item in list.into_iter() {
        let mut two_values = item.split_whitespace();
        let num1 = two_values.next().expect("").parse().expect("");
        let num2 = two_values.next().expect("").parse().expect("");
        v1.push(num1);
        v2.push(num2);
    }
    return (v1, v2);
}

fn solution_part_1(mut left: Vec<i32>, mut right: Vec<i32>) {
    left.sort();
    right.sort();

    let result = calc_distances(left, right);

    println!("Solution part 1: {:?}", result.into_iter().sum::<i32>());
}

fn solution_part_2(left: Vec<i32>, right: Vec<i32>) {
    // first number -> count occurances -> multiply by value -> store number + value in hashmap
    // continue until complete left list of numbers

    let mut total = 0;
    let mut left_num_map: HashMap<i32, i32> = HashMap::new();

    for left_num in left.into_iter() {
        match left_num_map.get(&left_num) {
            Some(value) => total += value,
            None => {
                let count: i32 = right
                    .clone()
                    .into_iter()
                    .filter(|x| x.eq(&left_num))
                    .count() as i32;
                let value = left_num * count;
                left_num_map.insert(left_num, value);
                total += value
            }
        }
    }

    println!("Solution part 2: {:?}", total)
}

fn main() {
    let input = fs::read_to_string(
        "/Users/nshaw/Code/kolasniwash/advent-of-code/rust/aoc2024/day-1/src/input.txt",
    )
    .expect("Failed to read file");

    let (v1, v2) = split_number_list(input);
    solution_part_1(v1.clone(), v2.clone());

    solution_part_2(v1.clone(), v2.clone())
}

fn calc_distances(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let pairs = v1
        .iter()
        .zip(v2.iter())
        .map(|x| x.0 - x.1)
        .map(|x| x.abs())
        .collect();
    pairs
}

#[cfg(test)]
mod tests {
    use crate::calc_distances;
    use std::result;

    #[test]
    fn test_distances() {
        let expected = vec![2, 1, 0, 1, 2, 5];

        let v1 = vec![1, 2, 3, 3, 3, 4];
        let v2 = vec![3, 3, 3, 4, 5, 9];

        let result = calc_distances(v1, v2);

        assert_eq!(result, expected);
    }
}
