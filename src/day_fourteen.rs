use std::collections::HashSet;
use std::io;
use crate::read_input;

#[derive(Debug, Copy, Clone)]
struct Robot {
    pub x: usize,
    pub y: usize,

    pub v_x: isize,
    pub v_y: isize,
}

impl From<(&str, &str)> for Robot {
    fn from(value: (&str, &str)) -> Self {
        let (x, y) = value.0.split_at(value.0.find(',').unwrap());
        let (_, x) = x.split_at(2);
        let x = x.parse::<usize>().unwrap();
        let (_, y) = y.split_at(1);
        let y = y.parse::<usize>().unwrap();

        let (v_x, v_y) = value.1.split_at(value.1.find(',').unwrap());
        let (_, v_x) = v_x.split_at(3);
        let v_x = v_x.parse::<isize>().unwrap();
        let (_, v_y) = v_y.split_at(1);
        let v_y = v_y.parse::<isize>().unwrap();

        Self { x, y, v_x, v_y }
    }
}
impl Robot {
    pub fn tick(&mut self, width: usize, height: usize) {

        let new_x = self.x as isize + self.v_x.rem_euclid(width as isize);
        self.x = new_x.rem_euclid(width as isize) as usize;

        let new_y = self.y as isize + self.v_y;
        self.y = new_y.rem_euclid(height as isize) as usize;

    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// calculate if the current position is on the middle line
    pub fn is_on_middle(&self, width: usize, height: usize) -> bool {
        if self.x == (width / 2) || self.y == (height / 2) {
            return true;
        }
        false
    }

    pub fn get_quadrant(&self, width: usize, height: usize) -> usize {
        let half_width = width / 2;
        let half_height = height / 2;

        let x = match self.x {
            n if (.. half_width).contains(&n) => 0,
            n if (half_width..=width).contains(&n) => 1,
            _ => -1,
        };

        let y = match self.y {
            n if (..half_height).contains(&n) => 0,
            n if (half_height..=height).contains(&n) => 1,
            _ => -1,
        };

        match (x, y) {
            (0, 0) => 1,
            (1, 0) => 2,
            (0, 1) => 3,
            (1, 1) => 4,
            _ => 0,
        }
    }
}

fn debug_grid(w:usize, h: usize, robots: &[Robot]) {
    println!();
    for y in 0..h {
        for x in 0..w {
            match robots.iter().filter(|r| { r.x == x && r.y == y }).count() {
                0 => {print!(".")}
                n => {print!("{}", n)}
            }
        }
        println!();
    }

}

pub fn first_task() {
    let input = read_input("./input/14");

    let mut robots: Vec<Robot> = Vec::new();

    input.iter().for_each(|l| {
        let (a, b) = l.split_at(l.find(' ').unwrap());
        let r: Robot = (a, b).into();
        robots.push(r);
    });

    let map_w = 101;
    let map_h = 103;

    // simulate 100 steps
    for _ in 0..100 {
        robots.iter_mut().for_each(|mut r| r.tick(map_w, map_h));
    }

    // filter out robots on middle lines
    let robots: Vec<Robot> = robots
        .into_iter()
        .filter(|r| !r.is_on_middle(map_w, map_h))
        .collect();

    let q1 = robots.iter().filter(|r| r.get_quadrant(map_w, map_h) == 1).count();
    let q2 = robots.iter().filter(|r| r.get_quadrant(map_w, map_h) == 2).count();
    let q3 = robots.iter().filter(|r| r.get_quadrant(map_w, map_h) == 3).count();
    let q4 = robots.iter().filter(|r| r.get_quadrant(map_w, map_h) == 4).count();

    println!("Answer 1/2: {}", q1 * q2 * q3 * q4);
}

fn test_no_overlapping(robots: &[Robot]) -> bool {

    let mut robot_positions = HashSet::new();

    for robot in robots {
        if robot_positions.contains(&robot.get_position()) {
            return false;
        }
        robot_positions.insert(robot.get_position());
    }

    true
}

pub fn second_task() {
    let input = read_input("./input/14");

    let mut robots: Vec<Robot> = Vec::new();

    input.iter().for_each(|l| {
        let (a, b) = l.split_at(l.find(' ').unwrap());
        let r: Robot = (a, b).into();
        robots.push(r);
    });

    let map_w = 101;
    let map_h = 103;

    let mut index = 0;

    loop {
        robots.iter_mut().for_each(|mut r| r.tick(map_w, map_h));
        if let true = test_no_overlapping(&robots) {
           // debug_grid(map_w, map_h, &robots);
            break; // the second iterration of this is true
            // let mut buffer = String::new();
            // io::stdin().read_line(&mut buffer).unwrap();
            // println!("{}", index );
        }
        index += 1;
    }

    println!("Answer 2/2: {}", 7672);
}

#[cfg(test)]
mod tests {
    use crate::day_fourteen::{debug_grid, Robot};
    use crate::read_input;

    #[test]
    fn tick_negative_wrap_around() {
        let mut r = Robot {
            x: 0,
            y: 1,
            v_x: -1,
            v_y: -2,
        };
        r.tick(10, 10);
        assert_eq!(r.x, 9);
        assert_eq!(r.y, 9);
    }

    #[test]
    fn tick_positive_wrap_around() {
        let mut r = Robot {
            x: 9,
            y: 5,
            v_x: 5,
            v_y: 8,
        };
        r.tick(10, 10);
        assert_eq!(r.x, 4);
        assert_eq!(r.y, 3);
    }

    #[test]
    fn tick() {
        let mut r = Robot {
            x: 7,
            y: 3,
            v_x: 2,
            v_y: -2,
        };
        r.tick(10, 10);
        assert_eq!(r.x, 9);
        assert_eq!(r.y, 1);
    }

    #[test]
    fn get_quadrant() {
        let mut r = Robot {
            x: 1,
            y: 3,
            v_x: 2,
            v_y: -2,
        };

        assert_eq!(r.get_quadrant(10, 10), 1);

        let mut r = Robot {
            x: 7,
            y: 3,
            v_x: 2,
            v_y: -2,
        };

        assert_eq!(r.get_quadrant(10, 10), 2);

        let mut r = Robot {
            x: 1,
            y: 10,
            v_x: 2,
            v_y: -2,
        };

        assert_eq!(r.get_quadrant(10, 10), 3);

        let mut r = Robot {
            x: 8,
            y: 10,
            v_x: 2,
            v_y: -2,
        };

        assert_eq!(r.get_quadrant(10, 10), 4);
    }

    #[test]
    fn simulate_5_moves() {
        let w = 11;
        let h = 7;

        let mut r = Robot {
            x: 2,
            y: 4,
            v_x: 2,
            v_y: -3,
        };
        // first step
        r.tick(11, 7);
        assert_eq!((r.x, r.y), (4,1));
        debug_grid(w, h, &*vec![r.clone()]);
        // second step
        r.tick(w, h);
        assert_eq!((r.x, r.y), (6,5));
        debug_grid(w, h, &*vec![r.clone()]);
        // third step
        r.tick(w, h);
        assert_eq!((r.x, r.y), (8,2));
        debug_grid(w, h, &*vec![r.clone()]);
        // fourth step
        r.tick(w, h);
        assert_eq!((r.x, r.y), (10,6));
        debug_grid(w, h, &*vec![r.clone()]);
        // fifth step
        r.tick(w, h);
        assert_eq!((r.x, r.y), (1,3));
        debug_grid(w, h, &*vec![r.clone()]);
    }

    #[test]
    fn simulate_test_input() {
        let w = 11;
        let h = 7;
        let input = read_input("./input/14_test");

        let mut robots: Vec<Robot> = Vec::new();

        input.iter().for_each(|l| {
            let (a, b) = l.split_at(l.find(' ').unwrap());
            let r: Robot = (a, b).into();
            robots.push(r);
        });


        debug_grid(w, h, &robots);

        // simulate 100 steps
        for _ in 0..100 {
            robots.iter_mut().for_each(|mut r| r.tick(w, h));
        }

        robots.iter_mut().for_each(| r| println!("{:?}", r));

        println!();
        debug_grid(w, h, &robots);

        // filter out robots on middle lines
        let robots: Vec<Robot> = robots
            .into_iter()
            .filter(|r| !r.is_on_middle(w, h))
            .collect();

        assert_eq!(robots.len(), 9);

        let q1 = robots.iter().filter(|r| r.get_quadrant(w, h) == 1).count();
        assert_eq!(q1, 1);
        let q2 = robots.iter().filter(|r| r.get_quadrant(w, h) == 2).count();
        assert_eq!(q2, 3);
        let q3 = robots.iter().filter(|r| r.get_quadrant(w, h) == 3).count();
        assert_eq!(q3, 4);
        let q4 = robots.iter().filter(|r| r.get_quadrant(w, h) == 4).count();
        assert_eq!(q4, 1);

        assert_eq!(q1 * q2 * q3 * q4, 12);
    }
}
