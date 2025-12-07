pub struct Day07;
use std::collections::HashSet;

use crate::day::Day;

type Point2D = (usize, usize);

impl Day for Day07 {
    fn solve_part_one(&self, input: &str) -> String {
        let (start, splitters, line_count) = self.parse_input(input);
        let mut splitters_per_column = Vec::with_capacity(splitters.len());

        for column in splitters {
            let splitters_x: Vec<usize> = column.iter().map(|x| x.0).collect();
            splitters_per_column.push(splitters_x);
        }

        let mut laser_locations = HashSet::from([start.1]);
        let mut laser_split_counter = 0;

        for i in 0..line_count {
            let mut laser_locations_next_line = HashSet::new();
            for column in laser_locations.iter() {
                let splitters = splitters_per_column.get(*column).unwrap();
                if splitters.iter().any(|line| *line == i) {
                    laser_locations_next_line.insert(column - 1);
                    laser_locations_next_line.insert(column + 1);
                    laser_split_counter += 1;
                } else {
                    laser_locations_next_line.insert(*column);
                }
            }
            laser_locations = laser_locations_next_line;
        }
        format!("{}", laser_split_counter)
    }

    fn solve_part_two(&self, input: &str) -> String {
        todo!()
    }
}

impl Day07 {
    fn parse_input(&self, input: &str) -> (Point2D, Vec<Vec<Point2D>>, usize) {
        let mut lines = input.lines();
        let start_line = lines.next().unwrap();
        // ASSUMING ASCII, this works as chars().len()
        let line_length = start_line.len();
        let start_column = start_line.chars().position(|x| x == 'S').unwrap();
        let mut splitters_per_column = vec![vec![]; line_length];

        let mut line_count = 0;
        for (line_number, line) in lines.enumerate() {
            for (column, char) in line.chars().enumerate() {
                if char == '^' {
                    splitters_per_column[column].push((line_number, column))
                }
            }
            line_count += 1;
        }
        ((0, start_column), splitters_per_column, line_count + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_example_input_parsing() {
        let known_splitters = [
            vec![],
            vec![(13, 1)],
            vec![(11, 2)],
            vec![(9, 3), (13, 3)],
            vec![(7, 4)],
            vec![(5, 5), (9, 5), (13, 5)],
            vec![(3, 6), (7, 6), (11, 6)],
            vec![(1, 7), (5, 7), (13, 7)],
            vec![(3, 8)],
            vec![(5, 9), (9, 9), (13, 9)],
            vec![(7, 10)],
            vec![(9, 11)],
            vec![(11, 12)],
            vec![(13, 13)],
            vec![],
        ];
        let known_start = (0, 7);
        let known_line_length = 16;

        let (parsed_start, parsed_splitters, parsed_line_length) = Day07.parse_input(EXAMPLE_INPUT);
        assert_eq!(parsed_start, known_start);
        assert_eq!(parsed_line_length, known_line_length);
        assert_eq!(parsed_splitters, known_splitters);
    }

    #[test]
    fn test_example_input_running_part1() {
        assert_eq!("21", Day07.solve_part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_example_input_running_part2() {
        todo!()
    }
}
