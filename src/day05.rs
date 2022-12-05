use recap::Recap;
use serde::Deserialize;
use std::error::Error;

type Stack = Vec<char>;

#[derive(Debug, Deserialize, Recap, PartialEq)]
#[recap(regex = r#"move (?P<num_crates>\d+) from (?P<from>\d+) to (?P<to>\d+)"#)]
struct Direction {
    num_crates: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(input: &str) -> Vec<Stack> {
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

    stacks
}

fn parse_directions(input: &str) -> Result<Vec<Direction>, Box<dyn Error>> {
    let mut res = vec![];
    for line in input.lines() {
        let d: Direction = line.parse()?;
        res.push(d);
    }
    Ok(res)
}

fn parse(input: &str) -> (Vec<Stack>, Vec<Direction>) {
    let (stacks, directions) = input.split_once("\n\n").unwrap();

    (parse_stacks(stacks), parse_directions(directions).unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stacks_fixture() -> Vec<Stack> {
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
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
}
