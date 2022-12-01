use std::io::Read;

fn part1(raw_input: &str) -> i32 {
    0
}

fn part2(raw_input: &str) -> i32 {
    0
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
    fn test_day07_part1() {
        assert_eq!(part1("foo"), 0);
    }

    #[test]
    fn test_day07_part2() {
        assert_eq!(part2("bar"), 0);
    }
}
