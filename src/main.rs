use crate::aoc_2024_01::run_day1;
use crate::aoc_2024_02::run_day2;
use crate::aoc_2024_03::run_day3;
use crate::aoc_2024_04::run_day4;
use std::io;

mod aoc_2024_01;
mod aoc_2024_02;
mod aoc_2024_03;
mod aoc_2024_04;

fn main() -> io::Result<()> {
    run_day1()?;
    run_day2()?;
    run_day3()?;
    run_day4()?;

    Ok(())
}
