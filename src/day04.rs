use std::error::Error;

type Range = (usize, usize);
type WorkerPair = (Range, Range);

fn parse(input: &str) -> Vec<WorkerPair> {
    input
        .lines()
        .map(|line| {
            let pair: Vec<Range> = line
                .split(',')
                .map(|part| {
                    let xs: Vec<usize> = part
                        .split('-')
                        .map(|s| s.to_string().parse::<usize>().unwrap())
                        .collect();
                    (xs[0], xs[1])
                })
                .collect();
            (pair[0], pair[1])
        })
        .collect()
}

fn fully_contained(wp: WorkerPair) -> bool {
    let ((a, b), (x, y)) = wp;
    (a <= x && b >= y) || (x <= a && y >= b)
}

fn overlap(wp: WorkerPair) -> bool {
    let ((a, b), (x, y)) = wp;
    (a <= y && a >= x) || (b <= y && b >= x) || (x <= b && x >= a) || (y <= b && y >= a)
}

fn contained_count(worker_pairs: &[WorkerPair]) -> usize {
    worker_pairs.iter().fold(
        0,
        |acc, wp| {
            if fully_contained(*wp) {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn overlap_count(worker_pairs: &[WorkerPair]) -> usize {
    worker_pairs
        .iter()
        .fold(0, |acc, wp| if overlap(*wp) { acc + 1 } else { acc })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day04.txt");

    let elf_pairs = parse(input);
    let elf_pairs = elf_pairs.as_slice();

    let part_one = contained_count(elf_pairs);
    println!("Contained count (part 1): {:?}", part_one);

    let part_two = overlap_count(elf_pairs);
    println!("Overlap Count (part 2): {:?}", part_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worker_pairs() -> Vec<WorkerPair> {
        vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ]
    }

    #[test]
    fn parse_test() {
        let input = include_str!("../input/day04_test.txt");
        let result = parse(input);
        let expected = worker_pairs();

        assert_eq!(result, expected);
    }

    #[test]
    fn contained_count_test() {
        let wp = worker_pairs();
        let result = contained_count(wp.as_slice());

        assert_eq!(result, 2);
    }

    #[test]
    fn overlap_count_test() {
        let wp = worker_pairs();
        let result = overlap_count(wp.as_slice());

        assert_eq!(result, 4);
    }
}
