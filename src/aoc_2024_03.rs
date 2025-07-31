use std::fs::File;
use std::io;
use std::io::{BufRead, Seek};
use regex::Regex;

pub fn run_day3() -> io::Result<()> {
    let path = "input/2024_03.txt";
    let mut file = File::open(path)?;
    run_part1(&mut file)?;

    file.seek(io::SeekFrom::Start(0))?;

    run_part2(&mut file)?;

    Ok(())
}

fn run_part1(file: &mut File) -> io::Result<()> {
    let reader = io::BufReader::new(file);
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;
        for captures in re.captures_iter(line.as_str()) {
            let digit1 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let digit2 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

            result += digit1 * digit2;
        }
    }

    println!("Sum of multiplies: {}", result);
    Ok(())
}

fn run_part2(file: &mut File) -> io::Result<()> {
    let reader = io::BufReader::new(file);
    let control_re = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let data_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut regions: Vec<String> = Vec::new();
    let mut active = true;

    for line_result in reader.lines() {
        // active = true;
        let line = line_result?;
        let mut last_index = 0;
        for ctrl_match in control_re.find_iter(&line) {
            let segment = &line[last_index..ctrl_match.start()];
            if active {
                regions.push(segment.to_string());
            }

            active = ctrl_match.as_str() == "do()";
            last_index = ctrl_match.end();
        }

        if active {
            regions.push(line[last_index..].to_string());
        }
    }

    let mut result = 0;

    for region in regions {
        for captures in data_re.captures_iter(&*region) {
            let digit1 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let digit2 = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

            result += digit1 * digit2;
        }
    }

    println!("Sum of active multiplies: {}", result);
    Ok(())
}