use std::collections::HashSet;
use crate::common::{DaySpec};

pub const DAY_EIGHTEEN: DaySpec<u32, String> = DaySpec {
    day_num: 18,
    part_1_name: "minimum steps to escape after 1kB",
    part_1,
    part_2_name: "first byte preventing escape",
    part_2,
};

fn part_1(input: &str) -> u32 {
    part_1_sized(input, 71, 71)
}

fn part_1_sized(input: &str, width: usize, length: usize) -> u32 {
    let bytes = parse_input(input);
    let mut corruption_grid = init_corruption_grid(width, length);
    drop_bytes(&mut corruption_grid, &bytes, get_bytes_falling(width, length));
    let mut move_grid = init_move_grid(width, length);
    traverse_grid(&mut move_grid, &corruption_grid, width, length);
    move_grid[length - 1][width - 1]
}

fn part_2(input: &str) -> String {
    part_2_sized(input, 71, 71)
}

fn part_2_sized(input: &str, width: usize, length: usize) -> String {
    let bytes = parse_input(input);
    let mut corruption_grid = init_corruption_grid(width, length);
    for i in 0..bytes.len() {
        drop_byte(&mut corruption_grid, &bytes, i);
        let mut move_grid = init_move_grid(width, length);
        traverse_grid(&mut move_grid, &corruption_grid, width, length);
        if move_grid[length - 1][width - 1] == u32::MAX {
            let obstructing_byte = bytes[i];
            return format!("{},{}", obstructing_byte.col, obstructing_byte.row);
        }
    }
    panic!("escape still possible after all bytes have fallen")
}

fn init_move_grid(width: usize, length: usize) -> Vec<Vec<u32>> {
    let mut move_grid = vec![vec![u32::MAX; width]; length];
    move_grid[0][0] = 0;
    move_grid
}

fn init_corruption_grid(width: usize, length: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; length]
}

fn get_bytes_falling(width: usize, length: usize) -> usize {
    match (width, length) {
        (7, 7) => 12,
        (71, 71) => 1024,
        _ => panic!("unknown grid size"),
    }
}

fn drop_bytes(corruption_grid: &mut [Vec<bool>], bytes: &[Point], num_bytes: usize) {
    for i in 0..num_bytes {
        drop_byte(corruption_grid, bytes, i);
    }
}

fn drop_byte(corruption_grid: &mut [Vec<bool>], bytes: &[Point], i: usize) {
    corruption_grid[bytes[i].row][bytes[i].col] = true;
}

fn traverse_grid(
    move_grid: &mut [Vec<u32>],
    corruption_grid: &[Vec<bool>],
    width: usize,
    length: usize
) {
    let mut path_ends = Vec::from([Point { row: 0, col: 0 }]);
    while !path_ends.is_empty() {
        let mut new_path_ends = Vec::new();
        for &path_end in &path_ends {
            let Point { row: cur_row, col: cur_col } = path_end;
            let surrounding = surrounding(path_end, width, length);
            for &new_path_end in surrounding.iter().flatten() {
                let Point { row: new_row, col: new_col } = new_path_end;
                if !corruption_grid[new_row][new_col] &&
                    move_grid[new_row][new_col] > move_grid[cur_row][cur_col] + 1 {
                    move_grid[new_row][new_col] = move_grid[cur_row][cur_col] + 1;
                    new_path_ends.push(new_path_end);
                }
            }
        }
        path_ends = new_path_ends;
    }
}

fn surrounding(path_end: Point, width: usize, length: usize) -> [Option<Point>; 4] {
    [
        if path_end.row > 0 {
            Some(Point { row: path_end.row - 1, col: path_end.col })
        } else {
            None
        },
        if path_end.row < width - 1 {
            Some(Point { row: path_end.row + 1, col: path_end.col })
        } else {
            None
        },
        if path_end.col > 0 {
            Some(Point { row: path_end.row, col: path_end.col - 1 })
        } else {
            None
        },
        if path_end.col < length - 1 {
            Some(Point { row: path_end.row, col: path_end.col + 1 })
        } else {
            None
        }
    ]
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines()
        .map(|line| {
            let parts = line.split_once(',').unwrap();
            Point {
                col: parts.0.parse().unwrap(),
                row: parts.1.parse().unwrap()
            }
        })
        .collect()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point { row: usize, col: usize }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part_1_sample() {
        assert_eq!(22, part_1_sized(INPUT, 7, 7));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!("6,1", part_2_sized(INPUT, 7, 7));
    }
}