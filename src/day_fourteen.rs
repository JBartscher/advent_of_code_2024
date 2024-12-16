use crate::read_input;

#[derive(Debug)]
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
        let new_x = (self.x as isize) + self.v_x;
        if new_x < 0 {
            self.x = ((width as isize + 1) + new_x) as usize;
        } else if new_x > width as isize {
            self.x = (new_x - (width as isize + 1)) as usize;
        } else {
            self.x = new_x as usize;
        }

        let new_y = (self.y as isize) + self.v_y;
        if new_y < 0 {
            self.y = ((height as isize + 1) + new_y) as usize;
        } else if new_y > height as isize {
            self.y = (new_y - (height as isize + 1)) as usize;
        } else {
            self.y = new_y as usize;
        }
    }

    /// calculate if the current position is on the middle line
    pub fn is_on_middle(&self, width: usize, height: usize) -> bool {
        if self.x == (width / 2 + 1) || self.y == (height / 2 + 1) {
            return true;
        }
        false
    }

    pub fn get_quadrant(&self, width: usize, height: usize) -> usize {
        let half_width = width / 2;
        let half_height = height / 2;

        let x = match self.x {
            n if (0..half_width).contains(&n) => 0,
            n if (half_width..=width).contains(&n) => 1,
            _ => -1
        };

        let y = match self.y {
            n if (0..half_height).contains(&n) => 0,
            n if (half_height..=height).contains(&n) => 1,
            _ => -1
        };

        match (x, y) {
            (0, 0) => 1,
            (1, 0) => 2,
            (0, 1) => 3,
            (1, 1) => 4,
            _ => 0
        }
    }
}

pub fn first_task() {
    let input = read_input("./input/14_test");

    let mut robots: Vec<Robot> = Vec::new();

    input.iter().for_each(|l| {
        let (a, b) = l.split_at(l.find(' ').unwrap());
        let r: Robot = (a, b).into();
        println!("robot:{:?}", r);
        robots.push(r);
        // second_list.push(b.trim().parse::<i64>().unwrap());
    });

    let mut r = Robot {
        x: 5,
        y: 10,
        v_x: 6,
        v_y: 1,
    };

    r.tick(10, 10);

    println!("Answer 1/2: {}", 0);
}

pub fn second_task() {
    println!("Answer 2/2: {}", 0);
}

#[cfg(test)]
mod tests {
    use crate::day_fourteen::Robot;

    #[test]
    fn tick_negative_wrap_around() {
        let mut r = Robot {
            x: 0,
            y: 1,
            v_x: -1,
            v_y: -2,
        };
        r.tick(10, 10);
        assert_eq!(r.x, 10);
        assert_eq!(r.y, 10);
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
        assert_eq!(r.x, 3);
        assert_eq!(r.y, 2);
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
}
