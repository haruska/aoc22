use std::collections::VecDeque;
use std::error::Error;

#[derive(Debug, PartialEq)]
enum Operation {
    Square,
    Add(u64),
    Mult(u64),
}

#[derive(Debug, PartialEq)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    div_test: u64,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    fn inspect_items(
        &self,
        items: &[u64],
        divisor: Option<u64>,
        modulo: Option<u64>,
    ) -> Vec<(usize, u64)> {
        items
            .iter()
            .map(|item| {
                let mut inspect_val = match self.operation {
                    Operation::Square => item * item,
                    Operation::Add(x) => item + x,
                    Operation::Mult(x) => item * x,
                };

                if let Some(divisor) = divisor {
                    inspect_val /= divisor;
                }

                let mut throw_val = inspect_val;
                if let Some(modulo) = modulo {
                    throw_val %= modulo;
                }

                if inspect_val % self.div_test == 0 {
                    (self.throw_true, throw_val)
                } else {
                    (self.throw_false, throw_val)
                }
            })
            .collect()
    }

    fn from_str_block(input: &str) -> Monkey {
        let lines: Vec<&str> = input.lines().collect();

        let (_, starting_items) = lines[1].split_once(": ").unwrap();
        let starting_items = starting_items
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();

        let (_, operation) = lines[2].split_once("= ").unwrap();
        let operation = if operation == "old * old" {
            Operation::Square
        } else if operation.starts_with("old *") {
            let x: u64 = operation
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            Operation::Mult(x)
        } else {
            let x = operation
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            Operation::Add(x)
        };

        let (_, test) = lines[3].split_once("by ").unwrap();
        let div_test = test.parse().unwrap();

        let (_, if_true) = lines[4].split_once("monkey ").unwrap();
        let throw_true = if_true.parse().unwrap();

        let (_, if_false) = lines[5].split_once("monkey ").unwrap();
        let throw_false = if_false.parse().unwrap();

        Monkey {
            starting_items,
            operation,
            div_test,
            throw_true,
            throw_false,
        }
    }
}

fn part_two(monkeys: &[Monkey]) -> usize {
    let m = monkeys.iter().fold(1, |acc, m| acc * m.div_test);
    process(monkeys, 10000, None, Some(m))
}

fn part_one(monkeys: &[Monkey]) -> usize {
    process(monkeys, 20, Some(3), None)
}

fn process(monkeys: &[Monkey], rounds: usize, divisor: Option<u64>, modulo: Option<u64>) -> usize {
    let mut counts: Vec<usize> = vec![0; monkeys.len()];

    let mut queues: Vec<VecDeque<u64>> = vec![VecDeque::new(); monkeys.len()];
    for (i, m) in monkeys.iter().enumerate() {
        for item in m.starting_items.iter() {
            queues[i].push_back(*item);
        }
    }

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let q: Vec<u64> = queues[i].drain(..).collect();
            let q = &q[..];

            counts[i] += q.len();

            let changes = monkeys[i].inspect_items(q, divisor, modulo);
            for (m_index, val) in changes {
                queues[m_index].push_back(val);
            }
        }
    }

    counts.sort();
    let x = counts[monkeys.len() - 1];
    let y = counts[monkeys.len() - 2];

    x * y
}

fn parse(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::from_str_block).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day11.txt");
    let monkeys = parse(input);
    let monkeys = monkeys.as_slice();

    let p1 = part_one(monkeys);
    println!("Part One: {p1}");

    let p2 = part_two(monkeys);
    println!("Part Two: {p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monkey_from_input_test() {
        let input = r#"Monkey 0:
            Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
        "#;
        let monkey = Monkey::from_str_block(input);

        assert_eq!(monkey.starting_items, vec![79, 98]);
        assert_eq!(monkey.operation, Operation::Mult(19));
        assert_eq!(monkey.div_test, 23);
        assert_eq!(monkey.throw_true, 2);
        assert_eq!(monkey.throw_false, 3);
    }

    #[test]
    fn part_one_test() {
        let input = include_str!("../input/day11_test.txt");
        let monkeys = parse(input);
        let result = part_one(monkeys.as_slice());

        assert_eq!(result, 10605);
    }

    #[test]
    fn part_two_test() {
        let input = include_str!("../input/day11_test.txt");
        let monkeys = parse(input);
        let result = part_two(monkeys.as_slice());

        assert_eq!(result, 2713310158);
    }
}
