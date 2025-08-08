use crate::util;
use std::io::Result;

pub fn run_day6() -> Result<()> {
    let path = "input/2024_06_.txt";
    let map = util::read_char_matrix(path)?;

    let guard_position = get_guard_position(&map);
    // println!("Guard position is {:?}", guard_position);
    let (x, y, direction) = guard_position;

    let (start_x, start_y, end_x, end_y) = run_part1(&map, &x, &y, &direction);

    run_part2(&map, &x, &y, &direction, start_x, start_y, end_x, end_y);

    Ok(())
}

fn run_part1(
    map: &Vec<Vec<char>>,
    x: &usize,
    y: &usize,
    direction: &char,
) -> (usize, usize, usize, usize) {
    let mut map_cloned = map.clone();
    do_guard_moves(
        &mut map_cloned,
        &mut (x.clone() as isize),
        &mut (y.clone() as isize),
        &mut direction.clone(),
        false,
    );

    let (count, start_x, start_y, end_x, end_y) = count_guard_positions(&map_cloned);

    println!("Guard positions count: {}", count);

    (start_x, start_y, end_x, end_y)
}

fn run_part2(
    map: &Vec<Vec<char>>,
    x: &usize,
    y: &usize,
    direction: &char,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) {
    let mut solutions = 0;

    for i in start_y..end_y {
        for j in start_x..end_x {
            if (i == *y && j == *x)
                || map[i][j] == '#'
                || (*direction == '^' && i == y - 1 && j == *x)
                || (*direction == 'v' && i == y + 1 && j == *x)
                || (*direction == '<' && i == *y && j == x - 1)
                || (*direction == '>' && i == *y && j == x + 1)
            {
                continue;
            }
            let mut cloned_map = map.clone();
            cloned_map[i][j] = 'O';

            if do_guard_moves(
                &mut cloned_map,
                &mut (x.clone() as isize),
                &mut (y.clone() as isize),
                &mut direction.clone(),
                true,
            ) {
                solutions += 1
            };
        }
    }

    println!("Possible solutions with guard looped: {}", solutions);
}

fn do_guard_moves(
    map: &mut [Vec<char>],
    x: &mut isize,
    y: &mut isize,
    direction: &mut char,
    is_catch_cycle: bool,
) -> bool {
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    while *x >= 0 && *x < width && *y >= 0 && *y < height {
        if is_catch_cycle && is_cycled(&map, *x as usize, *y as usize, *direction) {
            print_map(&map);
            return true;
        }
        if can_step(&map, *x as usize, *y as usize, *direction) {
            do_step(map, x, y, *direction, is_catch_cycle, false);
        } else {
            turn_right(map, *x as usize, *y as usize, direction);
            do_step(map, x, y, *direction, is_catch_cycle, true);
        }
    }
    // if is_catch_cycle { print_map(&map); }

    false
}

fn get_guard_position(map: &[Vec<char>]) -> (usize, usize, char) {
    let height = map.len();
    let width = map[0].len();

    let guard = ['^', '>', 'v', '<'];
    for i in 0..height {
        for j in 0..width {
            if guard.contains(&map[i][j]) {
                return (j, i, map[i][j]);
            }
        }
    }

    unreachable!("Guard should be on the map")
}

fn can_step(map: &[Vec<char>], x: usize, y: usize, direction: char) -> bool {
    let height = map.len();
    let width = map[0].len();

    let obstacles = ['#', 'O'];
    match direction {
        '^' => {
            if y > 0 && obstacles.contains(&map[y - 1][x]) {
                return false;
            }
        }
        '>' => {
            if x < width - 1 && obstacles.contains(&map[y][x + 1]) {
                return false;
            }
        }
        'v' => {
            if y < height - 1 && obstacles.contains(&map[y + 1][x]) {
                return false;
            }
        }
        '<' => {
            if x > 0 && obstacles.contains(&map[y][x - 1]) {
                return false;
            }
        }
        _ => panic!(),
    }

    true
}

fn do_step(
    map: &mut [Vec<char>],
    x: &mut isize,
    y: &mut isize,
    direction: char,
    is_catch_cycle: bool,
    is_turned: bool,
) {
    map[*y as usize][*x as usize] = if is_catch_cycle && is_turned {
        '+'
    } else {
        if is_catch_cycle {
            match direction {
                '^' => '|',
                '>' => '-',
                'v' => '|',
                '<' => '-',
                _ => panic!(),
            }
        } else {
            'X'
        }
    };
    match direction {
        '^' => *y -= 1,
        '>' => *x += 1,
        'v' => *y += 1,
        '<' => *x -= 1,
        _ => panic!(),
    }
    if *x >= 0 && *x < map[0].len() as isize && *y >= 0 && *y < map.len() as isize {
        map[*y as usize][*x as usize] = direction;
    }
}

fn turn_right(map: &mut [Vec<char>], x: usize, y: usize, direction: &mut char) {
    match direction {
        '^' => *direction = '>',
        '>' => *direction = 'v',
        'v' => *direction = '<',
        '<' => *direction = '^',
        _ => panic!(),
    }
    map[y][x] = *direction;
}

fn count_guard_positions(map: &[Vec<char>]) -> (i32, usize, usize, usize, usize) {
    let height = map.len();
    let width = map[0].len();
    let mut min_x: usize = width;
    let mut min_y: usize = height;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    let mut count = 0;
    for i in 0..height {
        for j in 0..width {
            if map[i][j] == 'X' {
                count += 1;
                min_x = std::cmp::min(min_x, j);
                min_y = std::cmp::min(min_y, i);
                max_x = std::cmp::max(max_x, j);
                max_y = std::cmp::max(max_y, i);
            }
        }
    }

    min_x = std::cmp::max(min_x - 1, 0);
    min_y = std::cmp::max(min_y - 1, 0);
    max_x = std::cmp::min(max_x + 1, width);
    max_y = std::cmp::min(max_y + 1, height);

    (count, min_x, min_y, max_x, max_y)
}

fn is_cycled(map: &[Vec<char>], x: usize, y: usize, direction: char) -> bool {
    match direction {
        '^' => {
            if y == 0 {
                return false;
            }
            if map[y - 1][x] == '|' {
                return true;
            }
        }
        '>' => {
            if x + 1 == map[0].len() {
                return false;
            }
            if map[y][x + 1] == '-' {
                return true;
            }
        }
        'v' => {
            if y + 1 == map.len() {
                return false;
            }
            if map[y + 1][x] == '|' {
                return true;
            }
        }
        '<' => {
            if x == 0 {
                return false;
            }
            if map[y][x - 1] == '-' {
                return true;
            }
        }
        _ => panic!(),
    }

    false
}

fn print_map(map: &[Vec<char>]) {
    // println!();
    print!("\x1B[2J\x1B[1;1H");
    for i in 0..map.len() {
        println!("{}", map[i].iter().collect::<String>());
    }
    println!();
}
