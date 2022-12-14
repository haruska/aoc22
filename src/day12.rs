use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/day12.txt");
    let h_map = parse(input);

    let p1 = part_one(&h_map);
    println!("Part One: {p1}");
}

type Point = (usize, usize);

struct HeightMap {
    map: Vec<Vec<u8>>,
    start: Point,
    finish: Point,
}

impl HeightMap {
    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn on_map(&self, point: Point) -> bool {
        let (x, y) = point;
        x < self.width() && y < self.height()
    }

    fn val_at(&self, point: Point) -> u8 {
        let (x, y) = point;
        self.map[y][x]
    }

    fn neighbor_finish(&self, point: Point) -> bool {
        self.neighbors(&point).contains(&self.finish)
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let (x, y) = *point;

        let mut points = vec![];

        if x < self.width() - 1 {
            points.push((x + 1, y));
        }
        if x > 0 {
            points.push((x - 1, y));
        }
        if y < self.height() - 1 {
            points.push((x, y + 1));
        }
        if y > 0 {
            points.push((x, y - 1));
        }

        points
    }

    fn has_visited(&self, visited: &[Vec<bool>], point: &Point) -> bool {
        let (x, y) = *point;
        if self.on_map(*point) {
            visited[y][x]
        } else {
            true
        }
    }
}

fn possible(h_map: &HeightMap, visited: &[Vec<bool>], point: Point) -> Vec<Point> {
    let points = h_map.neighbors(&point);

    points
        .into_iter()
        .filter(|p| {
            let not_visited = !h_map.has_visited(visited, p);

            let cur_val = h_map.val_at(point);
            let nei_val = h_map.val_at(*p);
            let can_climb = point == h_map.start || nei_val <= cur_val + 1;

            not_visited && can_climb
        })
        .collect()
}

fn part_one(h_map: &HeightMap) -> usize {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; h_map.width()]; h_map.height()];
    let mut steps = 0;

    let mut points = HashSet::new();

    points.insert(h_map.start);
    visited[h_map.start.1][h_map.start.0] = true;

    while !points.is_empty() {
        steps += 1;
        let mut next_points = vec![];
        for p in &points {
            if h_map.neighbor_finish(*p) {
                return steps;
            }

            let possible_points = possible(h_map, &visited, *p);
            for pp in possible_points {
                next_points.push(pp);
            }
        }

        points.clear();
        for p in next_points {
            points.insert(p);
            visited[p.1][p.0] = true;
        }
    }
    steps
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

    #[test]
    fn part_one_test() {
        let h_map = test_map();
        let result = part_one(&h_map);
        assert_eq!(result, 31);
    }
}
