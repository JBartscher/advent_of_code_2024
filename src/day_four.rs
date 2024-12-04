use crate::read_input;

const MIN: usize = 0;
const MAX: usize = 140;

pub fn first_task() {
    let mut rows: Vec<Vec<char>> = Vec::new();

    let input = read_input("./input/4");
    input.iter().for_each(|l| {
        let row = l.chars().collect::<Vec<char>>();
        rows.push(row);
    });

    let mut sum_of_xmas = 0;

    for y in MIN..MAX {
        for x in MIN..MAX {
            sum_of_xmas = sum_of_xmas + check_vertical(x, y, &rows);
            sum_of_xmas = sum_of_xmas + check_horizontal(x, y, &rows);
            sum_of_xmas = sum_of_xmas + check_diagonals(x, y, &rows);
        }
        print!("\n");
    }

    println!("Answer 1/2: {}", sum_of_xmas); // > 1523 > 2153 < 2542
}

fn check_diagonals(x: usize, y: usize, rows: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    match rows[y][x] {
        'X' => {
            // ↖
            if (x >= 3 && y >= 3)
                && (rows[y - 1][x - 1] == 'M'
                    && rows[y - 2][x - 2] == 'A'
                    && rows[y - 3][x - 3] == 'S')
            {
                count += 1;
            }
            // ↗
            if (MAX - x > 3 && y >= 3)
                && (rows[y - 1][x + 1] == 'M'
                    && rows[y - 2][x + 2] == 'A'
                    && rows[y - 3][x + 3] == 'S')
            {
                count += 1;
            }
            // ↙
            if (x >= 3 && MAX - y > 3)
                && (rows[y + 1][x - 1] == 'M'
                    && rows[y + 2][x - 2] == 'A'
                    && rows[y + 3][x - 3] == 'S')
            {
                count += 1;
            }
            // ↘
            if (MAX - x > 3 && MAX - y > 3)
                && (rows[y + 1][x + 1] == 'M'
                    && rows[y + 2][x + 2] == 'A'
                    && rows[y + 3][x + 3] == 'S')
            {
                count += 1;
            }
        }
        _ => {}
    }
    count
}

fn check_horizontal(x: usize, y: usize, rows: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    match rows[y][x] {
        'X' => {
            // →
            if (MAX - x > 3)
                && (rows[y][x + 1] == 'M' && rows[y][x + 2] == 'A' && rows[y][x + 3] == 'S')
            {
                count = count + 1;
            }
            // ←
            if (x >= 3) && (rows[y][x - 1] == 'M' && rows[y][x - 2] == 'A' && rows[y][x - 3] == 'S')
            {
                count = count + 1;
            }
        }
        _ => {}
    }
    count
}

fn check_vertical(x: usize, y: usize, rows: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    match rows[y][x] {
        'X' => {
            // ↑
            if (y >= 3) && (rows[y - 1][x] == 'M' && rows[y - 2][x] == 'A' && rows[y - 3][x] == 'S')
            {
                count = count + 1;
            }
            // ↓
            if (MAX - y > 3)
                && (rows[y + 1][x] == 'M' && rows[y + 2][x] == 'A' && rows[y + 3][x] == 'S')
            {
                count = count + 1;
            }
        }
        _ => {}
    }
    count
}

pub fn second_task() {
    let mut rows: Vec<Vec<char>> = Vec::new();

    let input = read_input("./input/4");
    input.iter().for_each(|l| {
        let row = l.chars().collect::<Vec<char>>();
        rows.push(row);
    });

    let mut sum_of_x_mas = 0;

    for y in MIN..MAX {
        for x in MIN..MAX {
            sum_of_x_mas = sum_of_x_mas + check_x_mas(x, y, &rows);
        }
        print!("\n");
    }

    println!("Answer 2/2: {}", sum_of_x_mas);
}

fn check_x_mas(x: usize, y: usize, rows: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    match rows[y][x] {
        'A' => {
            // ↑ MM ↓ SS
            if ((x >= 1 && MAX - x > 1) && (y >= 1 && MAX - y > 1))
                && (rows[y - 1][x - 1] == 'M'
                    && rows[y - 1][x + 1] == 'M'
                    && rows[y + 1][x - 1] == 'S'
                    && rows[y + 1][x + 1] == 'S')
            {
                count += 1;
            }
            // ↑ SS ↓ MM
            if ((x >= 1 && MAX - x > 1) && (y >= 1 && MAX - y > 1))
                && (rows[y - 1][x - 1] == 'S'
                && rows[y - 1][x + 1] == 'S'
                && rows[y + 1][x - 1] == 'M'
                && rows[y + 1][x + 1] == 'M')
            {
                count += 1;
            }
            // → MM ← SS
            if ((x >= 1 && MAX - x > 1) && (y >= 1 && MAX - y > 1))
                && (rows[y - 1][x - 1] == 'S'
                && rows[y - 1][x + 1] == 'M'
                && rows[y + 1][x - 1] == 'S'
                && rows[y + 1][x + 1] == 'M')
            {
                count += 1;
            }
            // → SS ← MM
            if ((x >= 1 && MAX - x > 1) && (y >= 1 && MAX - y > 1))
                && (rows[y - 1][x - 1] == 'M'
                && rows[y - 1][x + 1] == 'S'
                && rows[y + 1][x - 1] == 'M'
                && rows[y + 1][x + 1] == 'S')
            {
                count += 1;
            }
        }
        _ => {}
    }
    count
}
