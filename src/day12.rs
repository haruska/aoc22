use itertools::Itertools;

fn main() {}

type Point = (usize, usize);

struct HeightMap {
    map: Vec<Vec<u8>>,
    start: Point,
    finish: Point,
}

fn parse(input: &str) -> HeightMap {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].as_bytes().len();

    let mut map = vec![Vec::with_capacity(width); height];
    let mut start = (0, 0);
    let mut finish = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        let bytes = line.as_bytes();
        if let Some((j, _)) = bytes.iter().find_position(|b| **b == b'S') {
            start = (i, j);
        }
        if let Some((j, _)) = bytes.iter().find_position(|b| **b == b'E') {
            finish = (i, j);
        }
        map[i].append(&mut bytes.to_vec());
    }

    HeightMap { map, start, finish }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_map() -> HeightMap {
        let input = include_str!("../input/day12_test.txt");
        parse(input)
    }

    #[test]
    fn parse_test() {
        let h_map = test_map();
        assert_eq!(h_map.start, (0, 0));
        assert_eq!(h_map.finish, (2, 5));
    }
}
