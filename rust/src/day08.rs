use std::error::Error;

type Grid = Vec<Vec<u8>>;

#[derive(Debug)]
struct Point(usize, usize);

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| {
                    b - b'0' // convert to ints 0-9
                })
                .collect()
        })
        .collect()
}

fn visible(g: &Grid, p: Point) -> bool {
    let Point(x, y) = p;
    let num_rows = g.len();
    let num_cols = g[0].len();

    if x == 0 || y == 0 || x == num_rows - 1 || y == num_cols - 1 {
        return true;
    }

    let val = g[x][y];

    //look north j == y && i in [..x]
    if (0..x).map(|i| g[i][y]).all(|val2| val > val2) {
        return true;
    }

    //look south j == y && i in [x+1..]
    if (x + 1..num_cols).map(|i| g[i][y]).all(|val2| val > val2) {
        return true;
    }

    //look west i == x && j in [..y]
    if (0..y).map(|j| g[x][j]).all(|val2| val > val2) {
        return true;
    }

    //look east i == x && j in [y+1..]
    if (y + 1..num_rows).map(|j| g[x][j]).all(|val2| val > val2) {
        return true;
    }

    false
}

fn scenic(g: &Grid, p: Point) -> usize {
    let Point(x, y) = p;
    let num_rows = g.len();
    let num_cols = g[0].len();

    if x == 0 || y == 0 || x == num_rows - 1 || y == num_cols - 1 {
        return 0;
    }

    let val = g[x][y];

    //look north j == y && i in [..x]
    let mut north = 0;
    for i in (0..x).rev() {
        let val2 = g[i][y];
        north += 1;
        if val2 >= val {
            break;
        }
    }

    //look south j == y && i in [x+1..]
    let mut south = 0;
    #[allow(clippy::needless_range_loop)]
    for i in x + 1..num_cols {
        let val2 = g[i][y];
        south += 1;
        if val2 >= val {
            break;
        }
    }

    //look west i == x && j in [..y]
    let mut west = 0;
    for j in (0..y).rev() {
        let val2 = g[x][j];
        west += 1;
        if val2 >= val {
            break;
        }
    }

    //look east i == x && j in [y+1..]
    let mut east = 0;
    for j in y + 1..num_rows {
        let val2 = g[x][j];
        east += 1;
        if val2 >= val {
            break;
        }
    }

    north * south * east * west
}

fn part_one(g: &Grid) -> usize {
    let mut n = 0;
    let num_rows = g.len();
    let num_cols = g[0].len();

    for i in 0..num_rows {
        for j in 0..num_cols {
            if visible(g, Point(i, j)) {
                n += 1;
            }
        }
    }
    n
}

fn part_two(g: &Grid) -> usize {
    let mut n = 0;
    let num_rows = g.len();
    let num_cols = g[0].len();

    for i in 0..num_rows {
        for j in 0..num_cols {
            let scenic_score = scenic(g, Point(i, j));
            if scenic_score > n {
                n = scenic_score;
            }
        }
    }

    n
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/day08.txt");
    let grid = parse(input);

    let p1 = part_one(&grid);
    println!("Total visible trees (Part 1): {p1}");

    let p2 = part_two(&grid);
    println!("Top visible score (Part 2): {p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_fixture() -> Vec<Vec<u8>> {
        vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]
    }

    #[test]
    fn parse_test() {
        let input = include_str!("../input/day08_test.txt");
        let result = parse(input);
        let expected = input_fixture();

        assert_eq!(result, expected);
    }

    fn assert_visible(g: &Grid, i: usize, j: usize) {
        assert!(
            visible(&g, Point(i, j)),
            "{:?} was not visible",
            Point(i, j)
        );
    }

    fn assert_not_visible(g: &Grid, i: usize, j: usize) {
        assert!(!visible(&g, Point(i, j)), "{:?} was visible", Point(i, j));
    }

    #[test]
    fn visible_test() {
        let grid = input_fixture();

        //all edges should be visible
        for (i, row) in grid.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if i == 0 || j == 0 || i == grid.len() - 1 || j == row.len() - 1 {
                    assert_visible(&grid, i, j);
                }
            }
        }

        //specific points
        assert_visible(&grid, 1, 1);
        assert_visible(&grid, 1, 2);
        assert_not_visible(&grid, 1, 3);
        assert_visible(&grid, 2, 1);
        assert_not_visible(&grid, 2, 2);
        assert_visible(&grid, 2, 3);
        assert_not_visible(&grid, 3, 1);
        assert_visible(&grid, 3, 2);
        assert_not_visible(&grid, 3, 3);
    }

    #[test]
    fn part_one_test() {
        let grid = input_fixture();

        assert_eq!(part_one(&grid), 21);
    }

    #[test]
    fn scenic_test() {
        let grid = input_fixture();
        let res = scenic(&grid, Point(1, 2));
        assert_eq!(res, 4);
    }

    #[test]
    fn scenic_test_two() {
        let grid = input_fixture();
        let res = scenic(&grid, Point(3, 2));
        assert_eq!(res, 8);
    }

    #[test]
    fn part_two_test() {
        let grid = input_fixture();

        assert_eq!(part_two(&grid), 8);
    }
}
