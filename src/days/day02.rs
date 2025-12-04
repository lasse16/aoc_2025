use std::ops::{ControlFlow, Range};

use crate::day::Day;
pub struct Day02;

impl Day for Day02 {
    fn solve_part_one(&self, input: &str) -> String {
        let mut invalids: Vec<u64> = vec![];
        let ranges = input
            .split(",")
            .map(|range_| self.parse_instruction(range_.trim()));
        for range_ in ranges {
            for number in range_ {
                if is_invalid_number(number) {
                    invalids.push(number);
                }
            }
        }
        invalids.iter().sum::<u64>().to_string()
    }

    fn solve_part_two(&self, input: &str) -> String {
        todo!()
    }
}

fn is_invalid_number(number: u64) -> bool {
    let string = number.to_string();
    let string = string.as_bytes();
    let n = string.len();
    if n % 2 != 0 {
        return false;
    }
    let mut idx1 = 0;
    let mut idx2 = n / 2;
    while idx2 < n && string[idx1] == string[idx2] {
        idx1 += 1;
        idx2 += 1;
    }
    idx2 >= n
}

impl Day02 {
    fn parse_instruction(&self, instruction: &str) -> Range<u64> {
        let mut ends = instruction
            .split("-")
            .map(|x| x.parse::<u64>().expect("Could not parse number for range"));
        let start = ends.next().unwrap();
        let end = ends.next().unwrap();

        start..end + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_simple_instruction_parsing() {
        assert_eq!(11..23, Day02.parse_instruction("11-22"));
    }

    #[test]
    fn test_simple_invalid_checking() {
        let known_invalids = [11, 22, 99, 1010, 1188511885, 222222];
        for invalid in known_invalids {
            assert!(is_invalid_number(invalid))
        }
    }

    #[test]
    fn test_example_input_parsing() {
        let ranges: Vec<Range<u64>> = EXAMPLE_INPUT
            .split(",")
            .map(|range_| Day02.parse_instruction(range_))
            .collect();
        assert_eq!(
            ranges,
            [
                11..23,
                95..116,
                998..1013,
                1188511880..1188511891,
                222220..222225,
                1698522..1698529,
                446443..446450,
                38593856..38593863,
                565653..565660,
                824824821..824824828,
                2121212118..2121212125
            ]
        );
    }

    #[test]
    fn test_example_input_running_part1() {
        assert_eq!(Day02.solve_part_one(EXAMPLE_INPUT),"1227775554");
    }
}
