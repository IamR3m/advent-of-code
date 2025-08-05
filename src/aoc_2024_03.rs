use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Seek, SeekFrom};

pub fn run_day3() -> Result<()> {
    let path = "input/2024_03.txt";
    let mut file = File::open(path)?;
    let sum_of_multiplies = get_sum_of_multiplies(&mut file)?;
    println!("Sum of multiplies: {}", sum_of_multiplies);

    file.seek(SeekFrom::Start(0))?;

    let sum_of_active_multiplies = get_sum_of_active_multiplies(&mut file)?;
    println!("Sum of active multiplies: {}", sum_of_active_multiplies);

    Ok(())
}

fn get_sum_of_multiplies(file: &mut File) -> Result<i32> {
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;
        result += sum_multiplies(&line);
    }

    Ok(result)
}

fn get_sum_of_active_multiplies(file: &mut File) -> Result<i32> {
    let reader = BufReader::new(file);
    let control_re = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let mut active = true;
    let mut result = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let mut last_index = 0;
        for ctrl_match in control_re.find_iter(&line) {
            let segment = &line[last_index..ctrl_match.start()];
            if active {
                result += sum_multiplies(&segment.to_string());
            }

            active = ctrl_match.as_str() == "do()";
            last_index = ctrl_match.end();
        }

        if active {
            result += sum_multiplies(&line[last_index..].to_string());
        }
    }

    Ok(result)
}

fn sum_multiplies(str: &String) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for captures in re.captures_iter(str) {
        let digit1 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let digit2 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

        sum += digit1 * digit2;
    }

    sum
}
