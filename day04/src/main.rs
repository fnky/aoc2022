use std::{collections::HashSet, io::Read};

fn part1(raw_input: &str) -> usize {
    raw_input
        .lines()
        .filter(|line| {
            let pairs = line.split(',');
            let mut ranges = pairs.map(|elf| {
                let mut range = elf.split('-');
                let start = range.next().unwrap().parse::<u32>().unwrap();
                let end = range.next().unwrap().parse::<u32>().unwrap();
                (start, end)
            });
            let (a, b) = ranges.next().unwrap();
            let (c, d) = ranges.next().unwrap();
            a <= c && b >= d || a >= c && b <= d
        })
        .count()
}

fn part2(raw_input: &str) -> usize {
    raw_input
        .lines()
        .filter(|line| {
            let pairs = line.split(',');
            let mut ranges = pairs.map(|elf| {
                let mut range = elf.split('-');
                let start = range.next().unwrap().parse::<u32>().unwrap();
                let end = range.next().unwrap().parse::<u32>().unwrap();
                (start, end)
            });
            let (a, b) = ranges.next().unwrap();
            let (c, d) = ranges.next().unwrap();
            !(b < c || a > d)
        })
        .count()
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
    fn test_day04_part1() {
        assert_eq!(
            part1(
                r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
            ),
            2
        );
    }

    #[test]
    fn test_day04_part2() {
        assert_eq!(
            part2(
                r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
            ),
            4
        );
    }
}
