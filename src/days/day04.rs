use crate::day::Day;
use std::collections::{HashMap, HashSet};
pub struct Day04;

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

type Position = (i32, i32);

impl Day for Day04 {
    fn solve_part_one(&self, input: &str) -> String {
        let live_cells = self.parse_input(input);
        let mut cell_counts: HashMap<Position, u8> = HashMap::new();

        for live in &live_cells {
            for (dx, dy) in &DIRS {
                *cell_counts.entry((live.0 + dx, live.1 + dy)).or_insert(0) += 1;
            }
        }

        let count = &live_cells
            .iter()
            .filter(|x| *cell_counts.get(x).unwrap_or(&0) < 4)
            .count();

        format!("{}", count)
    }
    fn solve_part_two(&self, input: &str) -> String {
        let mut live_cells = self.parse_input(input);
        let mut count: usize = 1;
        let mut removed = 0;

        while count != 0 {
            let mut cell_counts: HashMap<Position, u8> = HashMap::new();

            for live in &live_cells {
                for (dx, dy) in &DIRS {
                    *cell_counts.entry((live.0 + dx, live.1 + dy)).or_insert(0) += 1;
                }
            }

            let to_be_removed: Vec<Position> = live_cells
                .iter()
                .filter(|x| *cell_counts.get(x).unwrap_or(&0) < 4)
                .cloned()
                .collect();

            count = to_be_removed.len();

            for cell in &to_be_removed {
                live_cells.remove(cell);
            }

            removed += count;
        }
        format!("{}", removed)
    }
}

impl Day04 {
    fn parse_input(&self, input: &str) -> HashSet<Position> {
        let mut result = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                if value == '@' {
                    result.insert((x as i32, y as i32));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_example_input_parsing() {
        let known_cells = HashSet::from([
            (4, 7),
            (8, 9),
            (9, 4),
            (7, 5),
            (1, 4),
            (2, 5),
            (2, 7),
            (6, 1),
            (3, 7),
            (3, 8),
            (7, 0),
            (2, 0),
            (4, 9),
            (0, 3),
            (8, 1),
            (6, 2),
            (7, 6),
            (3, 0),
            (3, 6),
            (0, 9),
            (4, 4),
            (6, 0),
            (0, 2),
            (2, 2),
            (9, 2),
            (2, 3),
            (9, 1),
            (1, 5),
            (8, 6),
            (2, 9),
            (4, 5),
            (9, 6),
            (8, 7),
            (3, 4),
            (5, 3),
            (0, 7),
            (4, 2),
            (5, 4),
            (9, 7),
            (4, 1),
            (5, 0),
            (1, 6),
            (2, 8),
            (3, 5),
            (6, 9),
            (8, 4),
            (6, 8),
            (0, 4),
            (3, 2),
            (1, 1),
            (5, 6),
            (4, 3),
            (8, 3),
            (7, 7),
            (1, 8),
            (7, 8),
            (8, 8),
            (8, 2),
            (5, 8),
            (3, 3),
            (2, 1),
            (1, 2),
            (6, 4),
            (4, 8),
            (6, 7),
            (6, 5),
            (5, 5),
            (9, 5),
            (5, 9),
            (0, 1),
            (8, 0),
        ]);
        assert_eq!(Day04.parse_input(EXAMPLE_INPUT), known_cells);
    }

    #[test]
    fn test_example_input_running_part1() {
        assert_eq!(Day04.solve_part_one(EXAMPLE_INPUT), "13");
    }

    #[test]
    fn test_example_input_running_part2() {
        assert_eq!(Day04.solve_part_two(EXAMPLE_INPUT), "43");
    }
}
