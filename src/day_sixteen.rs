use std::collections::HashSet;
use std::ops::Add;
use crate::read_input;


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    pub x: isize,
    pub y: isize,
}

const X: Position = Position { x: 1, y: 0 };
const NEG_X: Position = Position { x: -1, y: 0 };
const Y: Position = Position { x: 0, y: 1 };
const NEG_Y: Position = Position { x: 0, y: -1 };

#[derive(Debug, Clone)]
enum Direction {
    East,
    West,
    South,
    North,
}

impl Direction {
    fn vector(&self) -> Position {
        match self {
            Direction::East => {
                X
            }
            Direction::West => {
                NEG_X
            }
            Direction::South => {
                NEG_Y
            }
            Direction::North => {
                Y
            }
        }
    }

    fn rotated_q(&self) -> Direction {
        match self {
            Direction::East => {
                Direction::South
            }
            Direction::West => {
                Direction::North
            }
            Direction::South => {
                Direction::West
            }
            Direction::North => {
                Direction::East
            }
        }
    }

    fn rotated_cq(&self) -> Direction {
        match self {
            Direction::East => {
                Direction::North
            }
            Direction::West => {
                Direction::South
            }
            Direction::South => {
                Direction::East
            }
            Direction::North => {
                Direction::West
            }
        }
    }
}

fn successors(input: (Position, Direction), size: &usize, walls: &[Position]) -> Vec<(Position, u32)> {
    let (position, direction) = input;
    let next_pos = position.add(&direction.vector());

    let moves = match walls.contains(&next_pos) {
        true => {
            vec![
                ((position.clone(), direction.rotated_q()), 1000),
                ((position, direction.rotated_cq()), 1000),
            ]
        }
        false => {
            vec![
                ((next_pos, direction.clone()), 1),
                ((position.clone(), direction.rotated_q()), 1000),
                ((position.clone(), direction.rotated_cq()), 1000),
            ]
        }
    };


    todo!()
}

impl Position {
    fn distance(&self, other: &Position) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }

    fn add(&self, other: &Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

// fn debug_grid(w: usize, h: usize, walls: HashSet<Position>, start_position: &Position, final_position: &Position) {
//     println!();
//     for y in 0..h {
//         for x in 0..w {
//             if x == start_position.x && y == start_position.y {
//                 print!("S");
//                 continue;
//             }
//             if x == final_position.x && y == final_position.y {
//                 print!("E");
//                 continue;
//             }
//             match walls.iter().filter(|r| { r.x == x && r.y == y }).count() {
//                 0 => { print!(".") }
//                 n => { print!("{}", "#") }
//             }
//         }
//         println!();
//     }
// }

fn build_maze(input: Vec<String>) -> (HashSet<Position>, Position, Position) {
    let mut walls: HashSet<Position> = HashSet::new();
    let mut start_position = Position { x: 0, y: 0 };
    let mut final_position = Position { x: 0, y: 0 };

    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert(Position { x: x as isize, y: y as isize });
                }
                'S' => {
                    start_position = Position { x: x as isize, y: y as isize };
                }
                'E' => {
                    final_position = Position { x: x as isize, y: y as isize };
                }
                '.' => {
                    // empty field do nothing
                }
                unknown => { panic!("this should not happen. Unknown char {}", unknown) }
            }
        }
    };

    // check that final position was set correctly
    assert_ne!(start_position, Position { x: 0, y: 0 });
    assert_ne!(final_position, Position { x: 0, y: 0 });

    (walls, start_position, final_position)
}

pub fn first_task() {
    let input = read_input("./input/16");

    let (walls, start_position, final_position) = build_maze(input);

    // debug_grid(141, 141, walls, &start_position, &final_position);

    println!("Answer 1/2: {}", 0);
}

pub fn second_task() {
    let input = read_input("./input/16");
    println!("Answer 2/2: {}", 0);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use pathfinding::prelude::astar;
    use crate::day_sixteen::{build_maze, Direction, Position};
    use crate::read_input;

    #[test]
    fn simulate_task_one_run() {
        let input = read_input("./input/16_test");

        let (walls, start_position, final_position) = build_maze(input);

        // debug_grid(141, 141, walls, &start_position, &final_position);

    }
}
