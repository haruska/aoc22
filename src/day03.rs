use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;

#[derive(PartialEq, Debug, Clone)]
struct Rucksack(Vec<char>, Vec<char>);

impl Rucksack {
    fn from_input(s: &str) -> Self {
        let middle = s.len() / 2;
        let chars: Vec<char> = s.chars().collect();
        Self(chars[..middle].to_vec(), chars[middle..].to_vec())
    }

    fn common_item(&self) -> Option<char> {
        for c in &self.0 {
            if self.1.contains(c) {
                return Some(*c);
            }
        }
        None
    }

    fn unique_items(&self) -> HashSet<char> {
        let mut chars = HashSet::new();
        for c in &self.0 {
            chars.insert(*c);
        }
        for c in &self.1 {
            chars.insert(*c);
        }

        chars
    }
}

fn common_item(rucksacks: &[Rucksack]) -> Option<char> {
    // unique char set of each rucksack sorted by size of set
    let char_sets: Vec<HashSet<char>> = rucksacks
        .iter()
        .map(|r| r.unique_items())
        .sorted_by(|a, b| a.len().partial_cmp(&b.len()).unwrap())
        .collect();

    // iterate over smallest set finding the first char existing in the other sets
    char_sets[0]
        .iter()
        .find(|c| char_sets[1..].iter().all(|set| set.contains(c)))
        .copied()
}

fn priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as usize - 'a' as usize + 1 // a-z priority range: 1-26
    } else {
        c as usize - 'A' as usize + 27 // A-Z priority range: 27-52
    }
}

fn part_one(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .iter()
        .map(|r| r.common_item().unwrap())
        .map(priority)
        .sum()
}

fn part_two(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .chunks(3)
        .map(|group| priority(common_item(group).unwrap()))
        .sum()
}

fn parse(input: &str) -> Vec<Rucksack> {
    input.lines().map(Rucksack::from_input).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day03.txt");
    let rucksacks = parse(input);
    let rucksacks = rucksacks.as_slice();

    let sum = part_one(rucksacks);
    println!("Sum of common items priority (part 1): {}", sum);

    let sum = part_two(rucksacks);
    println!("Sum of badges (part 2): {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rucksacks() -> Vec<Rucksack> {
        vec![
            Rucksack(
                "vJrwpWtwJgWr".chars().collect(),
                "hcsFMMfFFhFp".chars().collect(),
            ),
            Rucksack(
                "jqHRNqRjqzjGDLGL".chars().collect(),
                "rsFMfFZSrLrFZsSL".chars().collect(),
            ),
            Rucksack("PmmdzqPrV".chars().collect(), "vPwwTWBwg".chars().collect()),
            Rucksack(
                "wMqvLMZHhHMvwLH".chars().collect(),
                "jbvcjnnSBnvTQFn".chars().collect(),
            ),
            Rucksack("ttgJtRGJ".chars().collect(), "QctTZtZT".chars().collect()),
            Rucksack(
                "CrZsJsPPZsGz".chars().collect(),
                "wwsLwLmpwMDw".chars().collect(),
            ),
        ]
    }

    #[test]
    fn parse_test() {
        let input = include_str!("../input/day03_test.txt");
        let result = parse(input);
        let expected = test_rucksacks();

        assert_eq!(result, expected);
    }

    #[test]
    fn common_item_test() {
        let expected: Vec<char> = "pLPvts".chars().collect();
        let result: Vec<char> = test_rucksacks()
            .iter()
            .map(|r| r.common_item().unwrap())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn priority_test() {
        //16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s)
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('L'), 38);
        assert_eq!(priority('P'), 42);
        assert_eq!(priority('v'), 22);
        assert_eq!(priority('t'), 20);
        assert_eq!(priority('s'), 19);
    }

    #[test]
    fn part_one_test() {
        let rucksacks = test_rucksacks();
        let result = part_one(rucksacks.as_slice());
        assert_eq!(result, 157);
    }

    #[test]
    fn part_two_test() {
        let rucksacks = test_rucksacks();
        let result = part_two(rucksacks.as_slice());
        assert_eq!(result, 70);
    }
}
