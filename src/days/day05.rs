use std::ops::Range;

use crate::day::Day;
pub struct Day05;

impl Day for Day05 {
    fn solve_part_one(&self, input: &str) -> String {
        todo!()
    }

    fn solve_part_two(&self, input: &str) -> String {
        todo!()
    }
}

impl Day05 {
    fn parse_instruction(&self, input: &str) -> (Vec<Range<u32>>, Vec<u32>) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32";

    #[test]
    fn test_example_input_parsing() {
        todo!()
    }

    #[test]
    fn test_example_input_running_part1() {
        todo!()
    }

    #[test]
    fn test_example_input_running_part2() {
        todo!()
    }
}
