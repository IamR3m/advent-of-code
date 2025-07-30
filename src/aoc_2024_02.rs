use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run_day2() -> io::Result<()> {
    let reports = read_input("input/2024_02.txt")?;
    let mut counter = 0;

    for report in reports.clone() {
        if is_report_safe(report, None) {
            counter += 1;
        }
    }

    println!("Safe reports: {}", counter);

    counter = 0;
    for report in reports {
        if is_report_safe(report.clone(), None) {
            counter += 1;
        } else {
            if is_report_safe_with_problem_dampener(report) {
                counter += 1;
            }
        }
    }

    println!("Safe reports with dampener: {}", counter);

    Ok(())
}

fn read_input<P: AsRef<Path>>(path: P) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let re = Regex::new(r"\s+").unwrap();

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let levels: Vec<i32> = re
            .split(&line)
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        reports.push(levels);
    }

    Ok(reports)
}

// Худший сценарий O(n). Присутствует early exit
fn is_report_safe(levels: Vec<i32>, index_to_skip: Option<usize>) -> bool {
    let mut maybe_is_increasing: Option<bool> = None;
    for i in 0..levels.len() - 1 {
        let mut delta = 1;
        if index_to_skip.is_some() {
            if i == index_to_skip.unwrap() {
                continue;
            }
            if i + 1 == index_to_skip.unwrap() {
                delta += 1;
            }
        }
        if i + delta >= levels.len() {
            continue;
        }
        let diff = levels[i + delta] - levels[i];
        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        if let Some(is_increasing) = maybe_is_increasing {
            if is_increasing && diff < 0 {
                return false;
            }
            if !is_increasing && diff > 0 {
                return false;
            }
        } else {
            maybe_is_increasing = Some(diff > 0);
        }
    }

    true
}

// Худший сценарий O(n^2). Средний O(k * n), k - количество проверок до успеха
fn is_report_safe_with_problem_dampener(levels: Vec<i32>) -> bool {
    let mut is_safe = false;
    let mut index: usize = 0;

    while !is_safe && index < levels.len() {
        is_safe = is_report_safe(levels.clone(), Some(index));

        index += 1;
    }

    is_safe
}
