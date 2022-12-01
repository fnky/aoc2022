use std::io::Read;

// Input: Number of calories

fn part1(raw_input: &str) -> i32 {
    raw_input
        .split("\n\n")
        .map(|item| {
            item.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .max()
        .expect("No max")
}

fn part2(raw_input: &str) -> i32 {
    let mut sums = raw_input
        .split("\n\n")
        .map(|item| {
            item.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    sums.sort();
    sums.reverse();
    sums[..3].iter().sum::<i32>()
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01_part1() {
        let input = r#"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
        "#;
        assert_eq!(part1(input), 24000);
    }

    #[test]
    fn test_day01_part2() {
        let input = r#"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
        "#;
        assert_eq!(part2(input), 45000);
    }
}
