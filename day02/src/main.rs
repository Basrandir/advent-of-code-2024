use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(levels: &[u32]) -> bool {
    // Either descending OR ascending
    levels
        .windows(2)
        .all(|num| num[0] > num[1] && (num[0] - num[1] <= 3))
        || levels
            .windows(2)
            .all(|num| num[0] < num[1] && (num[1] - num[0] <= 3))
}

fn is_safe_with_dampner(levels: &[u32]) -> bool {
    if is_safe(&levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut levels_vector = levels.to_vec();
        levels_vector.remove(i);
        if is_safe(&levels_vector) {
            return true;
        }
    }

    false
}

fn part1(reports: &[Vec<u32>]) -> usize {
    reports.into_iter().filter(|report| is_safe(report)).count()
}

fn part2(reports: &[Vec<u32>]) -> usize {
    reports
        .into_iter()
        .filter(|report| is_safe_with_dampner(report))
        .count()
}

fn main() {
    let records: Vec<Vec<u32>> = parse_input("src/input");
    println!("{:?}", part1(&records));
    println!("{:?}", part2(&records));
}
