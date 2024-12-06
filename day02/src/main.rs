use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<String> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn part1(reports: Vec<String>) -> usize {
    reports
        .into_iter()
        .filter(|report| {
            let levels: Vec<u32> = report
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();

            // Either descending or ascending
            levels
                .windows(2)
                .all(|num| num[0] > num[1] && (num[0] - num[1] <= 3))
                || levels
                    .windows(2)
                    .all(|num| num[0] < num[1] && (num[1] - num[0] <= 3))
        })
        .count()
}

fn main() {
    let records = parse_input("src/input");
    println!("{:?}", part1(records));
}
