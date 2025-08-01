use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn run_day5() -> io::Result<()> {
    let path = "input/2024_05.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut rules: Vec<(i32, i32)> = vec![];
    let mut sum = 0;

    for line in &mut lines {
        let line = line?;
        if line.trim().is_empty() {
            break;
        }
        rules.push(parse_pair(&line));
    }

    for line in lines {
        let line = line?;
        let update = line
            .split(',')
            .map(str::trim)
            .map(|p| p.parse().expect("parse error"))
            .collect::<Vec<i32>>();
        println!("update: {:?}", update);
        if validate_update(&rules, &update) {
            let med = update.len() / 2;
            println!("med: {}", update[med]);
            sum += update[med];
        }
    }

    println!("Sum of middle page numbers {}", sum);
    Ok(())
}

fn parse_pair(str: &str) -> (i32, i32) {
    let mut parts = str.split('|').map(str::trim);
    (
        parts
            .next()
            .expect("missing first number")
            .parse()
            .expect("invalid first number"),
        parts
            .next()
            .expect("missing second number")
            .parse()
            .expect("invalid second number"),
    )
}

fn validate_update(rules: &[(i32, i32)], update: &[i32]) -> bool {
    for (page1, page2) in rules {
        if let Some(page1_position) = update
            .iter()
            .position(|page| page == page1) {
            let page2_position = update
                .iter()
                .position(|page| page == page2)
                .map(|i| i as isize)
                .unwrap_or(-1);

            if page2_position == -1 { continue; };
            if page1_position as isize > page2_position {
                println!("== falsy ==");
                return false;
            }
        }
    }

    true
}
