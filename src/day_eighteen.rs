use std::collections::HashSet;
use std::io;
use std::num::ParseIntError;
use std::ops::Not;
use pathfinding::prelude::astar;
use crate::read_input;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    fn distance(&self, other: &Position) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }

    fn successors(&self, size: &usize, taken_positions: &[Position]) -> Vec<(Position, u32)> {
        static COST: u32 = 1;

        let &Position { x, y } = self;

        let mut possible_moves = vec![
            // right
            Position { x: x + 1, y },
            // left
            Position { x: x.saturating_sub(1), y },
            // down
            Position { x, y: y + 1 },
            // up
            Position { x, y: y.saturating_sub(1) },
        ];

        let possible_moves: Vec<Position> = possible_moves.into_iter()
            // not out of bounds
            .filter(|p| {
                (p.x >= 0 && p.x <= *size) && (p.y >= 0 && p.y <= *size)
            })
            // not in taken positions
            .filter(|p| {
                !taken_positions.contains(&p)
            })
            .collect();

        // assign cost to each move
        possible_moves
            .into_iter().map(|p| (p, COST)).collect()
    }
}

impl From<(&str, &str)> for Position {
    fn from(value: (&str, &str)) -> Self {
        let x = value.0.parse::<usize>().unwrap();
        let (_, y) = value.1.split_at(1);
        let y = y.parse::<usize>().unwrap();

        Self { x, y }
    }
}

fn debug_grid(w: usize, h: usize, positions: &[Position]) {
    println!();
    for y in 0..h {
        for x in 0..w {
            match positions.iter().filter(|r| { r.x == x && r.y == y }).count() {
                0 => { print!(".") }
                n => { print!("{}", n) }
            }
        }
        println!();
    }
}


pub fn first_task() {
    let input = read_input("./input/18");

    let mut taken_positions: Vec<Position> = Vec::new();

    input.iter().for_each(|l| {
        let (x, y) = l.split_at(l.find(',').unwrap());
        let p: Position = (x, y).into();
        taken_positions.push(p);
    });

    let size = 70;

    let final_position: Position = Position { x: size, y: size };

    let result = astar(&Position { x: 0, y: 0 },
                       |p| p.successors(&size, &taken_positions[0..1024]),
                       |p| 1,
                       |p| *p == final_position);

    if let Some(result) = result {
        println!("Answer 1/2: {:?}", result.1)
    } else {
        println!("Answer 1/2: Unknown!");
    }
}


pub fn second_task() {
    let input = read_input("./input/14");


    println!("Answer 2/2: {}", 0);
}

#[cfg(test)]
mod tests {
    use pathfinding::prelude::astar;
    use crate::day_eighteen::{debug_grid, Position};
    use crate::read_input;

    #[test]
    fn simulate_run() {
        let input = read_input("./input/18_test");

        let mut taken_positions: Vec<Position> = Vec::new();

        input.iter().for_each(|l| {
            let (x, y) = l.split_at(l.find(',').unwrap());
            let p: Position = (x, y).into();
            taken_positions.push(p);
        });

        let size = 6;

        debug_grid(size + 1, size + 1, &taken_positions[0..12]);

        let final_position: Position = Position { x: size, y: size };

        let result = astar(&Position { x: 0, y: 0 }, |p| p.successors(&size, &taken_positions[0..12]), |p| 1,
                           |p| *p == final_position);

        assert!(result.is_some(), "Pathfinding did not work!");
        if let Some(result) = result {
            // print visited paths
            debug_grid(size, size, &result.0);
            assert_eq!(result.1, 22)
        }
    }
}
