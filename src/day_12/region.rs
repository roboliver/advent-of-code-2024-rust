use std::collections::{HashMap, HashSet};
use crate::common::Direction;

pub struct Region {
    plant_type: u8,
    plants: HashMap<(isize, isize), Plant>,
}

impl Region {
    pub fn new(plant_type: u8) -> Self {
        Region { plant_type, plants: HashMap::new() }
    }

    pub fn add_plant(&mut self, row: isize, col: isize) {
        let mut plant = Plant::new();
        self.attach_adjacent(row - 1, col, &mut plant, Direction::NORTH);
        self.attach_adjacent(row + 1, col, &mut plant, Direction::SOUTH);
        self.attach_adjacent(row, col - 1, &mut plant, Direction::WEST);
        self.attach_adjacent(row, col + 1, &mut plant, Direction::EAST);
        self.plants.insert((row, col), plant);
    }

    fn attach_adjacent(&mut self, row: isize, col: isize, plant: &mut Plant, dir: Direction) {
        self.plants.entry((row, col)).and_modify(|adjacent| {
            plant.add_adjacent(dir);
            adjacent.add_adjacent(dir.reverse());
        });
    }

    pub fn area(&self) -> usize {
        self.plants.len()
    }

    pub fn perimeter(&self) -> usize {
        self.plants.values()
            .map(Plant::border_sides)
            .sum()
    }

    pub fn number_of_sides(&self) -> usize {
        let mut sides = 0;
        for dir in [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST] {
            let mut borders_in_dir = HashMap::new();
            for ((row, col), plant) in self.plants.iter() {
                if !plant.has_adjacent(dir) {
                    let (&edge, &position) = {
                        // for horizontal borders (north, south) the edge is the row, and we will
                        // be looking for blocks of adjacent positions, i.e. columns, along this
                        // edge; for vertical borders (east, west) it's the other way round.
                        if dir == Direction::NORTH || dir == Direction::SOUTH {
                            (row, col)
                        } else {
                            (col, row)
                        }
                    };
                    borders_in_dir.entry(edge)
                        .or_insert(HashSet::new())
                        .insert(position);
                }
            }
            for aligned_borders in borders_in_dir.values() {
                for &border in aligned_borders {
                    if border == 0 || !aligned_borders.contains(&(border - 1)) {
                        sides += 1;
                    }
                }
            }
        }
        sides
    }

    pub fn plant_type(&self) -> u8 {
        self.plant_type
    }
}

struct Plant {
    has_north: bool,
    has_east: bool,
    has_south: bool,
    has_west: bool,
}

impl Plant {
    fn new() -> Self {
        Self { has_north: false, has_east: false, has_south: false, has_west: false }
    }

    fn add_adjacent(&mut self, dir: Direction) {
        match dir {
            Direction::NORTH => { self.has_north = true; }
            Direction::EAST => { self.has_east = true; }
            Direction::SOUTH => { self.has_south = true; }
            Direction::WEST => { self.has_west = true; }
        }
    }

    fn has_adjacent(&self, dir: Direction) -> bool {
        match dir {
            Direction::NORTH => self.has_north,
            Direction::EAST => self.has_east,
            Direction::SOUTH => self.has_south,
            Direction::WEST => self.has_west,
        }
    }

    fn border_sides(&self) -> usize {
        [self.has_north, self.has_east, self.has_south, self.has_west]
            .iter()
            .filter(|&&dir| !dir)
            .count()
    }
}
