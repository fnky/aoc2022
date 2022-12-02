use std::io::Read;

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn beats(&self, other: &Shape) -> bool {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => true,
            (Shape::Paper, Shape::Rock) => true,
            (Shape::Scissors, Shape::Paper) => true,
            _ => false,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl std::str::FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum RoundOutcome {
    Win,
    Loss,
    Draw,
}

impl RoundOutcome {
    fn score(&self) -> i32 {
        match self {
            RoundOutcome::Loss => 0,
            RoundOutcome::Draw => 3,
            RoundOutcome::Win => 6,
        }
    }
}

impl std::str::FromStr for RoundOutcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundOutcome::Loss),
            "Y" => Ok(RoundOutcome::Draw),
            "Z" => Ok(RoundOutcome::Win),
            _ => Err(()),
        }
    }
}

fn part1(raw_input: &str) -> i32 {
    let outcomes = raw_input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let opponent_move = parts.next().unwrap().parse::<Shape>().unwrap();
        let my_move = parts.next().unwrap().parse::<Shape>().unwrap();

        let score = if opponent_move.beats(&my_move) {
            my_move.score() + 0
        } else if my_move.beats(&opponent_move) {
            my_move.score() + 6
        } else {
            my_move.score() + 3
        };

        score
    });

    outcomes.sum::<i32>()
}

// Round Outcome: [X, Y, Z] === [0, 3, 6]
// Move [A, B, C] === [1, 2, 3]

fn part2(raw_input: &str) -> i32 {
    let outcomes = raw_input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let opponent_move = parts.next().unwrap().parse::<Shape>().unwrap();
        let round_outcome = parts.next().unwrap().parse::<RoundOutcome>().unwrap();

        let my_move = match round_outcome {
            RoundOutcome::Win => match opponent_move {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            RoundOutcome::Draw => opponent_move.clone(),
            RoundOutcome::Loss => match opponent_move {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            _ => panic!("Invalid round outcome"),
        };

        my_move.score() + round_outcome.score()
    });

    outcomes.sum::<i32>()
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
    fn test_day02_part1() {
        assert_eq!(
            part1(
                r#"A Y
B X
C Z"#
            ),
            15
        );
    }

    #[test]
    fn test_day02_part2() {
        assert_eq!(
            part2(
                r#"A Y
        B X
        C Z"#
            ),
            12
        );
    }
}
