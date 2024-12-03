use crate::read_input;
use regex::Regex;
use std::sync::LazyLock;

static TASK_ONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").expect("failed to compile regex"));

static TASK_TWO_REMOVE_DONT_BLOCKS: LazyLock<Regex> = LazyLock::new(|| {
    // Regex::new(r"don't\(\).*?do\(\)").expect("failed to compile regex")
    Regex::new(r"(don't\(\).*?do\(\))|(don't\(\).*)").expect("failed to compile regex")
});

pub fn first_task() {
    let binding = read_input("./input/3");
    let input = binding
        .into_iter()
        .reduce(|mut a, b| {
            a.push_str(&b);
            a
        })
        .unwrap();

    let mut results = vec![];

    for (_, [a, b]) in TASK_ONE_REGEX
        .captures_iter(input.as_str())
        .map(|cap| cap.extract())
    {
        results.push(a.parse::<i64>().unwrap() * (b.parse::<i64>().unwrap()));
    }

   // println!("{:?}", results);

    let answer: i64 = results.into_iter().sum();
    println!("Answer 1/2: {}", answer);
}

pub fn second_task() {
    let binding = read_input("./input/3");
    let input = binding
        .into_iter()
        .reduce(|mut a, b| {
            a.push_str(&b);
            a
        })
        .unwrap();

    let cleaned_text = TASK_TWO_REMOVE_DONT_BLOCKS.replace_all(input.as_str(), "").to_string();
    println!("{:?}", cleaned_text);

    let mut results = vec![];

    for (_, [a, b]) in TASK_ONE_REGEX
        .captures_iter(cleaned_text.as_str())
        .map(|cap| cap.extract())
    {
        results.push(a.parse::<i64>().unwrap() * (b.parse::<i64>().unwrap()));
    }

    let answer: i64 = results.into_iter().sum();
    println!("Answer 2/2: {}", answer);
}
