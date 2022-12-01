fn parse(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .fold(0, |a, s2| a + s2.to_string().parse().unwrap_or(0))
        })
        .collect()
}

fn top_elf(elves: &[usize]) -> usize {
    elves.iter().max().copied().unwrap_or(0)
}

fn top_elves(mut elves: Vec<usize>, count: usize) -> usize {
    elves.sort();
    let top = &elves[elves.len() - count..];
    top.iter().sum()
}

fn main() {
    let input = include_str!("../input/day01.txt");
    let elves = parse(input);

    let top_elf = top_elf(elves.as_slice());
    println!("Top elf (part 1): {}", top_elf);

    let top_elves = top_elves(elves, 3);
    println!("Sum top elves (part 2): {}", top_elves);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_elf_sums() {
        let input = include_str!("../input/day01_test.txt");
        let result = parse(input);
        assert_eq!(result, [6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn find_max_elf() {
        let result = top_elf(&[6000, 4000, 11000, 24000, 10000]);
        assert_eq!(result, 24000);
    }

    #[test]
    fn find_top_sum() {
        let result = top_elves(vec![6000, 4000, 11000, 24000, 10000], 3);
        assert_eq!(result, 45000);
    }
}
