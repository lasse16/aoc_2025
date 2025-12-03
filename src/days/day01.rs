use crate::day::Day;
pub struct Day01;

#[derive(PartialEq, Eq, Debug)]
enum Op {
    Add,
    Sub,
}
impl Day for Day01 {
    fn solve_part_one(&self, input: &str) -> String {
        let instructions = input.lines().map(|line| self.parse_instruction(line));
        let mut current_value = 50;
        let mut counter = 0;
        for instruction in instructions {
            self.apply_instruction(&mut current_value, &mut counter, instruction);
        }

        format!("{}", counter)
    }

    fn solve_part_two(&self, input: &str) -> String {
        todo!()
    }
}

impl Day01 {
    fn parse_instruction(&self, instruction: &str) -> (Op, u16) {
        let mut chars = instruction.chars();
        let operator = chars.next().unwrap();

        let op = match operator {
            'L' => Op::Sub,
            'R' => Op::Add,
            _ => unimplemented!(),
        };
        let value = chars
            .collect::<String>()
            .parse::<u16>()
            .expect("Could not parse remaining string to digit");
        (op, value)
    }

    fn apply_instruction(&self, current_value: &mut u8, counter: &mut i32, instruction: (Op, u16)) {
        let (op, value) = instruction;
        let value = (value % 100) as u8;
        *current_value = match op {
            Op::Add => (*current_value + value) % 100,
            Op::Sub => (*current_value + 100 - value) % 100,
        };

        if *current_value == 0 {
            *counter += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use std::iter::zip;

    use super::*;
    const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_simple_instruction_parsing() {
        assert_eq!((Op::Sub, 1), Day01.parse_instruction("L1"));
    }

    #[test]
    fn test_apply_instruction_addition() {
        let mut counter = 0;
        let mut value = 5;

        Day01.apply_instruction(&mut value, &mut counter, (Op::Add, 4));

        assert_eq!(value, 9);
    }

    #[test]
    fn test_apply_instruction_addition_across_zero() {
        let mut counter = 0;
        let mut value = 5;

        Day01.apply_instruction(&mut value, &mut counter, (Op::Add, 99));

        assert_eq!(value, 4);
    }

    #[test]
    fn test_apply_instruction_addition_onto_zero() {
        let mut counter = 0;
        let mut value = 5;

        Day01.apply_instruction(&mut value, &mut counter, (Op::Add, 95));

        assert_eq!(counter, 1);
    }

    #[test]
    fn test_apply_instruction_subtraction() {
        let mut counter = 0;
        let mut value = 5;

        Day01.apply_instruction(&mut value, &mut counter, (Op::Sub, 4));

        assert_eq!(value, 1);
    }

    #[test]
    fn test_apply_instruction_subtraction_across_zero() {
        let mut counter = 0;
        let mut value = 5;

        Day01.apply_instruction(&mut value, &mut counter, (Op::Sub, 6));

        assert_eq!(value, 99);
    }

    #[test]
    fn test_apply_instruction_subtraction_onto_zero() {
        let mut counter = 0;
        let mut value = 5;

        Day01.apply_instruction(&mut value, &mut counter, (Op::Sub, 5));

        assert_eq!(counter, 1);
    }

    #[test]
    fn test_example_input_parsing() {
        let instructions = EXAMPLE_INPUT
            .lines()
            .map(|line| Day01.parse_instruction(line))
            .collect::<Vec<(Op, u8)>>();
        assert_eq!(
            instructions,
            vec![
                (Op::Sub, 68),
                (Op::Sub, 30),
                (Op::Add, 48),
                (Op::Sub, 5),
                (Op::Add, 60),
                (Op::Sub, 55),
                (Op::Sub, 1),
                (Op::Sub, 99),
                (Op::Add, 14),
                (Op::Sub, 82),
            ]
        );
    }

    #[test]
    fn test_example_input_running() {
        let instructions = [
            (Op::Sub, 68),
            (Op::Sub, 30),
            (Op::Add, 48),
            (Op::Sub, 5),
            (Op::Add, 60),
            (Op::Sub, 55),
            (Op::Sub, 1),
            (Op::Sub, 99),
            (Op::Add, 14),
            (Op::Sub, 82),
        ];
        let mut value = 50;
        let mut counter = 0;
        let expected_values = [82, 52, 0, 95, 55, 0, 99, 0, 14, 32];

        for (instruction, expected_value) in zip(instructions, expected_values) {
            Day01.apply_instruction(&mut value, &mut counter, instruction);
            assert_eq!(value, expected_value);
        }

        assert_eq!(3, counter);
    }
}
