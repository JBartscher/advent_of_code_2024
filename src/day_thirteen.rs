use crate::read_input;

#[derive(Clone, Copy, Debug)]
struct Button {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug)]
struct Target {
    x: isize,
    y: isize,
}


const COST_A: isize = 3;
const COST_B: isize = 1;

impl From<&str> for Target {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_at(value.find(',').unwrap());

        let (_, x) = x.split_at(9);
        let x = x.parse::<isize>().unwrap();

        let (_, y) = y.split_at(4);
        let y = y.parse::<isize>().unwrap();

        Self { x, y }
    }
}

impl Target {
    fn from_faulty_machine(value: &str) -> Self {
        let (x, y) = value.split_at(value.find(',').unwrap());

        let (_, x) = x.split_at(9);
        let x = x.parse::<isize>().unwrap() + 10000000000000;

        let (_, y) = y.split_at(4);
        let y = y.parse::<isize>().unwrap() + 10000000000000;

        Self { x, y }
    }
}
impl From<&str> for Button {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_at(value.find(',').unwrap());

        let (_, x) = x.split_at(11);
        let x = x.parse::<isize>().unwrap();

        let (_, y) = y.split_at(3);
        let y = y.parse::<isize>().unwrap();

        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
struct Clawmachine {
    pub button_a: Button,
    pub button_b: Button,

    pub target: Target,
}

impl Clawmachine {
   pub fn solve_for_s(&self) -> isize {
       let numerator = (self.target.x * self.button_b.y) - (self.target.y * self.button_b.x);
       let denominator = (self.button_a.x * self.button_b.y) - (self.button_a.y * self.button_b.x);
       if denominator == 0 {
           return 0;
       }

       numerator / denominator
   }

    pub fn solve_for_t(&self, s: isize) -> isize {
        let numerator = self.target.x - self.button_a.x * s;
        let denominator =  self.button_b.x;

        if denominator == 0 {
            return 0;
        }

        numerator / denominator
    }
}


pub fn first_task() {
    let input = read_input("./input/13");

    let mut input_str: String = String::new();

    for l in input.iter() {
        if (l.len() == 0) {
            input_str.push('|');
            continue;
        }
        input_str.push_str(l);
        input_str.push(';')
    };


    let mut claw_machines = Vec::new();

    let claw_parts = input_str.split("|").collect::<Vec<&str>>();
    for cw in claw_parts.into_iter() {
        let claw_machine = cw.splitn(4, ';').collect::<Vec<&str>>();

        let button_a = Button::from(claw_machine[0]);
        let button_b = Button::from(claw_machine[1]);
        let target = Target::from(claw_machine[2]);

        let c = Clawmachine {
            button_a,
            button_b,
            target,
        };
        claw_machines.push(c)
    };

    // never would have figured this out https://www.youtube.com/watch?v=-5J-DAsWuJc
    let answer: isize =  claw_machines.into_iter().map(|c| {

        let s = c.solve_for_s();
        if(s == 0){
            return 0;
        }
        let t = c.solve_for_t(s);
        if(t == 0){
            return 0;
        }

        let traversed_distance = (s * c.button_a.x + t* c.button_b.x, s * c.button_a.y + t* c.button_b.y);
        // did not reach target
        if traversed_distance != (c.target.x, c.target.y){
            return 0;
        }
        // reached target -> calculate cost
        s*COST_A + t*COST_B
    }).sum();


    println!("Answer 1/2: {}", answer);
}

pub fn second_task() {
    let input = read_input("./input/13");

    let mut input_str: String = String::new();

    for l in input.iter() {
        if (l.len() == 0) {
            input_str.push('|');
            continue;
        }
        input_str.push_str(l);
        input_str.push(';')
    };


    let mut claw_machines = Vec::new();

    let claw_parts = input_str.split("|").collect::<Vec<&str>>();
    for cw in claw_parts.into_iter() {
        let claw_machine = cw.splitn(4, ';').collect::<Vec<&str>>();

        let button_a = Button::from(claw_machine[0]);
        let button_b = Button::from(claw_machine[1]);
        let target = Target::from_faulty_machine(claw_machine[2]);

        let c = Clawmachine {
            button_a,
            button_b,
            target,
        };
        claw_machines.push(c)
    };

    // never would have figured this out https://www.youtube.com/watch?v=-5J-DAsWuJc
    let answer: isize =  claw_machines.into_iter().map(|c| {

        let s = c.solve_for_s();
        if(s == 0){
            return 0;
        }
        let t = c.solve_for_t(s);
        if(t == 0){
            return 0;
        }

        let traversed_distance = (s * c.button_a.x + t* c.button_b.x, s * c.button_a.y + t* c.button_b.y);
        // did not reach target
        if traversed_distance != (c.target.x, c.target.y){
            return 0;
        }
        // reached target -> calculate cost
        s*COST_A + t*COST_B
    }).sum();


    println!("Answer 2/2: {}", answer);
}
