use itertools::Itertools;
use std::error::Error;

fn unique_marker(bytes: &[u8], n: usize) -> usize {
    for (i, window) in bytes.windows(n).enumerate() {
        if window.iter().all_unique() {
            return i + n;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day06.txt");

    let p1_result = unique_marker(input.as_bytes(), 4);
    println!("Index of uniq stream (part 1): {}", p1_result);

    let p2_result = unique_marker(input.as_bytes(), 14);
    println!("Index of uniq message (part 2): {}", p2_result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        assert_eq!(
            unique_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), 4),
            7
        );
        assert_eq!(
            unique_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 4),
            5
        );
        assert_eq!(
            unique_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 4),
            10
        );
        assert_eq!(
            unique_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), 4),
            11
        );
    }

    #[test]
    fn part_two_test() {
        assert_eq!(
            unique_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), 14),
            19
        );
        assert_eq!(
            unique_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 14),
            23
        );
        assert_eq!(
            unique_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 14),
            29
        );
    }
}
