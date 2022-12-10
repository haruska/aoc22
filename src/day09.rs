use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
struct Cmd(Direction, usize);
type Point = (i32, i32);

fn parse(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .map(|l| {
            let (d, n) = l.split_once(' ').unwrap();
            let n: usize = n.parse().unwrap();
            let d = match d {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Direction {d} was not R, L, U, or D."),
            };
            Cmd(d, n)
        })
        .collect()
}

fn mv(p: Point, d: &Direction) -> Point {
    let (x, y) = p;

    match d {
        Direction::Right => (x + 1, y),
        Direction::Left => (x - 1, y),
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y - 1),
    }
}

fn mv_tail(head: &Point, tail: Point) -> Point {
    let (x1, y1) = *head;
    let (x2, y2) = tail;

    let delta_x = (x1 - x2).abs();
    let delta_y = (y1 - y2).abs();

    if delta_x <= 1 && delta_y <= 1 {
        //touching
        return tail;
    }

    let mut ret = tail;

    if delta_y != 0 {
        if y1 > y2 {
            //head is higher
            ret = mv(ret, &Direction::Up);
        } else {
            ret = mv(ret, &Direction::Down);
        }
    }

    if delta_x != 0 {
        if x1 > x2 {
            //head is to the right
            ret = mv(ret, &Direction::Right);
        } else {
            ret = mv(ret, &Direction::Left);
        }
    }

    ret
}

fn part_one(cmds: Vec<Cmd>) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert((0, 0));

    let mut head = (0, 0);
    let mut tail = (0, 0);

    for cmd in cmds.into_iter() {
        let Cmd(d, i) = cmd;
        for _ in 0..i {
            head = mv(head, &d);
            tail = mv_tail(&head, tail);
            visited.insert(tail);
        }
    }

    visited.len()
}

fn part_two(cmds: Vec<Cmd>) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert((0, 0));

    let mut knots: Vec<Point> = vec![(0, 0); 10];

    for cmd in cmds.into_iter() {
        let Cmd(dir, times) = cmd;
        for _ in 0..times {
            knots[0] = mv(knots[0], &dir);
            for i in 1..knots.len() {
                knots[i] = mv_tail(&knots[i - 1], knots[i]);
            }
            visited.insert(knots[knots.len() - 1]);
        }
    }

    visited.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day09.txt");
    let cmds = parse(input);

    let p1 = part_one(cmds);
    println!("Total positions the tail visited (Part 1): {p1}");

    let cmds = parse(input);
    let p2 = part_two(cmds);
    println!("Total positions the tail visited (Part 2): {p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_fixture() -> Vec<Cmd> {
        vec![
            Cmd(Direction::Right, 4),
            Cmd(Direction::Up, 4),
            Cmd(Direction::Left, 3),
            Cmd(Direction::Down, 1),
            Cmd(Direction::Right, 4),
            Cmd(Direction::Down, 1),
            Cmd(Direction::Left, 5),
            Cmd(Direction::Right, 2),
        ]
    }

    #[test]
    fn parse_test() {
        let input = include_str!("../input/day09_test.txt");
        let result = parse(input);
        let expected = input_fixture();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_movements() {
        let head = (0, 0);
        let tail = (0, 0);

        assert_eq!(tail, mv_tail(&head, tail));

        let head = mv(head, &Direction::Right);
        assert_eq!(tail, mv_tail(&head, tail));

        let head = mv(head, &Direction::Right);
        let ex_tail = mv(tail, &Direction::Right);
        assert_eq!(ex_tail, mv_tail(&head, tail));
        let tail = ex_tail;

        let head = mv(mv(head, &Direction::Left), &Direction::Up);
        assert_eq!(tail, mv_tail(&head, tail));

        let head = mv(head, &Direction::Up);
        let ex_tail = mv(tail, &Direction::Up);
        assert_eq!(ex_tail, mv_tail(&head, tail));
        let tail = ex_tail;

        //diag
        let head = mv(mv(head, &Direction::Left), &Direction::Up);
        let ex_tail = mv(mv(tail, &Direction::Left), &Direction::Up);
        assert_eq!(ex_tail, mv_tail(&head, tail));
    }

    #[test]
    fn part_one_test() {
        let cmds = input_fixture();
        let result = part_one(cmds);
        assert_eq!(result, 13);
    }
}
