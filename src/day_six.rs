use crate::read_input;
use lazy_static::lazy_static;
use std::collections::{HashSet};
use std::sync::Mutex;
use std::thread;

const MIN: usize = 0;
const MAX: usize = 10;

enum Direction {
    North,
    East,
    South,
    West,
}

lazy_static! {
    static ref VISITED_LOCATIONS: Mutex<HashSet<(usize, usize)>> = Mutex::new(HashSet::new());
}

pub fn first_task() {
    let mut rows: Vec<Vec<char>> = Vec::new();

    let input = read_input("./input/6_test");
    input.iter().for_each(|l| {
        let row = l.chars().collect::<Vec<char>>();
        rows.push(row);
    });

    let start_position = find_start_position(&rows);

    // insert the first pos
    VISITED_LOCATIONS
        .lock()
        .unwrap()
        .insert((start_position.0, start_position.1));

    traverse_maze(start_position, &rows, Direction::North, true);

    println!("Answer 1/2: {}", VISITED_LOCATIONS.lock().unwrap().len());
}

pub fn second_task() {

    VISITED_LOCATIONS
        .lock()
        .unwrap().clear();

    let mut rows: Vec<Vec<char>> = Vec::new();

    let input = read_input("./input/6_test");
    input.iter().for_each(|l| {
        let row = l.chars().collect::<Vec<char>>();
        rows.push(row);
    });

    let start_position = find_start_position(&rows);
    traverse_maze(start_position, &rows, Direction::North, true);

    // visited locations now contains all positions where we can put an obstacle

    let mut handles = vec![];

    for location in VISITED_LOCATIONS.lock().unwrap().iter() {
        let mut maze_with_obstacle = rows.clone();
        maze_with_obstacle[location.0][location.1] = '#';

        let handle = thread::Builder::new()
            .stack_size(64 * 1024*  1024) // Optionally set stack size if needed
            .spawn(move || {
                traverse_maze_with_obstacle(start_position, &maze_with_obstacle, Direction::North, 0)
            });
        if let Ok(handle) = handle {
            handles.push(handle);
        } else {
            println!("Can't spawn thread")
        }
        //break;
    }

    let sum: Vec<usize> =  handles.into_iter().map(|h| {h.join().unwrap()}).collect();
    let sum: usize = sum.iter().sum();

    println!("Answer 2/2: {}",sum); // 791 > x < 5029
}

fn change_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn traverse_maze(
    position: (usize, usize),
    maze: &Vec<Vec<char>>,
    direction: Direction,
    collect: bool,
) {
    let mut new_position = (position.0, position.1);
    match direction {
        Direction::North => {
            let pos = new_position.1.checked_sub(1);
            if let Some(y) = pos {
                new_position.1 = y;
            } else {
                return;
            }
        }
        Direction::East => new_position.0 += 1,
        Direction::South => new_position.1 += 1,
        Direction::West => {
            let pos = new_position.0.checked_sub(1);
            if let Some(x) = pos {
                new_position.0 = x;
            } else {
                return;
            }
        }
    }
    // out of bounds
    if (new_position.0 < 0 || new_position.0 >= MAX)
        || (new_position.1 < 0 || new_position.1 >= MAX)
    {
        return;
    }

    if maze[new_position.1][new_position.0] == '#' {
        let new_direction = change_direction(direction);
        return traverse_maze(position, maze, new_direction, collect);
    }

    if collect {
        VISITED_LOCATIONS
            .lock()
            .unwrap()
            .insert((new_position.0, new_position.1));
    }

    traverse_maze(new_position, maze, direction, collect)
}

fn traverse_maze_with_obstacle(
    position: (usize, usize),
    maze: &Vec<Vec<char>>,
    direction: Direction,
    mut recursion_depth: usize,
) -> usize {

    // println!("recursion depth {}.", recursion_depth);

    let mut new_position = (position.0, position.1);
    match direction {
        Direction::North => {
            // y - 1
            let pos = new_position.1.checked_sub(1);
            if let Some(y) = pos {
                new_position.1 = y;
            } else {
                return 0;
            }
        }
        Direction::East => new_position.0 += 1,
        Direction::South => new_position.1 += 1,
        Direction::West => {
            // x - 1
            let pos = new_position.0.checked_sub(1);
            if let Some(x) = pos {
                new_position.0 = x;
            } else {
                return 0;
            }
        }
    }
    // out of bounds
    if (new_position.0 < 0 || new_position.0 >= MAX)
        || (new_position.1 < 0 || new_position.1 >= MAX)
    {
        return 0;
    }

    if maze[new_position.1][new_position.0] == '#' {
        let new_direction = change_direction(direction);
        return traverse_maze_with_obstacle(
            position,
            maze,
            new_direction,
            recursion_depth + 1,
        );
    }

    if recursion_depth > 25000 {
        println!("we encountered  a loop");
        return 1;
    }

    traverse_maze_with_obstacle(
        new_position,
        maze,
        direction,
        recursion_depth + 1,
    )
}

fn count_occurrences<T: PartialEq>(haystack: &[T], needle: &[T]) -> usize {
    if needle.is_empty() || haystack.len() < needle.len() {
        return 0; // Edge cases: needle is empty or larger than haystack
    }

    let result = haystack.windows(needle.len()).filter(|window| *window == needle).count();
    result
}

fn find_start_position(maze: &Vec<Vec<char>>) -> (usize, usize) {
    let start_line = maze.iter().find(|l| l.contains(&'^')).unwrap();
    let pos_x = start_line.iter().position(|c| c.eq(&'^')).unwrap();
    let pos_y = maze.iter().position(|l| l.contains(&'^')).unwrap();
    (pos_x, pos_y)
}
