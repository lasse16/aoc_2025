use std::{cmp, ops::Range};

use crate::day::Day;
pub struct Day05;

impl Day for Day05 {
    fn solve_part_one(&self, input: &str) -> String {
        let (ranges, ids) = Day05.parse_input(input);
        let count = ids
            .iter()
            .filter(|id| ranges.iter().any(|range| range.contains(id)))
            .count();
        format!("{}", count)
    }

    fn solve_part_two(&self, input: &str) -> String {
        let mut non_overlapping_ranges: Vec<Range<u64>> = vec![];
        let (mut ranges, _) = Day05.parse_input(input);
        ranges.sort_unstable_by(|x, y| x.start.cmp(&y.start));

        let mut prev_interval = ranges[0].clone();
        for current_interval in ranges.iter().skip(1) {
            if current_interval.start < prev_interval.end {
                let new_end = cmp::max(current_interval.end, prev_interval.end);
                prev_interval = prev_interval.start..new_end;
            } else {
                non_overlapping_ranges.push(prev_interval);
                prev_interval = current_interval.clone();
            }
        }
        non_overlapping_ranges.push(prev_interval);

        let count = non_overlapping_ranges
            .iter()
            .fold(0, |acc, range| acc + (range.end - range.start));
        format!("{}", count)
    }
}

impl Day05 {
    fn parse_input(&self, input: &str) -> (Vec<Range<u64>>, Vec<u64>) {
        let mut ranges = vec![];
        let mut ids = vec![];
        let (raw_ranges, raw_ids) = input
            .split_once("\n\n")
            .expect("No blankline found in input string");

        for mut line in raw_ranges.lines() {
            line = line.trim();
            let (start, end) = line.split_once("-").unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            ranges.push(start..end + 1);
        }

        for mut line in raw_ids.lines() {
            line = line.trim();
            let id: u64 = line.parse().unwrap();
            ids.push(id);
        }

        (ranges, ids)
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
        let known_ranges = [3..6, 10..15, 16..21, 12..19];
        let known_ids = [1, 5, 8, 11, 17, 32];
        let (parsed_ranges, parsed_ids) = Day05.parse_input(EXAMPLE_INPUT);
        assert_eq!(known_ranges, parsed_ranges.as_slice());
        assert_eq!(known_ids, parsed_ids.as_slice());
    }

    #[test]
    fn test_example_input_running_part1() {
        assert_eq!(Day05.solve_part_one(EXAMPLE_INPUT), "3");
    }

    #[test]
    fn test_example_input_running_part2() {
        assert_eq!(Day05.solve_part_two(EXAMPLE_INPUT), "14");
    }
}
