use std::fs::read_to_string;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| {
            let location_id = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (location_id[0], location_id[1])
        })
        .unzip()
}

fn part1(left_list: &[i32], right_list: &[i32]) -> i32 {
    left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| (left - right).abs())
        .sum()
}

fn part2(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    left_list
        .into_iter()
        .map(|left| {
            let count = right_list
                .clone()
                .into_iter()
                .filter(|right| *right == left)
                .count();
            left * i32::try_from(count).unwrap()
        })
        .sum()
}

fn main() {
    let (mut left_list, mut right_list) = parse_input("./src/input");

    left_list.sort();
    right_list.sort();

    println!("{}", part1(&left_list, &right_list));
    println!("{}", part2(left_list, right_list));
}
