use itertools::Itertools;
use std::collections::HashSet;

type Point = (usize, usize);

trait Map {
    type Item;

    fn map(&self) -> &Vec<Vec<Self::Item>>;

    fn height(&self) -> usize {
        self.map().len()
    }

    fn width(&self) -> usize {
        self.map()[0].len()
    }

    fn on_map(&self, point: &Point) -> bool {
        let (i, j) = *point;
        i < self.height() && j < self.width()
    }

    fn val_at(&self, point: &Point) -> &Self::Item {
        let (i, j) = *point;
        &self.map()[i][j]
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let (i, j) = *point;

        let mut points = vec![];

        if i < self.height() - 1 {
            points.push((i + 1, j));
        }
        if i > 0 {
            points.push((i - 1, j));
        }
        if j < self.width() - 1 {
            points.push((i, j + 1));
        }
        if j > 0 {
            points.push((i, j - 1));
        }

        points
    }
}

struct HeightMap {
    map: Vec<Vec<u8>>,
    start: Point,
    finish: Point,
}

impl Map for HeightMap {
    type Item = u8;

    fn map(&self) -> &Vec<Vec<Self::Item>> {
        &self.map
    }

    fn val_at(&self, point: &Point) -> &Self::Item {
        if *point == self.start {
            return &b'a';
        }
        if *point == self.finish {
            return &b'z';
        }

        let (i, j) = *point;
        &self.map()[i][j]
    }
}

struct VisitedMap {
    map: Vec<Vec<bool>>,
}

impl Map for VisitedMap {
    type Item = bool;

    fn map(&self) -> &Vec<Vec<Self::Item>> {
        &self.map
    }
}

impl VisitedMap {
    fn new(height: usize, width: usize) -> Self {
        Self {
            map: vec![vec![false; width]; height],
        }
    }

    fn visit(&mut self, point: &Point) {
        let (i, j) = *point;
        self.map[i][j] = true;
    }

    fn has_visited(&self, point: &Point) -> bool {
        let (i, j) = *point;
        if self.on_map(point) {
            self.map[i][j]
        } else {
            true
        }
    }
}

fn possible(h_map: &HeightMap, visited: &VisitedMap, point: &Point) -> Vec<Point> {
    let neighbors = h_map.neighbors(point);
    let point_val = h_map.val_at(point);

    neighbors
        .into_iter()
        .filter(|neighbor| {
            let has_visited = visited.has_visited(neighbor);

            let neighbor_val = h_map.val_at(neighbor);
            let can_climb = *neighbor_val <= *point_val + 1;

            !has_visited && can_climb
        })
        .collect()
}

fn part_one(h_map: &HeightMap) -> usize {
    let mut visited = VisitedMap::new(h_map.height(), h_map.width());
    let mut steps = 0;

    let mut points = HashSet::new();

    points.insert(h_map.start);

    while !points.is_empty() {
        let mut next_points = HashSet::new();
        for p in &points {
            visited.visit(p);

            if h_map.finish == *p {
                return steps;
            }

            let possible_points = possible(h_map, &visited, p);
            for pp in possible_points {
                next_points.insert(pp);
            }
        }

        points.clear();

        for p in next_points {
            points.insert(p);
        }

        steps += 1;
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

fn main() {
    let input = include_str!("../input/day12.txt");
    let h_map = parse(input);

    let p1 = part_one(&h_map);
    println!("Part One: {p1}");
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
