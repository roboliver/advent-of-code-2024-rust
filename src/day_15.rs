use std::cmp::PartialEq;
use crate::common::DaySpec;

pub const DAY_FIFTEEN: DaySpec<u32, u32> = DaySpec {
    day_num: 15,
    part_1_name: "box coordinates sum",
    part_1,
    part_2_name: "box coordinates sum (big warehouse)",
    part_2,
};

fn part_1(input: &str) -> u32 {
    let (warehouse_lines, moves) = parse_input(input);
    do_part(&mut parse_warehouse(&warehouse_lines), &moves)
}

fn part_2(input: &str) -> u32 {
    let (warehouse_lines, moves) = parse_input(input);
    do_part(&mut parse_warehouse_wide(&warehouse_lines), &moves)
}

fn do_part(warehouse: &mut [Vec<Tile>], moves: &[Move]) -> u32 {
    let mut robot = find_robot(&warehouse);
    for mv in moves {
        robot = do_move(warehouse, robot, *mv);
    }
    box_coordinate_sum(warehouse)
}

fn find_robot(warehouse: &[Vec<Tile>]) -> Point {
    for (row, tiles) in warehouse.iter().enumerate() {
        for (col, tile) in tiles.iter().enumerate() {
            if *tile == Tile::Robot {
                return Point { x: col, y: row };
            }
        }
    }
    panic!("couldn't find the robot!");
}

fn do_move(warehouse: &mut [Vec<Tile>], robot: Point, mv: Move) -> Point {
    let next_robot = increment(robot, mv);
    let next_tile = tile_at(warehouse, next_robot);
    match next_tile {
        Tile::Empty => do_move_empty(warehouse, robot, mv),
        Tile::Box => do_move_box(warehouse, robot, mv),
        Tile::LeftBox | Tile::RightBox => do_move_wide_box(warehouse, robot, mv),
        _ => robot
    }
}

fn do_move_empty(warehouse: &mut [Vec<Tile>], robot: Point, mv: Move) -> Point {
    let next_robot = increment(robot, mv);
    set_tile(warehouse, next_robot, Tile::Robot);
    set_tile(warehouse, robot, Tile::Empty);
    next_robot
}

fn do_move_box(warehouse: &mut [Vec<Tile>], robot: Point, mv: Move) -> Point {
    let next_robot = increment(robot, mv);
    let mut bx = next_robot;
    while tile_at(warehouse, bx) == Tile::Box {
        bx = increment(bx, mv);
    }
    if tile_at(warehouse, bx) == Tile::Empty {
        while bx != next_robot {
            set_tile(warehouse, bx, Tile::Box);
            bx = increment(bx, mv.reverse());
        }
        set_tile(warehouse, next_robot, Tile::Robot);
        set_tile(warehouse, robot, Tile::Empty);
        return next_robot;
    }
    robot
}

fn do_move_wide_box(warehouse: &mut [Vec<Tile>], robot: Point, mv: Move) -> Point {
    match mv {
        Move::Left | Move::Right => do_move_wide_box_horizontal(warehouse, robot, mv),
        Move::Up | Move::Down => do_move_wide_box_vertical(warehouse, robot, mv),
    }
}

fn do_move_wide_box_horizontal(warehouse: &mut [Vec<Tile>], robot: Point, mv: Move) -> Point {
    let next_robot = increment(robot, mv);
    let mut bx = next_robot;
    while let Tile::LeftBox | Tile::RightBox = tile_at(warehouse, bx) {
        bx = increment(bx, mv);
    }
    if tile_at(warehouse, bx) == Tile::Empty {
        while bx != next_robot {
            let prev_bx = increment(bx, mv.reverse());
            set_tile(warehouse, bx, tile_at(warehouse, prev_bx));
            bx = prev_bx;
        }
        set_tile(warehouse, next_robot, Tile::Robot);
        set_tile(warehouse, robot, Tile::Empty);
        return next_robot;
    }
    robot
}

fn do_move_wide_box_vertical(warehouse: &mut [Vec<Tile>], robot: Point, mv: Move) -> Point {
    let next_robot = increment(robot, mv);
    let (bx_left, bx_right) = {
        if tile_at(warehouse, next_robot) == Tile::LeftBox {
            (next_robot, increment(next_robot, Move::Right))
        } else {
            (increment(next_robot, Move::Left), next_robot)
        }
    };
    if can_move_wide_vertical(warehouse, bx_left, bx_right, mv) {
        exec_move_wide_vertical(warehouse, bx_left, bx_right, mv);
        set_tile(warehouse, next_robot, Tile::Robot);
        set_tile(warehouse, robot, Tile::Empty);
        return next_robot;
    }
    robot
}

fn can_move_wide_vertical(
    warehouse: &[Vec<Tile>],
    bx_left: Point,
    bx_right: Point,
    mv: Move
) -> bool {
    let next_bx_left = increment(bx_left, mv);
    let next_tile_left = tile_at(warehouse, next_bx_left);
    let next_bx_right = increment(bx_right, mv);
    let next_tile_right = tile_at(warehouse, next_bx_right);
    match (next_tile_left, next_tile_right) {
        (Tile::Empty, Tile::Empty) => true,
        (_, Tile::Wall) | (Tile::Wall, _) => false,
        (Tile::LeftBox, Tile::RightBox) => can_move_wide_vertical(
            warehouse,
            next_bx_left,
            next_bx_right,
            mv
        ),
        _ => {
            let can_move_left = (next_tile_left == Tile::Empty) ||
                can_move_wide_vertical(
                    warehouse,
                    increment(next_bx_left, Move::Left),
                    next_bx_left,
                    mv
                );
            let can_move_right = (next_tile_right) == Tile::Empty ||
                can_move_wide_vertical(
                    warehouse,
                    next_bx_right,
                    increment(next_bx_right, Move::Right),
                    mv
                );
            can_move_left && can_move_right
        }
    }
}

fn exec_move_wide_vertical(warehouse: &mut [Vec<Tile>], bx_left: Point, bx_right: Point, mv: Move) {
    let next_bx_left = increment(bx_left, mv);
    let next_tile_left = tile_at(warehouse, next_bx_left);
    let next_bx_right = increment(bx_right, mv);
    let next_tile_right = tile_at(warehouse, next_bx_right);
    if next_tile_left == Tile::LeftBox && next_tile_right == Tile::RightBox {
        exec_move_wide_vertical(warehouse, next_bx_left, next_bx_right, mv);
    }
    if next_tile_left == Tile::RightBox {
        exec_move_wide_vertical(warehouse, increment(next_bx_left, Move::Left), next_bx_left, mv);
    }
    if next_tile_right == Tile::LeftBox {
        exec_move_wide_vertical(warehouse, next_bx_right, increment(next_bx_right, Move::Right), mv);
    }
    set_tile(warehouse, next_bx_left, Tile::LeftBox);
    set_tile(warehouse, next_bx_right, Tile::RightBox);
    set_tile(warehouse, bx_left, Tile::Empty);
    set_tile(warehouse, bx_right, Tile::Empty);
}

fn increment(robot: Point, mv: Move) -> Point {
    match mv {
        Move::Up => Point { x: robot.x, y: robot.y - 1 },
        Move::Down => Point { x: robot.x, y: robot.y + 1 },
        Move::Left => Point { x: robot.x - 1, y: robot.y },
        Move::Right => Point { x: robot.x + 1, y: robot.y },
    }
}

fn tile_at(warehouse: &[Vec<Tile>], point: Point) -> Tile {
    warehouse[point.y][point.x]
}

fn set_tile(warehouse: &mut [Vec<Tile>], point: Point, tile: Tile) {
    warehouse[point.y][point.x] = tile;
}

fn box_coordinate_sum(warehouse: &[Vec<Tile>]) -> u32 {
    warehouse.iter().enumerate()
        .flat_map(|(row, tiles)| {
            tiles.iter().enumerate()
                .filter(|(_, tile)| **tile == Tile::Box || **tile == Tile::LeftBox)
                .map(move |(col, _)| ((100 * row) + col) as u32)
        })
        .sum()
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<Move>) {
    let mut lines = input.lines();
    let warehouse_lines: Vec<&str> = lines.by_ref()
        .take_while(|&line| !line.is_empty())
        .collect();
    let moves: Vec<Move> = lines.flat_map(|line| {
        line.chars()
            .map(|c| Move::parse(c))
    })
        .collect();
    (warehouse_lines, moves)
}

fn parse_warehouse(warehouse_lines: &[&str]) -> Vec<Vec<Tile>> {
    warehouse_lines.iter()
        .map(|&line| {
            line.chars()
                .map(Tile::parse)
                .collect()
        })
        .collect()
}

fn parse_warehouse_wide(warehouse_lines: &[&str]) -> Vec<Vec<Tile>> {
    warehouse_lines.iter()
        .map(|&line| {
            line.chars()
                .flat_map(Tile::parse_wide)
                .collect()
        })
        .collect()
}

fn warehouse_str(warehouse: &[Vec<Tile>]) -> String {
    let mut s = String::new();
    for row in warehouse {
        for tile in row {
            s.push({
                match tile {
                    Tile::Box => 'O',
                    Tile::Wall => '#',
                    Tile::Robot => '@',
                    Tile::Empty => '.',
                    Tile::LeftBox => '[',
                    Tile::RightBox => ']',
                }
            });
        }
        s.push('\n');
    }
    s
}

#[derive(Copy, Clone)]
enum Move { Up, Down, Left, Right }

impl Move {
    fn parse(c: char) -> Self {
        match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("unexpected move char: {}", c),
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Move::Up => Move::Down,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
            Move::Right => Move::Left,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile { Box, Wall, Robot, Empty, LeftBox, RightBox }

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            'O' => Tile::Box,
            '#' => Tile::Wall,
            '@' => Tile::Robot,
            '.' => Tile::Empty,
            _ => panic!("unexpected tile char: {}", c),
        }
    }

    fn parse_wide(c: char) -> [Self; 2] {
        match c {
            'O' => [Tile::LeftBox, Tile::RightBox],
            '#' => [Tile::Wall, Tile::Wall],
            '@' => [Tile::Robot, Tile::Empty],
            '.' => [Tile::Empty, Tile::Empty],
            _ => panic!("unexpected double tile char: {}", c),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Point { x: usize, y: usize }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const INPUT_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part_1_sample() {
        assert_eq!(2028, part_1(INPUT_SMALL));
        assert_eq!(10092, part_1(INPUT_LARGE));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(9021, part_2(INPUT_LARGE));
    }
}