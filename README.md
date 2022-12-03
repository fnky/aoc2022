# Advent of Code 2022

My solutions of the [Advent of Code 2022](https://adventofcode.com/2022) challenges in Rust.

## Completed challenges

- [`Day 01`](./day01) - ⭐⭐
- [`Day 02`](./day02) - ⭐⭐
- [`Day 03`](./day03) - ⭐⭐
- [`Day 04`](./day04)
- [`Day 05`](./day05)
- [`Day 06`](./day06)
- [`Day 07`](./day07)
- [`Day 08`](./day08)
- [`Day 09`](./day09)
- [`Day 10`](./day10)
- [`Day 10`](./day10)
- [`Day 11`](./day11)
- [`Day 12`](./day12)
- [`Day 13`](./day13)
- [`Day 14`](./day14)
- [`Day 15`](./day15)
- [`Day 16`](./day16)
- [`Day 17`](./day17)
- [`Day 18`](./day18)
- [`Day 19`](./day19)
- [`Day 20`](./day20)
- [`Day 21`](./day21)
- [`Day 22`](./day22)
- [`Day 23`](./day23)
- [`Day 24`](./day24)
- [`Day 25`](./day25)

## Running solutions

Run solutions with Cargo and pass input through stdin

```sh
# Stdin (end with ^D)
cargo run --bin day01

# Manual input
echo "input" | cargo run --bin day01

# File input
cat input.txt | cargo run --bin day01
cargo run --bin day01 < input.txt
```

You can also run tests for all or partial solutions:

```sh
# Run test for a specific parts of a specific day
cargo test test_day01_part1

# Run tests for all parts of a specific day
cargo test test_day01

# Run tests for all parts of all days
cargo test
```
