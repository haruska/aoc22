use std::error::Error;

#[derive(Debug, PartialEq)]
enum Operation {
    Square,
    Add(u32),
    Mult(u32),
}

#[derive(Debug, PartialEq)]
struct Monkey {
    starting_items: Vec<u32>,
    operation: Operation,
    div_test: u32,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    fn inspect_items(&self, items: &[u32]) -> Vec<(usize, u32)> {
        items
            .iter()
            .map(|item| {
                let inspect_val = match self.operation {
                    Operation::Square => item * item,
                    Operation::Add(x) => item + x,
                    Operation::Mult(x) => item * x,
                };
                let bored_val = inspect_val / 3;

                if bored_val % self.div_test == 0 {
                    (self.throw_true, bored_val)
                } else {
                    (self.throw_false, bored_val)
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
            let x: u32 = operation
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

// fn play_round(mut monkeys: Vec<Monkey>) {
//     for i in 0..monkeys.len() {
//         let m = &mut monkeys[i];
//         while let Some(item) = m.items.pop_front() {
//             let inspect_val = match m.operation {
//                 Operation::Square => item * item,
//                 Operation::Add(x) => item + x,
//                 Operation::Mult(x) => item * x,
//             };
//             let bored_val = inspect_val / 3;
//             if bored_val % m.div_test == 0 {
//                 monkeys.get_mut(m.throw_true).unwrap().items.push_back(bored_val);
//             } else {
//                 monkeys[m.throw_false].items.push_back(bored_val);
//             }
//         }
//     }
// }

fn parse(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::from_str_block).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
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
}
