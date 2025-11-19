use std::collections::HashMap;
use crate::common::{DaySpec, Point};

pub const DAY_FOURTEEN: DaySpec<u32, usize> = DaySpec {
    day_num: 14,
    part_1_name: "safety factor after 100 seconds",
    part_1,
    part_2_name: "seconds for first Christmas tree",
    part_2,
};

fn part_1(input: &str) -> u32 {
    part_1_sized(input, 101, 103)
}

fn part_1_sized(input: &str, width: usize, length: usize) -> u32 {
    let robots = parse_input(input);
    let mut quadrants: HashMap<Quadrant, u32> = [
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
        .fold(1, |quadrant, count| quadrant * count)
}

fn part_2(input: &str) -> usize {
    part_2_sized(input, 101, 103)
}

fn part_2_sized(input: &str, width: usize, length: usize) -> usize {
    let robots = parse_input(input);
    let mut horizontal_band_offset = None;
    let mut vertical_band_offset = None;
    let mut i = 0;
    let mut by_x = vec![RobotsAfter { seconds: 0, count: 0 }; width];
    let mut by_y = vec![RobotsAfter { seconds: 0, count : 0}; length];
    while i < usize::max(width, length) &&
        (horizontal_band_offset.is_none() || vertical_band_offset.is_none()) {
        calc_room_after(&robots, &mut by_x, &mut by_y, i);
        if aligned(&by_x, robots.len(), i) {
            vertical_band_offset = Some(i);
        }
        if aligned(&by_y, robots.len(), i) {
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

fn calc_room_after(robots: &[Robot], by_x: &mut [RobotsAfter], by_y: &mut [RobotsAfter], seconds: usize) {
    let width = by_x.len();
    let length = by_y.len();
    for robot in robots {
        let x = calc_position_after(robot.position.x, robot.velocity.x, width, seconds);
        by_x[x] = update_room(&by_x[x], seconds);
        let y = calc_position_after(robot.position.y, robot.velocity.y, length, seconds);
        by_y[y] = update_room(&by_y[y], seconds);
    }
}

fn update_room(robots_after: &RobotsAfter, seconds: usize) -> RobotsAfter {
    if robots_after.seconds == seconds {
        RobotsAfter { seconds, count: robots_after.count + 1 }
    } else {
        RobotsAfter { seconds, count: 1 }
    }
}

fn aligned(
    room: &[RobotsAfter],
    robot_count: usize,
    seconds: usize,
) -> bool {
    let band_width = room.len() / 3;
    let mut count = 0;
    for i in 0..band_width {
        plus_count(&room[i], seconds, &mut count);
    }
    for i in band_width..room.len() {
        if is_band(count, robot_count) {
            return true;
        }
        minus_count(&room[i - band_width], seconds, &mut count);
        plus_count(&room[i], seconds, &mut count);
    }
    is_band(count, robot_count)
}

fn plus_count(robots_after: &RobotsAfter, seconds: usize, count: &mut usize) {
    if robots_after.seconds == seconds {
        *count += robots_after.count;
    }
}

fn minus_count(robots_after: &RobotsAfter, seconds: usize, count: &mut usize) {
    if robots_after.seconds == seconds {
        *count -= robots_after.count;
    }
}

fn is_band(count: usize, robot_count: usize) -> bool {
    count as f64 > robot_count as f64 * 0.75
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

#[derive(Clone)]
struct RobotsAfter {
    seconds: usize,
    count: usize,
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