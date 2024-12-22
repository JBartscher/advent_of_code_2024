use crate::read_input;

pub fn first_task() {
    let numbers: Vec<usize> = read_input("./input/22")
        .iter()
        .map(|l| {
            l.parse::<usize>().unwrap()
        }).collect();

    let result:usize = numbers.into_iter().map(|n| {
        do_2000_calculations(n, 2000)
    }).sum();

    println!("Answer 1/2: {}",result);
}

fn do_2000_calculations(number:usize, i: usize) -> usize {
    if i == 0 {
        return number;
    }
    let secret_number = calculate_secret_number(number);
    do_2000_calculations(secret_number, i - 1)
}

fn calculate_secret_number(number: usize) -> usize {
    third_step(second_step(first_step(number)))
}

/// Calculate the result of multiplying the secret number by 64.
/// Then, mix this result into the secret number.
/// Finally, prune the secret number.
fn first_step(secret_number: usize) -> usize {
    prune(mix(secret_number, secret_number * 64))
}
/// Calculate the result of dividing the secret number by 32.
/// Round the result down to the nearest integer.
/// Then, mix this result into the secret number.
/// Finally, prune the secret number.
fn second_step(secret_number: usize) -> usize {
    prune(mix(secret_number,  secret_number.div_euclid(32)))
}

/// Calculate the result of multiplying the secret number by 2048.
/// Then, mix this result into the secret number.
/// Finally, prune the secret number.
fn third_step(secret_number: usize) -> usize {
    prune(mix(secret_number,  secret_number * 2048))
}


fn mix(secret_number: usize, value: usize) -> usize {
    secret_number ^ value
}

fn prune(input: usize) -> usize {
    input.rem_euclid(16777216)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_first_task() {
        let numbers: Vec<usize> = read_input("./input/22_test")
            .iter()
            .map(|l| {
                l.parse::<usize>().unwrap()
            }).collect();

        let result:usize = numbers.into_iter().map(|n| {
            do_2000_calculations(n, 2000)
        }).sum();

        assert_eq!(result, 37327623);
    }


    #[test]
    fn test_do_2000_calculations() {
        assert_eq!(do_2000_calculations(1, 2000), 8685429);
        assert_eq!(do_2000_calculations(10, 2000), 4700978);
        assert_eq!(do_2000_calculations(100, 2000), 15273692);
        assert_eq!(do_2000_calculations(2024, 2000), 8667524);
    }

    #[test]
    fn test_calculate_secret_number() {
        assert_eq!(calculate_secret_number(123), 15887950);
        assert_eq!(calculate_secret_number(15887950), 16495136);
        assert_eq!(calculate_secret_number(16495136), 527345);
        assert_eq!(calculate_secret_number(527345), 704524);
        assert_eq!(calculate_secret_number(704524), 1553684);
        assert_eq!(calculate_secret_number(1553684), 12683156);
        assert_eq!(calculate_secret_number(12683156), 11100544);
        assert_eq!(calculate_secret_number(11100544), 12249484);
        assert_eq!(calculate_secret_number(12249484), 7753432);
        assert_eq!(calculate_secret_number(7753432), 5908254);
    }

    #[test]
    fn mix_number(){
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn prune_number(){
        assert_eq!(prune(100000000), 16113920);
    }
}