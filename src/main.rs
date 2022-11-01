use std::fs;

mod day2;
mod utils;
mod day3;
mod day4;

fn main() {
    let input = fs::read_to_string("./src/day4/input.txt").unwrap();

    let result = day4::part_1(&input);
    println!("Day 4, Part 1: {}", result);

//    let result = day3::part_2(&input);
//    println!("Day 3, Part 2: {}", result);

    // let result = day2::part_2(&input);
    // println!("Day 2, Part 2: {}", result)
}
