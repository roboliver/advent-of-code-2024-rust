mod garden;

use crate::common::DaySpec;
use crate::day_12::garden::Region;

pub const DAY_TWELVE: DaySpec<u32, u32> = DaySpec {
    day_num: 12,
    part_1_name: "total fencing price",
    part_1,
    part_2_name: "total fencing price with bulk discount",
    part_2,
};

fn part_1(input: &str) -> u32 {
    parse_input(input).iter()
        .map(|region| (region.area() * region.perimeter()) as u32)
        .sum()
}

fn part_2(input: &str) -> u32 {
    parse_input(input).iter()
        .map(|region| (region.area() * region.number_of_sides()) as u32)
        .sum()
}

fn parse_input(input: &str) -> Vec<Region> {
    let plant_types: Vec<Vec<u8>> = input.lines()
        .map(|line| line.bytes().collect())
        .collect();
    let mut plants_mapped: Vec<Vec<bool>> = plant_types.iter()
        .map(|row| vec![false; row.len()])
        .collect();
    let regions = build_regions(&plant_types, &mut plants_mapped);
    regions
}

fn build_regions(plant_types: &[Vec<u8>], plants_mapped: &mut [Vec<bool>]) -> Vec<Region> {
    let mut regions = Vec::new();
    for row in 0..plant_types.len() {
        for col in 0..plant_types[row].len() {
            if !plants_mapped[row][col] {
                let region = build_region(row, col, plant_types, plants_mapped);
                regions.push(region);
            }
        }
    }
    regions
}

fn build_region(
    row: usize,
    col: usize,
    plant_types: &[Vec<u8>],
    plants_mapped: &mut [Vec<bool>],
) -> Region {
    let mut region = Region::new(plant_types[row][col]);
    // start with the first plant at the region origin; this function will then recursively
    // add adjacent plants until the whole region is filled in
    add_plant_to_region(row, col, plant_types, plants_mapped, &mut region);
    region
}

fn add_plant_to_region(
    row: usize,
    col: usize,
    plant_types: &[Vec<u8>],
    plants_mapped: &mut [Vec<bool>],
    region: &mut Region,
) {
    if !plants_mapped[row][col] && plant_types[row][col] == region.plant_type() {
        // part of the same region that we haven't mapped out yet.
        region.add_plant(
            isize::try_from(row).unwrap(),
            isize::try_from(col).unwrap());
        plants_mapped[row][col] = true;
        expand_region(row, col, plant_types, plants_mapped, region);
    }
}

fn expand_region(
    row: usize,
    col: usize,
    plant_types: &[Vec<u8>],
    plants_mapped: &mut [Vec<bool>],
    region: &mut Region,
) {
    if row > 0 {
        add_plant_to_region(row - 1, col, plant_types, plants_mapped, region);
    }
    if row < plant_types.len() - 1 {
        add_plant_to_region(row + 1, col, plant_types, plants_mapped, region);
    }
    if col > 0 {
        add_plant_to_region(row, col - 1, plant_types, plants_mapped, region);
    }
    if col < plant_types[row].len() - 1 {
        add_plant_to_region(row, col + 1, plant_types, plants_mapped, region);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part_1_sample() {
        assert_eq!(1930, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(1206, part_2(INPUT));
    }
}