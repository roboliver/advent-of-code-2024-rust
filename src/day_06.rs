use std::collections::HashSet;
use crate::common::DaySpec;

pub const DAY_SIX: DaySpec<usize, usize> = DaySpec { day_num: 6, part_1, part_2 };

pub fn part_1(input: &str) -> usize {
    let (Lab {obstacles, length, width}, mut current) = parse_input(input);
    let mut visited = HashSet::new();
    while in_map(&current.pos, width, length) {
        visited.insert(current.pos);
        current = step_guard(&current, &obstacles, None);
    }
    visited.len()
}

pub fn part_2(input: &str) -> usize {
    let (lab, start) = parse_input(input);
    let route = calculate_route(&lab, &start);
    let Lab { obstacles, width, length } = lab;

    let mut route_traversed = HashSet::new();
    let mut tiles_visited = HashSet::new();
    let mut infinite_loop_positions = HashSet::new();

    for current in route.iter().take(route.len() - 1) {
        route_traversed.insert(current);
        tiles_visited.insert(current.pos);

        let new_obstacle = current.pos.step(current.dir);
        if obstacles.contains(&new_obstacle) || tiles_visited.contains(&new_obstacle) {
            continue;
        }

        let mut diverted_current = current.clone();
        let mut diverted_route_traversed = HashSet::new();

        while in_map(&diverted_current.pos, width, length) {
            diverted_current = step_guard(&diverted_current, &obstacles, Some(new_obstacle));

            if route_traversed.contains(&diverted_current) || diverted_route_traversed.contains(&diverted_current) {
                infinite_loop_positions.insert(new_obstacle);
                break;
            }

            diverted_route_traversed.insert(diverted_current.clone());
        }
    }
    infinite_loop_positions.len()
}

fn calculate_route(lab: &Lab, start: &DirectedPosition) -> Vec<DirectedPosition> {
    let mut current = start.clone();
    let Lab { obstacles, width, length } = lab;
    let mut route = Vec::new();
    while in_map(&current.pos, *width, *length) {
        route.push(current.clone());
        current = step_guard(&current, obstacles, None);
    }
    route
}

fn in_map(point: &Point, width: usize, length: usize) -> bool {
    let width_i = isize::try_from(width).unwrap();
    let length_i = isize::try_from(length).unwrap();
    point.x >= 0 && isize::from(point.x) < length_i &&
        point.y >= 0 && isize::from(point.y) < width_i
}

fn step_guard(
    current: &DirectedPosition,
    obstacles: &HashSet<Point>,
    extra_obstacle: Option<Point>) -> DirectedPosition {
    let next_pos = current.pos.step(current.dir);
    if obstacles.contains(&next_pos) || extra_obstacle.map_or(false, |o| o == next_pos)
    {
        DirectedPosition { pos: current.pos, dir: current.dir.rotate() }
    } else {
        DirectedPosition { pos: next_pos, dir: current.dir }
    }
}

fn parse_input(input: &str) -> (Lab, DirectedPosition) {
    let mut width = 0;
    let mut length = 0;
    let mut obstacles = HashSet::new();
    let mut start: Option<DirectedPosition> = None;
    for (row, line) in input.lines().enumerate() {
        if row == 0 {
            width = line.len();
        }
        for (col, tile) in line.chars().enumerate() {
            if tile == '#' {
                obstacles.insert(Point {
                    x: isize::try_from(col).unwrap(),
                    y: isize::try_from(row).unwrap()
                });
            } else if let Some(dir) = Direction::from_tile(tile) {
                if start.replace(
                    DirectedPosition { pos: Point {
                        x: isize::try_from(col).unwrap(),
                        y: isize::try_from(row).unwrap()
                    }, dir }
                ).is_some() {
                    panic!("multiple guards found");
                }
            }
        }
        length += 1;
    }
    (Lab { obstacles, width, length }, start.unwrap())
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point { x: isize, y: isize }

impl Point {
    fn step(&self, dir: Direction) -> Point {
        match dir {
            Direction::NORTH => Point { x: self.x, y: self.y - 1 },
            Direction::EAST => Point { x: self.x + 1, y: self.y },
            Direction::SOUTH => Point { x: self.x, y: self.y + 1 },
            Direction::WEST => Point { x: self.x - 1, y: self.y },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction { NORTH, EAST, SOUTH, WEST }

impl Direction {
    fn from_tile(tile: char) -> Option<Direction> {
        match tile {
            '^' => Some(Direction::NORTH),
            '>' => Some(Direction::EAST),
            'v' => Some(Direction::SOUTH),
            '<' => Some(Direction::WEST),
            _ => None,
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct DirectedPosition { pos: Point, dir: Direction }

struct Lab {
    obstacles: HashSet<Point>,
    length: usize,
    width: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_1_sample() {
        assert_eq!(41, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(6, part_2(INPUT));
    }
}