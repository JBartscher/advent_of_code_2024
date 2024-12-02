use crate::read_input;

pub fn first_task() {
    let input = read_input("./input/1");

    let mut first_list: Vec<i64> = Vec::new();
    let mut second_list: Vec<i64> = Vec::new();

    input.iter().for_each(|l| {
        let (a, b) = l.split_at(l.find(' ').unwrap());
        // println!("a:{:?} b:{:?}", a,b.trim());
        first_list.push(a.parse::<i64>().unwrap());
        second_list.push(b.trim().parse::<i64>().unwrap());
    });

    first_list.sort();
    second_list.sort();
    // second_list.reverse();

    let zipped = first_list.iter().zip(second_list.iter());

    let mut distance_sum = 0;

    for (first, second) in zipped {
        println!("{}-{}", first, second);
        let distance = first - second;
        let distance = distance.abs();
        distance_sum += distance;
    }

    println!("{}", distance_sum);
}

pub fn second_task() {
    let input = read_input("./input/1");

    let mut first_list: Vec<i64> = Vec::new();
    let mut second_list: Vec<i64> = Vec::new();

    input.iter().for_each(|l| {
        let (a, b) = l.split_at(l.find(' ').unwrap());
        // println!("a:{:?} b:{:?}", a,b.trim());
        first_list.push(a.parse::<i64>().unwrap());
        second_list.push(b.trim().parse::<i64>().unwrap());
    });

    let sum_occurrences = first_list.into_iter().map(|l| {
        let occurrences = second_list.iter().filter(|&r| l == *r).count() as i64;
        l * occurrences
    }).sum::<i64>();

    println!("{}", sum_occurrences);
}
