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

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day04.txt");
    let elf_pairs = parse(input);
    let contained_count: usize =
        elf_pairs.iter().fold(
            0,
            |acc, wp| {
                if fully_contained(*wp) {
                    acc + 1
                } else {
                    acc
                }
            },
        );

    let elf_pairs = parse(input);
    let overlap_count: usize = elf_pairs
        .iter()
        .fold(0, |acc, wp| if overlap(*wp) { acc + 1 } else { acc });
    println!("{:?}, {:?}", contained_count, overlap_count);
    Ok(())
}
