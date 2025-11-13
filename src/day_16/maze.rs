use crate::common::Direction;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

pub struct Maze {
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    pub fn parse_and_traverse(input: &str) -> Self {
        let mut tiles = Maze::parse_maze(input);
        let mut path_ends: Vec<PathEnd> = Vec::from([
            PathEnd { position: Maze::start_position(&tiles), direction: Direction::East }
        ]);
        while !path_ends.is_empty() {
            let mut new_path_ends = Vec::new();
            for path_end in path_ends {
                Maze::explore_path(&mut tiles, path_end, path_end.turn_clockwise(),
                                   1000, &mut new_path_ends);
                Maze::explore_path(&mut tiles, path_end, path_end.turn_anticlockwise(),
                                   1000, &mut new_path_ends);
                Maze::explore_path(&mut tiles, path_end, path_end.move_forward(),
                                   1, &mut new_path_ends);
            }
            path_ends = new_path_ends;
        }
        Maze { tiles }
    }

    fn parse_maze(input: &str) -> Vec<Vec<Tile>> {
        input.lines()
            .map(|line| {
                line.chars()
                    .map(Tile::parse)
                    .collect()
            })
            .collect()
    }

    fn explore_path(
        tiles: &mut [Vec<Tile>],
        cur_path_end: PathEnd,
        new_path_end: PathEnd,
        score_increase: u32,
        new_path_ends: &mut Vec<PathEnd>,
    ) {
        let (new_row, new_col, new_dir) = new_path_end.destructure();
        let new_tile = &tiles[new_row][new_col];
        if let Tile::Path { scores: new_scores, .. } = new_tile {
            let (cur_row, cur_col, cur_dir) = cur_path_end.destructure();
            let Tile::Path { scores: cur_scores, .. } = &tiles[cur_row][cur_col] else {
                panic!("somehow we are currently in a wall.");
            };
            let new_score_via_cur = cur_scores.get_score(cur_dir) + score_increase;
            if new_score_via_cur < new_scores.get_score(new_dir) {
                let new_tile_mut = &mut tiles[new_row][new_col];
                let Tile::Path { scores: new_scores, .. } = new_tile_mut else {
                    panic!("this can't happen, it's the same path tile we already got!");
                };
                new_scores.set_score(new_score_via_cur, new_dir);
                new_path_ends.push(new_path_end);
            }
        }
    }

    pub fn min_score(&self) -> u32 {
        let end = &self.tiles[1][&self.tiles[0].len() - 2];
        match end {
            Tile::Path { path_type: PathType::End, scores } => scores.min(),
            _ => panic!("expected end tile is not actually the end tile"),
        }
    }

    pub fn best_seats_count(&self) -> usize {
        let mut best_path_ends = Vec::new();
        let end_position = self.end_position();
        best_path_ends.push(PathEnd {
            position: end_position,
            direction: match &self.tiles[end_position.row][end_position.col] {
                Tile::Path { path_type: PathType::End, scores } => scores.min_dir(),
                _ => panic!("expected end tile is not actually the end tile"),
            }
        });
        let mut best_seats = HashSet::new();
        while !best_path_ends.is_empty() {
            let mut new_best_path_starts = Vec::new();
            for best_path_end in best_path_ends {
                let (best_row, best_col, best_dir) = best_path_end.destructure();
                let Tile::Path {
                    scores: best_scores,
                    ..
                } = &self.tiles[best_row][best_col] else {
                    panic!("somehow we are currently in a wall.");
                };
                best_seats.insert(best_path_end.position);
                let prev_path_ends = [
                    best_path_end.turn_clockwise(),
                    best_path_end.turn_anticlockwise(),
                    best_path_end.move_backward(),
                ];
                for prev_path_end in prev_path_ends {
                    let (prev_row, prev_col, prev_dir) = prev_path_end.destructure();
                    let Tile::Path {
                        scores: prev_scores,
                        ..
                    } = &self.tiles[prev_row][prev_col] else {
                        continue;
                    };
                    if prev_scores.get_score(prev_dir) < best_scores.get_score(best_dir) {
                        new_best_path_starts.push(prev_path_end);
                    }
                }
            }
            best_path_ends = new_best_path_starts;
        }
        best_seats.len()
    }

    fn start_position(tiles: &[Vec<Tile>]) -> Position {
        match &tiles[tiles.len() - 2][1] {
            Tile::Path {
                path_type: PathType::Start, ..
            } => Position { row: tiles.len() - 2, col: 1},
            _ => panic!("expected start tile is not actually the start tile"),
        }
    }

    fn end_position(&self) -> Position {
        match &self.tiles[1][&self.tiles[0].len() - 2] {
            Tile::Path {
                path_type: PathType::End, ..
            } => Position { row: 1, col: &self.tiles[0].len() - 2 },
            _ => panic!("expected end tile is not actually the end tile"),
        }
    }
}

enum Tile {
    Wall,
    Path {
        path_type: PathType,
        scores: PathScores,
    },
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::new_path(PathType::Path),
            'S' => Tile::new_path(PathType::Start),
            'E' => Tile::new_path(PathType::End),
            _ => panic!("unexpected tile char: {}", c),
        }
    }

    fn new_path(path_type: PathType) -> Self {
        let east_score = if path_type == PathType::Start {
            0
        } else {
            u32::MAX
        };
        Self::Path {
            path_type,
            scores: PathScores {
                north_score: u32::MAX,
                east_score,
                south_score: u32::MAX,
                west_score: u32::MAX,
            },
        }
    }
}

#[derive(Eq, PartialEq)]
enum PathType { Start, End, Path }

struct PathScores {
    north_score: u32,
    east_score: u32,
    south_score: u32,
    west_score: u32,
}

impl PathScores {
    fn get_score(&self, direction: Direction) -> u32 {
        match direction {
            Direction::North => self.north_score,
            Direction::East => self.east_score,
            Direction::South => self.south_score,
            Direction::West => self.west_score,
        }
    }

    fn set_score(&mut self, new_score: u32, direction: Direction) {
        let score: &mut u32 = match direction {
            Direction::North => &mut self.north_score,
            Direction::East => &mut self.east_score,
            Direction::South => &mut self.south_score,
            Direction::West => &mut self.west_score,
        };
        *score = new_score;
    }

    fn min(&self) -> u32 {
        *[self.north_score, self.east_score, self.south_score, self.west_score]
            .iter()
            .min()
            .unwrap()
    }

    fn min_dir(&self) -> Direction {
        let scores_by_dir: HashMap<Direction, u32> = [
            (Direction::North, self.north_score),
            (Direction::East, self.east_score),
            (Direction::South, self.south_score),
            (Direction::West, self.west_score),
        ].into_iter().collect();
        *scores_by_dir.iter()
            .min_by_key(|(_, score)| *score)
            .map(|(dir, _)| dir)
            .unwrap()
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct PathEnd { position: Position, direction: Direction }

impl PathEnd {
    fn turn_clockwise(&self) -> PathEnd {
        Self {
            position: self.position,
            direction: self.direction.rotate_clockwise(),
        }
    }

    fn turn_anticlockwise(&self) -> PathEnd {
        Self {
            position: self.position,
            direction: self.direction.rotate_anticlockwise(),
        }
    }

    fn move_forward(&self) -> PathEnd {
        Self {
            position: self.position.move_forward(self.direction),
            direction: self.direction,
        }
    }

    fn move_backward(&self) -> PathEnd {
        Self {
            position: self.position.move_forward(self.direction.reverse()),
            direction: self.direction,
        }
    }

    fn destructure(&self) -> (usize, usize, Direction) {
        (self.position.row, self.position.col, self.direction)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Position { row: usize, col: usize }

impl Position {
    fn move_forward(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self { row: self.row - 1, col: self.col },
            Direction::East => Self { row: self.row, col: self.col + 1 },
            Direction::South => Self { row: self.row + 1, col: self.col },
            Direction::West => Self { row: self.row, col: self.col - 1 },
        }
    }
}
