use crate::read_input;

const MIN: usize = 0;
const MAX: usize = 140;

pub fn first_task() {
    let mut array_2d: [[char; MAX]; MAX];

    let mut rows: Vec<Vec<char>> = Vec::new();

    let input = read_input("./input/4");
    input.iter().for_each(|l| {
        let row = l.chars().collect::<Vec<char>>();
        rows.push(row);
    });

    let mut sum_of_xmas = 0;

    for y in MIN..MAX {
        for x in MIN..MAX {
            print!("{}", rows[y][x]);
            if check_vertical(x, y, &rows) {
                sum_of_xmas = sum_of_xmas + 1;
            }
            if check_horizontal(x, y, &rows) {
                sum_of_xmas = sum_of_xmas + 1;
            }
            if check_diagonals(x, y, &rows) {
                sum_of_xmas = sum_of_xmas + 1;
            }
        }
        print!("\n");
    }

    println!("Answer 1/2: {}", sum_of_xmas); // > 1523 < 2542
}

fn check_diagonals(x: usize, y: usize, rows: &Vec<Vec<char>>) -> bool {
    match rows[y][x] {
        // 'S' => {
        //     // ↖
        //     if (x >= 3 && y >= 3)
        //         && (rows[y - 1][x - 1] == 'A'
        //             && rows[y - 2][x - 2] == 'M'
        //             && rows[y - 3][x - 3] == 'X')
        //     {
        //         return true;
        //     }
        //     // ↗
        //     if (MAX - x > 3 && y >= 3)
        //         && (rows[y - 1][x + 1] == 'A'
        //             && rows[y - 2][x + 2] == 'M'
        //             && rows[y - 3][x + 3] == 'X')
        //     {
        //         return true;
        //     }
        //     // ↙
        //     if (x >= 3 && MAX - y > 3)
        //         && (rows[y + 1][x - 1] == 'A'
        //             && rows[y + 2][x - 2] == 'M'
        //             && rows[y + 3][x - 3] == 'X')
        //     {
        //         return true;
        //     }
        //     // ↘
        //     if (MAX - x > 3 && MAX - y > 3)
        //         && (rows[y + 1][x + 1] == 'A'
        //             && rows[y + 2][x + 2] == 'M'
        //             && rows[y + 3][x + 3] == 'X')
        //     {
        //         return true;
        //     }
        //     false
        // }
        'X' => {
            // ↖
            if (x >= 3 && y >= 3)
                && (rows[y - 1][x - 1] == 'M'
                    && rows[y - 2][x - 2] == 'A'
                    && rows[y - 3][x - 3] == 'S')
            {
                return true;
            }
            // ↗
            if (MAX - x > 3 && y >= 3)
                && (rows[y - 1][x + 1] == 'M'
                    && rows[y - 2][x + 2] == 'A'
                    && rows[y - 3][x + 3] == 'S')
            {
                return true;
            }
            // ↙
            if (x >= 3 && MAX - y > 3)
                && (rows[y + 1][x - 1] == 'M'
                    && rows[y + 2][x - 2] == 'A'
                    && rows[y + 3][x - 3] == 'S')
            {
                return true;
            }
            // ↘
            if (MAX - x > 3 && MAX - y > 3)
                && (rows[y + 1][x + 1] == 'M'
                    && rows[y + 2][x + 2] == 'A'
                    && rows[y + 3][x + 3] == 'S')
            {
                return true;
            }
            false
        }
        _ => false,
    }
}

fn check_horizontal(x: usize, y: usize, rows: &Vec<Vec<char>>) -> bool {
    match rows[y][x] {
        'X' => {
            // →
            if (MAX - x > 3)
                && (rows[y][x + 1] == 'M' && rows[y][x + 2] == 'A' && rows[y][x + 3] == 'S')
            {
                return true;
            }
            // ←
            if (x >= 3) && (rows[y][x - 1] == 'M' && rows[y][x - 2] == 'A' && rows[y][x - 3] == 'S')
            {
                return true;
            }
            false
        }
        // 'S' => {
        //     // →
        //     if (MAX - x > 3)
        //         && (rows[y][x + 1] == 'A' && rows[y][x + 2] == 'M' && rows[y][x + 3] == 'X')
        //     {
        //         return true;
        //     }
        //     // ←
        //     if (x >= 3) && (rows[y][x - 1] == 'A' && rows[y][x - 2] == 'M' && rows[y][x - 3] == 'X')
        //     {
        //         return true;
        //     }
        //     false
        // }
        _ => false,
    }
}

fn check_vertical(x: usize, y: usize, rows: &Vec<Vec<char>>) -> bool {
    match rows[y][x] {
        'X' => {
            // ↑
            if (y >= 3) && (rows[y - 1][x] == 'M' && rows[y - 2][x] == 'A' && rows[y - 3][x] == 'S')
            {
                return true;
            }
            // ↓
            if (MAX - y > 3)
                && (rows[y + 1][x] == 'M' && rows[y + 2][x] == 'A' && rows[y + 3][x] == 'S')
            {
                return true;
            }
            false
        }
        'S' => {
            // ↑
            if (y >= 3) && (rows[y - 1][x] == 'A' && rows[y - 2][x] == 'M' && rows[y - 3][x] == 'X')
            {
                return true;
            }
            // ↓
            if (MAX - y > 3)
                && (rows[y + 1][x] == 'A' && rows[y + 2][x] == 'M' && rows[y + 3][x] == 'X')
            {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn second_task() {
    println!("Answer 2/2: {}", 0);
}
