use std::error::Error;

trait HasScore {
    fn score(&self) -> u32;
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl HasScore for Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl Shape {
    fn new(c: char) -> Option<Self> {
        match c {
            'A' | 'X' => Some(Shape::Rock),
            'B' | 'Y' => Some(Shape::Paper),
            'C' | 'Z' => Some(Shape::Scissors),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Lost,
    Draw,
    Won,
}

impl HasScore for Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Won => 6,
        }
    }
}

impl Outcome {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'X' => Some(Outcome::Lost),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Won),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Round(Shape, Shape);

impl Round {
    fn outcome(&self) -> Outcome {
        if self.0 == self.1 {
            return Outcome::Draw;
        }
        match self.0 {
            Shape::Rock => {
                if self.1 == Shape::Scissors {
                    Outcome::Won
                } else {
                    Outcome::Lost
                }
            }
            Shape::Paper => {
                if self.1 == Shape::Rock {
                    Outcome::Won
                } else {
                    Outcome::Lost
                }
            }
            Shape::Scissors => {
                if self.1 == Shape::Paper {
                    Outcome::Won
                } else {
                    Outcome::Lost
                }
            }
        }
    }
}

impl HasScore for Round {
    fn score(&self) -> u32 {
        self.0.score() + self.outcome().score()
    }
}

fn parse(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();
            let opponent = Shape::new(chars[0]).unwrap();
            let player = Shape::new(chars[2]).unwrap();
            Round(player, opponent)
        })
        .collect()
}

fn alt_parse(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();
            let opponent: Shape = Shape::new(chars[0]).unwrap();
            let desired_outcome: Outcome = Outcome::from_char(chars[2]).unwrap();
            let player = match desired_outcome {
                Outcome::Draw => opponent,
                Outcome::Lost => match opponent {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                },
                Outcome::Won => match opponent {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                },
            };
            Round(player, opponent)
        })
        .collect()
}

fn player_score(rounds: Vec<Round>) -> u32 {
    rounds.iter().map(|r| r.score()).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day02.txt");
    let rounds = parse(input);

    let player_score_one = player_score(rounds);
    println!("Player score (part 1): {}", player_score_one);

    let rounds = alt_parse(input);

    let player_score_two = player_score(rounds);
    println!("Real player score (part 2): {}", player_score_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rounds() {
        let input = include_str!("../input/day02_test.txt");
        let result = parse(input);
        let expected = vec![
            Round(Shape::Paper, Shape::Rock),
            Round(Shape::Rock, Shape::Paper),
            Round(Shape::Scissors, Shape::Scissors),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn player_score_calculates_total() {
        let rounds = vec![
            Round(Shape::Paper, Shape::Rock),
            Round(Shape::Rock, Shape::Paper),
            Round(Shape::Scissors, Shape::Scissors),
        ];
        let result = player_score(rounds);

        assert_eq!(result, 15);
    }

    #[test]
    fn alt_parse_rounds() {
        let input = include_str!("../input/day02_test.txt");
        let result = alt_parse(input);
        let expected = vec![
            Round(Shape::Rock, Shape::Rock),
            Round(Shape::Rock, Shape::Paper),
            Round(Shape::Rock, Shape::Scissors),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn player_score_calculates_total_part_two() {
        let rounds = vec![
            Round(Shape::Rock, Shape::Rock),
            Round(Shape::Rock, Shape::Paper),
            Round(Shape::Rock, Shape::Scissors),
        ];
        let result = player_score(rounds);

        assert_eq!(result, 12);
    }
}
