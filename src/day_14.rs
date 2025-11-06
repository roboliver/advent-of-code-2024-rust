use std::collections::HashMap;
use crate::common::{DaySpec, Point};

pub const DAY_FOURTEEN: DaySpec<usize, usize> = DaySpec {
    day_num: 14,
    part_1_name: "safety factor after 100 seconds",
    part_1,
    part_2_name: "seconds for first Christmas tree",
    part_2,
};

fn part_1(input: &str) -> usize {
    part_1_sized(input, 101, 103)
}

fn part_1_sized(input: &str, width: usize, length: usize) -> usize {
    let robots = parse_input(input);
    let mut quadrants: HashMap<Quadrant, usize> = [
        Quadrant::UpperLeft,
        Quadrant::UpperRight,
        Quadrant::LowerLeft,
        Quadrant::LowerRight,
        ]
        .iter()
        .map(|&quadrant| (quadrant, 0))
        .collect();

    for robot in robots {
        let x = calc_position_after(robot.position.x, robot.velocity.x, width, 100);
        let y = calc_position_after(robot.position.y, robot.velocity.y, length, 100);
        match determine_quadrant(x, y, width, length) {
            Quadrant::None => {},
            q => {
                quadrants.entry(q)
                    .and_modify(|count| { *count += 1 });
            },
        };
    }
    quadrants.values()
        .fold(1, |a, b| a * b)
}

fn part_2(input: &str) -> usize {
    part_2_sized(input, 101, 103)
}

fn part_2_sized(input: &str, width: usize, length: usize) -> usize {
    let robots = parse_input(input);
    let mut horizontal_band_offset = None;
    let mut vertical_band_offset = None;
    let mut i = 0;
    while i < usize::max(width, length) &&
        (horizontal_band_offset.is_none() || vertical_band_offset.is_none()) {
        let room = calc_room_after(&robots, width, length, i);
        if aligned(&room.by_x, robots.len()) {
            vertical_band_offset = Some(i);
        }
        if aligned(&room.by_y, robots.len()) {
            horizontal_band_offset = Some(i);
        }
        i += 1;
    }
    let (
        Some(horizontal_band_offset),
        Some(vertical_band_offset),
    ) = (horizontal_band_offset, vertical_band_offset) else {
        panic!("didn't find band offsets in expected iterations");
    };
    for i in 0..=width {
        let maybe_overlap = i * width + vertical_band_offset;
        if (maybe_overlap - horizontal_band_offset) % length == 0 {
            return maybe_overlap
        }
    }
    panic!("didn't find overlap in expected iterations");
}

fn calc_room_after(robots: &[Robot], width: usize, length: usize, seconds: usize) -> Room {
    let mut by_x = vec![0; width];
    let mut by_y = vec![0; length];
    for robot in robots {
        let x = calc_position_after(robot.position.x, robot.velocity.x, width, seconds);
        let y = calc_position_after(robot.position.y, robot.velocity.y, length, seconds);
        by_x[x] += 1;
        by_y[y] += 1;
    }
    Room { by_x, by_y }
}

fn aligned(
    room: &[usize],
    robot_count: usize,
) -> bool {
    let band_width = room.len() / 3;
    let mut count = 0;
    for i in 0..band_width {
        count += room[i];
    }
    for i in band_width..room.len() {
        if is_band(count, robot_count) {
            return true;
        }
        count -= room[i - band_width];
        count += room[i];
    }
    is_band(count, robot_count)
}

fn is_band(count: usize, robot_count: usize) -> bool {
    count as f64 > robot_count as f64 * 0.75
}

fn calc_position_after(
    position: isize,
    velocity: isize,
    room_size: usize,
    seconds: usize
) -> usize {
    let room_size = isize::try_from(room_size).unwrap();
    let seconds = isize::try_from(seconds).unwrap();
    let result = (position + (seconds * velocity) % room_size + room_size) % room_size;
    usize::try_from(result).unwrap()
}

fn determine_quadrant(x: usize, y: usize, width: usize, length: usize) -> Quadrant {
    let half_width = width / 2;
    let half_length = length / 2;
    if x < half_width && y < half_length {
        Quadrant::UpperLeft
    } else if x < half_width && y > half_length {
        Quadrant::LowerLeft
    } else if x > half_width && y < half_length {
        Quadrant::UpperRight
    } else if x > half_width && y > half_length {
        Quadrant::LowerRight
    } else {
        Quadrant::None
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input.lines()
        .map(|line| {
            let (position, velocity) = line.split_once(" ").unwrap();
            Robot { position: parse_part(position), velocity: parse_part(velocity) }
        })
        .collect()
}

fn parse_part(part: &str) -> Point {
    let (x, y) = part[2..].split_once(",").unwrap();
    Point { x: x.parse().unwrap(), y: y.parse().unwrap() }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Quadrant { UpperLeft, UpperRight, LowerLeft, LowerRight, None }

struct Robot {
    position: Point,
    velocity: Point,
}

struct Room {
    by_x: Vec<usize>,
    by_y: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(12, part_1_sized(input, 11, 7));
    }
}