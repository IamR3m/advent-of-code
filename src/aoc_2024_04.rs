use crate::util::read_char_matrix;
use std::io::Result;

pub fn run_day4() -> Result<()> {
    let content = read_char_matrix("input/2024_04.txt")?;

    let xmas_count = count_xmas_appearing(&content);
    println!("XMAS appears {} time(s)", xmas_count);

    let x_mas_count = count_x_mas_appearing(&content);
    println!("X-MAS appears {} time(s)", x_mas_count);

    Ok(())
}

fn count_xmas_appearing(content: &[Vec<char>]) -> usize {
    let mut counter = 0;

    // считаем горизонтальные строки
    counter += content
        .iter()
        .map(|line| count_patterns(line))
        .sum::<usize>();

    // считаем колонки
    counter += transpose(&content)
        .iter()
        .map(|line| count_patterns(line))
        .sum::<usize>();

    let (left_diag, right_diag) = get_diagonals(content);

    // считаем левую диагональ
    counter += left_diag
        .iter()
        .map(|line| count_patterns(line))
        .sum::<usize>();
    // считаем правую диагональ
    counter += right_diag
        .iter()
        .map(|line| count_patterns(line))
        .sum::<usize>();

    counter
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

fn count_x_mas_appearing(content: &[Vec<char>]) -> i32 {
    let rows = content.len();
    let cols = content.get(0).map_or(0, |r| r.len());

    let mut counter = 0;

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            if content[i][j] != 'A' {
                continue;
            }
            let nw = content[i - 1][j - 1];
            let ne = content[i - 1][j + 1];
            let sw = content[i + 1][j - 1];
            let se = content[i + 1][j + 1];

            let diag1 = (nw == 'M' && se == 'S') || (se == 'M' && nw == 'S');
            let diag2 = (sw == 'M' && ne == 'S') || (ne == 'M' && sw == 'S');
            if diag1 && diag2 {
                counter += 1;
            }
        }
    }
    counter
}
