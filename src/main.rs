use crate::aoc_2024_01::run_day1;
use crate::aoc_2024_02::run_day2;
use std::io;

mod aoc_2024_01;
mod aoc_2024_02;

fn main() -> io::Result<()> {
    run_day1()?;
    run_day2()?;

    Ok(())
}
