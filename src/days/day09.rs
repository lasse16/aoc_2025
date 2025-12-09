use std::any;

use crate::day::Day;
pub struct Day09;

type Point2D = (usize, usize);

impl Day for Day09 {
    fn solve_part_one(&self, input: &str) -> String {
        let points = self.parse_input(input);
        let max_area = self.find_max_area(points);
        format!("{}", max_area)
    }

    fn solve_part_two(&self, input: &str) -> String {
        todo!()
    }
}

impl Day09 {
    fn parse_input(&self, input: &str) -> Vec<Point2D> {
        let mut points = vec![];
        for line in input.lines() {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            points.push((x, y));
        }
        points
    }

    fn find_max_area(&self, points: Vec<(usize, usize)>) -> usize {
        let mut max_area = 0;

        for (i, a) in points.iter().enumerate() {
            for b in &points[i + 1..] {
                let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_example_input_parsing() {
        let known_points = [
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        let parsed_points = super::Day09.parse_input(EXAMPLE_INPUT);
        assert_eq!(parsed_points, known_points);
    }

    #[test]
    fn test_example_input_running_part1() {
        assert_eq!("50", Day09.solve_part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_example_input_running_part2() {
        todo!()
    }
}
