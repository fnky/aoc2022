use std::{io::Read, iter::zip};

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => (c as u8 - 'a' as u8) as i32 + 1,
        'A'..='Z' => (c as u8 - 'A' as u8) as i32 + 27,
        _ => 0,
    }
}

fn part1(raw_input: &str) -> i32 {
    raw_input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            left.chars()
                .find(|&c| right.contains(c))
                .map_or(0, priority)
        })
        .sum::<i32>()
}

fn part2(raw_input: &str) -> i32 {
    let lines = raw_input.lines().collect::<Vec<_>>();
    lines
        .chunks(3)
        .flat_map(|chunks| {
            chunks
                .iter()
                .map(|&line| {
                    line.chars()
                        .find(|&c| chunks.iter().all(|&other| other.contains(c)))
                        .map_or(0, priority)
                })
                .take(1)
        })
        .sum::<i32>()
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
    fn test_day03_part1() {
        assert_eq!(
            part1(
                r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            ),
            157
        );
    }

    #[test]
    fn test_day03_part2() {
        assert_eq!(
            part2(
                r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            ),
            70
        );
    }
}
