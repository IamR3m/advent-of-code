use crate::aoc_2024_01::run_day1;
use crate::aoc_2024_02::run_day2;
use crate::aoc_2024_03::run_day3;
use crate::aoc_2024_04::run_day4;
use crate::aoc_2024_05::run_day5;
use std::io::Result;

mod aoc_2024_01;
mod aoc_2024_02;
mod aoc_2024_03;
mod aoc_2024_04;
mod aoc_2024_05;

fn main() -> Result<()> {
    println!("day 1");
    run_day1()?;
    println!("\nday 2");
    run_day2()?;
    println!("\nday 3");
    run_day3()?;
    println!("\nday 4");
    run_day4()?;
    println!("\nday 5");
    run_day5()?;

    Ok(())
}
