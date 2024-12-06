use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<String> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn is_safe(levels: &[u32]) -> bool {
    levels
        .windows(2)
        .all(|num| num[0] > num[1] && (num[0] - num[1] <= 3))
        || levels
            .windows(2)
            .all(|num| num[0] < num[1] && (num[1] - num[0] <= 3))
}

fn part1(reports: &[String]) -> usize {
    reports
        .into_iter()
        .filter(|report| {
            let levels: Vec<u32> = report
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();

            // Either descending or ascending
            is_safe(&levels)
        })
        .count()
}

fn part2(reports: Vec<String>) -> usize {
    reports
        .into_iter()
        .filter(|report| {
            let levels: Vec<u32> = report
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();

            if !(is_safe(&levels)) {
                for i in 0..levels.len() {
                    let mut levels_vector = levels.to_vec();
                    levels_vector.remove(i);
                    if is_safe(&levels_vector) {
                        return true;
                    }
                }
                false
            } else {
                true
            }
        })
        .count()
}

fn main() {
    let records: Vec<String> = parse_input("src/input");
    println!("{:?}", part1(&records));
    println!("{:?}", part2(records));
}
