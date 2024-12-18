use std::cmp::max;
use std::collections::HashSet;
use std::ops::{Add, Not};
use pathfinding::prelude::{astar_bag, dijkstra};
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

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

fn successors(input: (Position, Direction), walls: &HashSet<Position>) -> Vec<((Position, Direction), u32)> {
    let (position, direction) = input;
    let next_pos = position.add(&direction.vector());

    // println!(" {:?} - {:?} -> {:?}", position, direction, next_pos);

    // cant go out of bounds
    if next_pos.x < 0 || next_pos.y < 0 {
        return vec![
            ((position.clone(), direction.rotated_q()), 1000),
            ((position, direction.rotated_cq()), 1000),
        ];
    }

    // cant walk into walls
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


    moves
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

    let result = dijkstra(&(start_position, Direction::East),
                          |(p)| successors(p.clone(), &walls),
                          |p| p.0 == final_position);

    if let Some(result) = result {
        println!("Answer 1/2: {}", result.1);
    } else {
        println!("Answer 1/2: Unknown!");
    }
}

pub fn second_task() {
    let input = read_input("./input/16");

    let (walls, start_position, final_position) = build_maze(input);

    let result = astar_bag(&(start_position, Direction::East),
                           |(p)| successors(p.clone(), &walls),
                           |p| 1,
                           |p| p.0 == final_position);

    assert!(result.is_some(), "Error. Pathfinding returned None() but there must be a Some()");

    let mut solution_positions: HashSet<Position> = HashSet::new();

    if let Some(result) = result {
        let (solutions, _) = result;
        solutions.into_iter().for_each(|solution| {
            solution.into_iter().for_each(|(p,_)| {
                if(solution_positions.contains(&p).not()) {
                    solution_positions.insert(p);
                }
            })
        });


        println!("Answer 2/2: {}", solution_positions.len());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::hash::Hash;
    use std::ops::Not;
    use pathfinding::prelude::{astar_bag, dijkstra};
    use crate::day_sixteen::{build_maze, successors, Direction, Position};
    use crate::read_input;

    #[test]
    fn simulate_task_one() {
        let input = read_input("./input/16_test");

        let (walls, start_position, final_position) = build_maze(input);

        let result = dijkstra(&(start_position, Direction::East),
                              |(p)| successors(p.clone(), &walls),
                              |p| p.0 == final_position);

        assert!(result.is_some(), "Error. Pathfinding returned None() but there must be a Some()");

        if let Some(result) = result {
            assert_eq!(7036, result.1)
        }
    }

    #[test]
    fn simulate_task_two() {
        let input = read_input("./input/16_test");

        let (walls, start_position, final_position) = build_maze(input);

        let result = astar_bag(&(start_position, Direction::East),
                              |(p)| successors(p.clone(), &walls),
                               |p| 1,
                              |p| p.0 == final_position);

        assert!(result.is_some(), "Error. Pathfinding returned None() but there must be a Some()");

        let mut solutiuon_positions: HashSet<Position> = HashSet::new();

        if let Some(result) = result {
            let (solutions, _) = result;
            solutions.into_iter().for_each(|solution| {
                solution.into_iter().for_each(|(p,_)| {
                    if(solutiuon_positions.contains(&p).not()) {
                        solutiuon_positions.insert(p);
                    }
                })
            });


            assert_eq!(45, solutiuon_positions.len())
        }
    }
}
