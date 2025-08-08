use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn run_day1() -> Result<()> {
    let (list1, list2) = read_input("input/2024_01.txt");
    let distance = get_distance_between_lists(&list1, &list2);

    println!("Total distance between lists: {}", distance);

    let similarity_score = get_similarity_score(&list1, &list2);

    println!("Similarity score: {}", similarity_score);

    Ok(())
}

fn read_input<P: AsRef<Path>>(path: P) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(Result::unwrap)
        .map(|line| {
            let line = line.split_once("   ").unwrap();
            (line.0.to_string(), line.1.to_string())
        })
        .fold((vec![], vec![]), |mut acc, x| {
            acc.0.push(x.0.parse().unwrap());
            acc.1.push(x.1.parse().unwrap());
            acc
        })
}

// O(n)
fn get_distance_between_lists(list1: &[i32], list2: &[i32]) -> i32 {
    let mut list1_clone = list1.to_vec();
    let mut list2_clone = list2.to_vec();
    list1_clone.sort();
    list2_clone.sort();

    let len = std::cmp::min(list1.len(), list2.len()); // Возможно лишнее. Сделал для безопасности
    let mut result: i32 = 0;

    for i in 0..len {
        result += (list1_clone[i] - list2_clone[i]).abs();
    }

    result
}

// O(2n)
fn get_similarity_score(list1: &[i32], list2: &[i32]) -> i32 {
    let mut scores: HashMap<i32, i32> = HashMap::new();
    let mut score: i32 = 0;

    // считаю в цикле количество вхождений чисел второго списка
    for &x in list2 {
        *scores.entry(x).or_insert(0) += 1;
    }

    // для чисел первого списка беру количество вхождений числа в правый список (0 при отсутствии) и
    // умножаю на само число.
    for &x in list1 {
        score += x * scores.get(&x).copied().unwrap_or(0);
    }

    score
}

#[test]
fn correct_part1() {
    let (list1, list2) = read_input("input/2024_01_test.txt");
    let distance = get_distance_between_lists(&list1, &list2);
    assert_eq!(distance, 11);
}

#[test]
fn correct_part2() {
    let (list1, list2) = read_input("input/2024_01_test.txt");
    let similarity_score = get_similarity_score(&list1, &list2);
    assert_eq!(similarity_score, 31);
}