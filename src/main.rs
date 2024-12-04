use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod day_one;
mod day_three;
mod day_two;
mod day_four;

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
