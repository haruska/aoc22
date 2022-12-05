use std::error::Error;

type Stack = Vec<char>;

struct Direction {
    stack_num: usize,
    num_crates: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Vec<Stack>, Vec<Direction>) {
    let (stacks_input, _directions_input) = input.split_once("\n\n").unwrap();
    let stack_vals: Vec<Vec<Option<char>>> = stacks_input
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            let vals = line
                .as_bytes()
                .chunks(4)
                .map(|cr| {
                    let c = cr[1] as char;
                    if c == ' ' {
                        None
                    } else {
                        Some(c)
                    }
                })
                .collect();
            vals
        })
        .collect();

    let mut stacks: Vec<Stack> = vec![vec![]; stack_vals[0].len()];
    for sval in stack_vals.iter() {
        for (i, c) in sval.iter().enumerate() {
            if let Some(c) = c {
                stacks[i].push(*c);
            }
        }
    }

    (stacks, vec![])
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

    #[test]
    fn parse_test() {
        let input = include_str!("../input/day05_test.txt");
        let (stacks, _directions) = parse(input);

        assert_eq!(stacks, stacks_fixture());
    }
}
