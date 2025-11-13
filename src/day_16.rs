mod maze;

use crate::common::DaySpec;
use crate::day_16::maze::Maze;

pub const DAY_SIXTEEN: DaySpec<u32, usize> = DaySpec {
    day_num: 16,
    part_1_name: "lowest score possible",
    part_1,
    part_2_name: "tiles on best paths",
    part_2,
};

fn part_1(input: &str) -> u32 {
    Maze::parse_and_traverse(input)
        .min_score()
}

fn part_2(input: &str) -> usize {
    Maze::parse_and_traverse(input)
        .best_seats_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    const INPUT_LARGE: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part_1_sample() {
        assert_eq!(7036, part_1(INPUT_SMALL));
        assert_eq!(11048, part_1(INPUT_LARGE));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(45, part_2(INPUT_SMALL));
        assert_eq!(64, part_2(INPUT_LARGE));
    }
}