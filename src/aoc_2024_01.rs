use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn run_day1() -> io::Result<()> {
    let (list1, list2) = read_input("input/2024_01.txt")?;
    let distance = get_distance_between_lists(list1.clone(), list2.clone());

    println!("Total distance between lists: {}", distance);

    let similarity_score = get_similarity_score(list1, list2);

    println!("Similarity score: {}", similarity_score);

    Ok(())
}

fn read_input<P: AsRef<Path>>(path: P) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let re = Regex::new(r"\s+").unwrap();

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = re.split(&line).collect();
        if parts.len() == 2 {
            if let Ok(n1) = parts[0].trim().parse::<i32>() {
                list1.push(n1);
            }
            if let Ok(n2) = parts[1].trim().parse::<i32>() {
                list2.push(n2);
            }
        }
    }
    Ok((list1, list2))
}

// O(n)
fn get_distance_between_lists(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    list1.sort();
    list2.sort();

    let len = std::cmp::min(list1.len(), list2.len()); // Возможно лишнее. Сделал для безопасности
    let mut result: i32 = 0;

    for i in 0..len {
        result += (list1[i] - list2[i]).abs();
    }

    result
}

// O(2n)
fn get_similarity_score(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let mut scores: HashMap<i32, i32> = HashMap::new();
    let mut score: i32 = 0;

    for x in list2 {
        *scores.entry(x).or_insert(0) += 1;
    }

    for x in list1 {
        score += x * scores.get(&x).copied().unwrap_or(0);
    }

    score
}
