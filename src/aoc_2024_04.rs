use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run_day4() -> io::Result<()> {
    let content = read_input("input/2024_04.txt")?;

    run_part1(&content);
    run_part2(&content);

    Ok(())
}

fn read_input<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line_result| {
            line_result.map(|line| line.chars().collect::<Vec<char>>())
        })
        .collect()
}

fn run_part1(content: &[Vec<char>]) {
    let mut counter = 0;

    // считаем горизонтальные строки
    counter += content.iter().map(|line| count_patterns(line)).sum::<usize>();

    // считаем колонки
    counter += transpose(&content).iter().map(|line| count_patterns(line)).sum::<usize>();

    let (left_diag, right_diag) = get_diagonals(content);

    // считаем левую диагональ
    counter += left_diag.iter().map(|line| count_patterns(line)).sum::<usize>();
    // считаем правую диагональ
    counter += right_diag.iter().map(|line| count_patterns(line)).sum::<usize>();

    println!("XMAS appears {} time(s)", counter);
}

fn run_part2(content: &[Vec<char>]) {
    let rows = content.len();
    let cols = content.get(0).map_or(0, |r| r.len());

    let mut counter = 0;

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            if content[i][j] != 'A' { continue; }
            let nw = content[i - 1][j - 1];
            let ne = content[i - 1][j + 1];
            let sw = content[i + 1][j - 1];
            let se = content[i + 1][j + 1];

            let diag1 = (nw == 'M' && se == 'S') || (se == 'M' && nw == 'S');
            let diag2 = (sw == 'M' && ne == 'S') || (ne == 'M' && sw == 'S');
            if diag1 && diag2 { counter += 1; }
        }
    }

    println!("X-MAS appears {} time(s)", counter);
}

fn count_patterns(line: &[char]) -> usize {
    let s: String = line.into_iter().collect();
    s.match_indices("XMAS").count() + s.match_indices("SAMX").count()
}

fn transpose(content: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..content[0].len())
        .map(|i| content.iter().map(|row| row[i]).collect())
        .collect()
}

fn get_diagonals(content: &[Vec<char>]) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let rows = content.len();
    let cols = content.get(0).map_or(0, |r| r.len());

    let mut left = vec![];
    let mut right = vec![];

    for d in 0..(rows + cols - 1) {
        let mut diag = vec![];
        for i in 0..=d {
            let x = i;
            let y = d - i;
            if x < rows && y < cols {
                diag.push(content[x][y]);
            }
        }

        left.push(diag);
    }

    for d in -(cols as isize - 1)..(rows as isize) {
        let mut diag = vec![];
        for i in 0..rows {
            let y = (i as isize - d) as usize;
            if y < cols {
                diag.push(content[i][y]);
            }
        }
        right.push(diag);
    }

    (left, right)
}