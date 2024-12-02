use crate::read_input;

pub fn first_task() {
    let input = read_input("./input/2");

    let mut valid = 0;
    input.iter().for_each(|x| {
        let report = x
            .split(' ')
            .map(|y| y.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if is_steadily_declining(&report) || is_steadily_climbing(&report) {
            valid = valid + 1;
        }
    });

    println!("Answer 1/2: {}", valid);
}

pub fn second_task() {
    let input = read_input("./input/2");

    let mut valid = 0;
    input.iter().for_each(|x| {
        let report = x
            .split(' ')
            .map(|y| y.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if is_steadily_declining_with_one_strike_allowed(&report)
            || is_steadily_climbing_with_one_strike_allowed(&report)
        {
            valid = valid + 1;
            println!("{:?}", &report);
        } else {
            //
        }
    });

    println!("Answer 2/2: {}", valid);
}

fn is_steadily_declining(report: &[i64]) -> bool {
    let iter = report.iter().enumerate();

    for (i, n) in iter.skip(1) {
        let b = report.get(i - 1).unwrap();
        if n >= b || b - n > 3 {
            return false;
        }
    }

    true
}

fn is_steadily_declining_with_one_strike_allowed(report: &[i64]) -> bool {
    let iter = report.iter().enumerate();

    for (i, n) in iter.skip(1) {
        let b = report.get(i - 1).unwrap();
        if n >= b || b - n > 3 {
            let mut vec_report: Vec<i64> = report.iter().cloned().collect();
            vec_report.remove(i);
            return is_steadily_declining(&vec_report);
        }
    }

    true
}
fn is_steadily_climbing(report: &[i64]) -> bool {
    let iter = report.iter().enumerate();

    for (i, n) in iter.skip(1) {
        let b = report.get(i - 1).unwrap();
        if b >= n || n - b > 3 {
            return false;
        }
    }

    true
}
fn is_steadily_climbing_with_one_strike_allowed(report: &[i64]) -> bool {
    let iter = report.iter().enumerate();

    for (i, n) in iter.skip(1) {
        let b = report.get(i - 1).unwrap();
        if b >= n || n - b > 3 {
            let mut vec_report: Vec<i64> = report.iter().cloned().collect();
            vec_report.remove(i);
            return is_steadily_climbing(&vec_report);
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use crate::day_two::is_steadily_climbing;
    use crate::day_two::{
        is_steadily_climbing_with_one_strike_allowed, is_steadily_declining,
        is_steadily_declining_with_one_strike_allowed,
    };

    #[test]
    fn is_steadily_declining_works() {
        assert_eq!(is_steadily_declining(&[4, 3, 2, 1]), true);
        assert_eq!(is_steadily_declining(&[4, 4, 2, 1]), false);
        assert_eq!(is_steadily_declining(&[1, 2, 3, 4]), false);
        assert_eq!(is_steadily_declining(&[7, 3, 2, 1]), false);
    }

    #[test]
    fn is_steadily_declining_works_with_one_strike_allowed_works() {
        assert_eq!(
            is_steadily_declining_with_one_strike_allowed(&[4, 3, 2, 1]),
            true
        );
        assert_eq!(
            is_steadily_declining_with_one_strike_allowed(&[4, 4, 2, 1]),
            true
        );
        assert_eq!(
            is_steadily_declining_with_one_strike_allowed(&[1, 2, 3, 4]),
            false
        );
        assert_eq!(
            is_steadily_declining_with_one_strike_allowed(&[11, 7, 3, 2, 1]),
            false
        );
        assert_eq!(
            is_steadily_declining_with_one_strike_allowed(&[7, 6, 4, 2, 1]),
            true
        );
        assert_eq!(
            is_steadily_declining_with_one_strike_allowed(&[9, 7, 6, 2, 1]),
            false
        );
        assert_eq!(
            is_steadily_declining_with_one_strike_allowed(&[8, 6, 4, 4, 1]),
            true
        );
        assert_eq!(
            is_steadily_climbing_with_one_strike_allowed(&[14, 18, 19, 20, 20, 27]),
            false
        );
    }

    #[test]
    fn is_steadily_climbing_works() {
        assert_eq!(is_steadily_climbing(&[1, 2, 3, 4]), true);
        assert_eq!(is_steadily_climbing(&[4, 4, 2, 1]), false);
        assert_eq!(is_steadily_climbing(&[1, 2, 6, 7]), false);
        assert_eq!(is_steadily_climbing(&[1, 1, 2, 3]), false);
    }

    #[test]
    fn is_steadily_climbing_with_one_strike_allowed_works() {
        assert_eq!(
            is_steadily_climbing_with_one_strike_allowed(&[1, 2, 3, 4]),
            true
        );
        assert_eq!(
            is_steadily_climbing_with_one_strike_allowed(&[2, 4, 6, 6]),
            true
        );
        assert_eq!(
            is_steadily_climbing_with_one_strike_allowed(&[1, 2, 6, 7]),
            false
        );
        assert_eq!(
            is_steadily_climbing_with_one_strike_allowed(&[1, 1, 2, 3]),
            true
        );
        assert_eq!(
            is_steadily_climbing_with_one_strike_allowed(&[1, 2, 7, 8, 9]),
            false
        );
        assert_eq!(
            is_steadily_climbing_with_one_strike_allowed(&[1, 3, 2, 4, 5]),
            true
        );
        assert_eq!(
            is_steadily_climbing_with_one_strike_allowed(&[1, 3, 6, 7, 9]),
            true
        );
        assert_eq!(
            is_steadily_climbing_with_one_strike_allowed(&[14, 18, 19, 20, 20, 27]),
            false
        );
    }
}
