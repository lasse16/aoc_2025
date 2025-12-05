use crate::day::Day;
pub struct Day03;

impl Day for Day03 {
    fn solve_part_one(&self, input: &str) -> String {
        let banks = self.parse_instruction(input);
        let mut joltages: Vec<u64> = vec![];
        for bank in banks {
            joltages.push(find_nth_max_values(&bank, 2));
        }
        format!("{}", joltages.iter().sum::<u64>())
    }

    fn solve_part_two(&self, input: &str) -> String {
        let banks = self.parse_instruction(input);
        let mut joltages: Vec<u64> = vec![];
        for bank in banks {
            joltages.push(find_nth_max_values(&bank, 12));
        }
        format!("{}", joltages.iter().sum::<u64>())
    }
}

fn find_nth_max_values(bank: &[u8], digit_count: u8) -> u64 {
    let mut result: u64 = 0;
    let n = bank.len();
    let mut start_idx = 0;
    for i in (0..digit_count).rev() {
        let (max, updated_start_idx) = max_with_index(&bank[start_idx..n - i as usize]);
        start_idx += updated_start_idx + 1;
        result += max as u64 * 10_u64.pow(i as u32);
    }

    result
}

fn max_with_index(iter: &[u8]) -> (u8, usize) {
    let mut max = 0;
    let mut max_index = 0;
    for (i, value) in iter.iter().enumerate() {
        if *value > max {
            max = *value;
            max_index = i;
        }
    }
    (max, max_index)
}

impl Day03 {
    fn parse_instruction(&self, input: &str) -> Vec<Vec<u8>> {
        let mut result = vec![];
        for line in input.lines() {
            let joltages = line
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect();
            result.push(joltages);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
    const KNOWN_BANKS: [[u8; 15]; 4] = [
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
        [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
        [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
        [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
    ];

    #[test]
    fn test_example_input_parsing() {
        let parsed_example_input = Day03.parse_instruction(EXAMPLE_INPUT);
        for (expected, parsed) in KNOWN_BANKS.iter().zip(parsed_example_input) {
            assert_eq!(expected, parsed.as_slice());
        }
    }

    #[test]
    fn test_example_input_running_part1() {
        assert_eq!(Day03.solve_part_one(EXAMPLE_INPUT), "357");
    }

    #[test]
    fn test_example_input_running_part2() {
        assert_eq!(Day03.solve_part_two(EXAMPLE_INPUT), "3121910778619");
    }

    #[test]
    fn test_example_input_12_digit_max() {
        let known_max_values = [987654321111, 811111111119, 434234234278, 888911112111];
        for (bank, known_max) in KNOWN_BANKS.iter().zip(known_max_values) {
            assert_eq!(find_nth_max_values(bank, 12), known_max);
        }
    }
}
