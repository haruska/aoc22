use std::error;
use std::num::ParseIntError;

fn parse(input: &str) -> Result<Vec<usize>, ParseIntError> {
    let mut elves: Vec<usize> = vec![];

    let mut sum = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            let x: usize = line.to_string().parse()?;
            sum += x;
        }
    }
    if sum != 0 {
        elves.push(sum);
    }

    Ok(elves)
}

fn top_elf(elves: &[usize]) -> Option<usize> {
    elves.iter().max().copied()
}

fn top_elves(mut elves: Vec<usize>, count: usize) -> usize {
    elves.sort();
    let top = &elves[elves.len() - count..];
    top.iter().sum()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = include_str!("../input/day01.txt");
    let elves = parse(input)?;

    let top_elf = top_elf(elves.as_slice()).unwrap();
    println!("Top elf (part 1): {}", top_elf);

    let top_elves = top_elves(elves, 3);
    println!("Sum top elves (part 2): {}", top_elves);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_elf_sums() {
        let input = include_str!("../input/day01_test.txt");
        let result = parse(input).unwrap();
        assert_eq!(result, [6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn find_max_elf() {
        let result = top_elf(&[6000, 4000, 11000, 24000, 10000]).unwrap();
        assert_eq!(result, 24000);
    }

    #[test]
    fn find_top_sum() {
        let result = top_elves(vec![6000, 4000, 11000, 24000, 10000], 3);
        assert_eq!(result, 45000);
    }
}
