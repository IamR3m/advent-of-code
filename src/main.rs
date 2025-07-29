use crate::aoc_2024_01::f2024_01;

mod aoc_2024_01;

fn main() {
    println!("Hello, world!");
    let r2024_01 = f2024_01(&mut [3, 4, 2, 1, 3, 3], &mut [4, 3, 5, 3, 9, 3]);
    println!("{}", r2024_01);
}
