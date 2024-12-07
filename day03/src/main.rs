use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<String> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_product(part: &str) -> Option<u32> {
    part.split(')').next().and_then(|param| {
        let numbers: Vec<_> = param.split(',').collect();

        if numbers.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<u32>(), numbers[1].parse::<u32>()) {
                if numbers[0].len() <= 3 && numbers[1].len() <= 3 {
                    return Some(num1 * num2);
                }
            }
        }
        None
    })
}

fn part1(input: &[String]) -> u32 {
    let mut sum = 0;
    for memory in input {
        sum += memory
            .split("mul(")
            .skip(1)
            .filter_map(get_product)
            .sum::<u32>()
    }
    sum
}

fn part2(input: &[String]) -> u32 {
    let mut sum = 0;
    let mut enabled = true;
    for memory in input {
        sum += memory
            .split("mul(")
            .skip(1)
            .filter_map(|part| {
                let response = if enabled { get_product(part) } else { None };

                if part.contains("do()") {
                    enabled = true;
                } else if part.contains("don't()") {
                    enabled = false;
                }

                response
            })
            .sum::<u32>()
    }
    sum
}

fn main() {
    let input = parse_input("src/input");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
