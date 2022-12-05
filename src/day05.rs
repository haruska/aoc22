use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;
use std::error::Error;

type Stack = Vec<char>;

#[derive(PartialEq, Debug)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    fn execute(&mut self, d: Direction) {
        for _ in times(d.num_crates) {
            let c = self.stacks[d.from - 1].pop().unwrap();
            self.stacks[d.to - 1].push(c);
        }
    }

    fn peek_all(&self) -> String {
        self.stacks.iter().map(|s| s.last().unwrap()).join("")
    }
}

#[derive(Debug, Deserialize, Recap, PartialEq)]
#[recap(regex = r#"move (?P<num_crates>\d+) from (?P<from>\d+) to (?P<to>\d+)"#)]
struct Direction {
    num_crates: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(input: &str) -> Stacks {
    let mut lines = input.lines().rev();

    // the first line is a header with the stack numbers. use it to count the number of stacks.
    let num_stacks = lines.next().unwrap().as_bytes().chunks(4).count();
    let mut stacks: Vec<Stack> = vec![vec![]; num_stacks];

    for line in lines {
        for (i, cr) in line.as_bytes().chunks(4).enumerate() {
            let c = cr[1] as char;
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    Stacks { stacks }
}

fn parse_directions(input: &str) -> Result<Vec<Direction>, Box<dyn Error>> {
    let mut res = vec![];
    for line in input.lines() {
        let d: Direction = line.parse()?;
        res.push(d);
    }
    Ok(res)
}

fn parse(input: &str) -> (Stacks, Vec<Direction>) {
    let (stacks, directions) = input.split_once("\n\n").unwrap();

    (parse_stacks(stacks), parse_directions(directions).unwrap())
}

fn part_one(stacks: Stacks, directions: Vec<Direction>) -> String {
    let mut stacks = stacks;

    for d in directions {
        stacks.execute(d);
    }
    stacks.peek_all()
}

fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day05.txt");
    let (stacks, directions) = parse(input);

    let part_one = part_one(stacks, directions);
    println!("Top of stacks (part 1): {}", part_one);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stacks_fixture() -> Stacks {
        Stacks {
            stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        }
    }

    fn directions_fixture() -> Vec<Direction> {
        vec![
            Direction {
                num_crates: 1,
                from: 2,
                to: 1,
            },
            Direction {
                num_crates: 3,
                from: 1,
                to: 3,
            },
            Direction {
                num_crates: 2,
                from: 2,
                to: 1,
            },
            Direction {
                num_crates: 1,
                from: 1,
                to: 2,
            },
        ]
    }

    #[test]
    fn parse_test() {
        let input = include_str!("../input/day05_test.txt");
        let (stacks, directions) = parse(input);

        assert_eq!(stacks, stacks_fixture());
        assert_eq!(directions, directions_fixture());
    }

    #[test]
    fn part_one_test() {
        let stacks = stacks_fixture();
        let directions = directions_fixture();

        let result = part_one(stacks, directions);

        assert_eq!(result, "CMZ")
    }
}
