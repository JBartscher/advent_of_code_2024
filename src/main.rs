use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod day_one;
mod day_three;
mod day_two;
mod day_four;

mod day_five;
mod day_six;
mod day_fourteen;
mod day_thirteen;
mod day_eighteen;
mod day_sixteen;
mod day_nineteen;
mod day_twentytwo;

fn main() {
    println!("Day one");
    day_one::first_task();
    day_one::second_task();
    println!("Day two");
    day_two::first_task();
    day_two::second_task(); // 337
    println!("Day three");
    day_three::first_task();
    day_three::second_task();
    println!("Day four");
    day_four::first_task();
    day_four::second_task();
    println!("Day five");
    day_five::first_task();
    println!("Day six");
    day_six::first_task();
    day_six::second_task();
    println!("Day thirteen");
    day_thirteen::first_task();
    day_thirteen::second_task();
    println!("Day fourteen");
    day_fourteen::first_task();
    day_fourteen::second_task();
    println!("Day sixteen");
    day_sixteen::first_task();
    day_sixteen::second_task();
    println!("Day eighteen");
    day_eighteen::first_task();
    day_eighteen::second_task();
    println!("Day nineteen");
    day_nineteen::first_task();
    println!("Day twentytwo");
    day_twentytwo::first_task();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_input(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            // println!("{}", line);
            result.push(line);
        }
    }

    result
}
