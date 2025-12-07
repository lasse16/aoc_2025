use crate::day::Day;
pub struct Day06;

type Problem = (u64, Op);

#[derive(PartialEq, Debug)]
enum Op {
    Add,
    Mul,
}

impl Day for Day06 {
    fn solve_part_one(&self, input: &str) -> String {
        let problems = Day06.parse_input(input);
        let sum: u64 = problems.iter().map(|(acc, _)| acc).sum();
        format!("{}", sum)
    }

    fn solve_part_two(&self, input: &str) -> String {
        let problems = Day06.parse_input_part_two(input);
        let sum: u64 = problems.iter().sum();
        format!("{}", sum)
    }
}

impl Day06 {
    fn parse_input(&self, input: &str) -> Vec<Problem> {
        let mut result = vec![];

        let mut lines = input.lines().rev();

        let operator_line = lines.next().unwrap().trim();
        for char in operator_line.chars() {
            if !char.is_whitespace() {
                match char {
                    '+' => result.push((0, Op::Add)),
                    '*' => result.push((1, Op::Mul)),
                    _ => panic!("Unexpected operation character"),
                }
            }
        }

        for line in lines {
            let mut active_number: u64 = 0;
            let mut current_column = 0;
            for char in line.trim().chars() {
                if char.is_ascii_digit() {
                    active_number = active_number * 10 + char.to_digit(10).unwrap() as u64
                } else if active_number != 0 {
                    let (acc, operation) = result.get_mut(current_column).unwrap();
                    match operation {
                        Op::Add => *acc += active_number,
                        Op::Mul => *acc *= active_number,
                    }
                    active_number = 0;
                    current_column += 1;
                }
            }
            let (acc, operation) = result.get_mut(current_column).unwrap();
            match operation {
                Op::Add => *acc += active_number,
                Op::Mul => *acc *= active_number,
            }
        }
        result
    }

    fn parse_input_part_two(&self, input: &str) -> Vec<u64> {
        let mut result = vec![];

        let mut lines = input.lines().rev();
        let mut operators = vec![];

        //IMPORTANT: All operators are always left-aligned in their columns
        let operator_line = lines.next().unwrap().chars();
        let mut line_length = 0;
        for (idx, char) in operator_line.enumerate() {
            if !char.is_whitespace() {
                match char {
                    '+' => operators.push((idx, Op::Add)),
                    '*' => operators.push((idx, Op::Mul)),
                    _ => panic!("Unexpected operation character"),
                }
            }
            line_length += 1;
        }
        line_length += 1;

        let mut numbers: Vec<u64> = vec![0; line_length];

        for line in lines.rev() {
            for (idx, char) in line.chars().enumerate() {
                let active_number = numbers.get_mut(idx).unwrap();
                if char.is_ascii_digit() {
                    *active_number = *active_number * 10 + char.to_digit(10).unwrap() as u64
                }
            }
        }

        for (i, (idx, operator)) in operators.iter().enumerate() {
            let next_operator_index = match operators.get(i + 1) {
                Some(next_operator) => next_operator.0,
                None => line_length,
            };
            match operator {
                Op::Add => {
                    result.push(numbers[*idx..next_operator_index].iter().sum());
                }
                Op::Mul => {
                    let numbers = &numbers[*idx..next_operator_index];
                    let mut acc = 1;
                    for number in numbers {
                        // All fields without numbers are zero in my parsing
                        // There are no "true" zeros in the input
                        if *number != 0 {
                            acc *= number
                        }
                    }

                    result.push(acc);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_example_input_parsing() {
        let known_accumulators = [
            (33210, Op::Mul),
            (490, Op::Add),
            (4243455, Op::Mul),
            (401, Op::Add),
        ];
        assert_eq!(
            known_accumulators,
            Day06.parse_input(EXAMPLE_INPUT).as_slice()
        )
    }

    #[test]
    fn test_example_input_running_part1() {
        assert_eq!(Day06.solve_part_one(EXAMPLE_INPUT), "4277556");
    }

    #[test]
    fn test_example_input_running_part2() {
        assert_eq!(Day06.solve_part_two(EXAMPLE_INPUT), "3263827");
    }
}
