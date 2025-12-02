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

    let input = fs::read_to_string("inputs/01.txt").expect("Couldn't read file");

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
    let matched_day = match day {
        1 => day01::Day01,
        _ => unimplemented!(),
    };

    Box::new(matched_day)
}
