use std::str::FromStr;
mod days;

use days::day::Day;
use days::*;
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide the day to run as a command-line argument.");
    }

    let day_number = u8::from_str(&args[1]).expect("Unparseable day given");

    let day = get_day(day_number);
    let input =
        fs::read_to_string(format!("inputs/{:02}.txt", day_number)).expect("Couldn't read file");

    let time = Instant::now();
    let mut total_runtime = 0.0;

    println!("\n=== Day {:02} ===", day_number);

    println!("Part 1: {}", day.solve_part_one(&input));
    let elapsed_ms = time.elapsed().as_millis() as f64;
    println!("  · Elapsed: {:.4} ms", elapsed_ms);

    total_runtime += elapsed_ms;

    let time = Instant::now();

    println!("Part 2: {}", day.solve_part_two(&input));
    let elapsed_ms = time.elapsed().as_millis() as f64;
    println!("  · Elapsed: {:.4} ms", elapsed_ms);

    total_runtime += elapsed_ms;
    println!("Total runtime: {}", total_runtime);
}

fn get_day(day: u8) -> Box<dyn Day> {
    match day {
        1 => Box::new(day01::Day01),
        2 => Box::new(day02::Day02),
        3 => Box::new(day03::Day03),
        4 => Box::new(day04::Day04),
        5 => Box::new(day05::Day05),
        6 => Box::new(day06::Day06),
        7 => Box::new(day07::Day07),
        8 => Box::new(day08::Day08),
        9 => Box::new(day09::Day09),
        10 => Box::new(day10::Day10),
        11 => Box::new(day11::Day11),
        _ => unimplemented!(),
    }
}
