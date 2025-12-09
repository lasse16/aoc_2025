use crate::day::Day;
use disjoint::DisjointSet;
use std::{fmt::Display, num::ParseIntError, str::FromStr};

pub struct Day08;

#[derive(Debug, PartialEq)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}
impl Point {
    fn squared_distance(&self, b: &Point) -> u64 {
        self.x.abs_diff(b.x).pow(2) + self.y.abs_diff(b.y).pow(2) + self.z.abs_diff(b.z).pow(2)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl PartialEq<(u64, u64, u64)> for Point {
    fn eq(&self, other: &(u64, u64, u64)) -> bool {
        let (x, y, z) = other;
        self.x == *x && self.y == *y && self.z == *z
    }
}

impl From<(u64, u64, u64)> for Point {
    fn from(value: (u64, u64, u64)) -> Self {
        let (x, y, z) = value;
        Point { x, y, z }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(",").map(u64::from_str);

        let x = values.next().unwrap()?;
        let y = values.next().unwrap()?;
        let z = values.next().unwrap()?;

        Ok(Point { x, y, z })
    }
}

impl Day for Day08 {
    fn solve_part_one(&self, input: &str) -> String {
        let points = self.parse_input(input);
        let mut vertices = DisjointSet::with_len(points.len());
        let mut pairs_with_distance = self.generate_pairs_with_distance(&points);
        pairs_with_distance.sort_unstable_by_key(|x| x.0);

        for i in 0..1000 {
            let active_edge = pairs_with_distance[i].1;
            let first_index = points.iter().position(|p| p == active_edge.0).unwrap();
            let second_index = points.iter().position(|p| p == active_edge.1).unwrap();

            if !vertices.is_joined(first_index, second_index) {
                vertices.join(first_index, second_index);
            }
        }
        let mut circuits = vertices.sets();
        circuits.sort_unstable_by_key(|set| set.len());
        format!(
            "{}",
            circuits
                .iter()
                .rev()
                .take(3)
                .map(|x| x.len())
                .product::<usize>()
        )
    }

    fn solve_part_two(&self, input: &str) -> String {
        let points = self.parse_input(input);
        let mut vertices = DisjointSet::with_len(points.len());
        let mut pairs_with_distance = self.generate_pairs_with_distance(&points);
        pairs_with_distance.sort_unstable_by_key(|x| x.0);
        let mut circuits = vertices.sets();
        let mut counter = 0;
        let mut latest_edge = pairs_with_distance[0].1;

        while circuits.len() != 1 {
            let active_edge = pairs_with_distance[counter].1;
            let first_index = points.iter().position(|p| p == active_edge.0).unwrap();
            let second_index = points.iter().position(|p| p == active_edge.1).unwrap();

            if !vertices.is_joined(first_index, second_index) {
                vertices.join(first_index, second_index);
            }

            counter += 1;
            circuits = vertices.sets();
            latest_edge = active_edge;
        }

        format!("{}", latest_edge.0.x * latest_edge.1.x)
    }
}

impl Day08 {
    fn parse_input(&self, input: &str) -> Vec<Point> {
        input
            .lines()
            .map(|line| Point::from_str(line).unwrap())
            .collect()
    }

    fn generate_pairs_with_distance<'a>(
        &self,
        points: &'a [Point],
    ) -> Vec<(u64, (&'a Point, &'a Point))> {
        let mut result = vec![];
        for (i, a) in points.iter().enumerate() {
            for b in &points[i + 1..] {
                let distance = a.squared_distance(b);
                result.push((distance, (a, b)));
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    const KNOWN_POINTS: [(u64, u64, u64); 20] = [
        (162, 817, 812),
        (57, 618, 57),
        (906, 360, 560),
        (592, 479, 940),
        (352, 342, 300),
        (466, 668, 158),
        (542, 29, 236),
        (431, 825, 988),
        (739, 650, 466),
        (52, 470, 668),
        (216, 146, 977),
        (819, 987, 18),
        (117, 168, 530),
        (805, 96, 715),
        (346, 949, 466),
        (970, 615, 88),
        (941, 993, 340),
        (862, 61, 35),
        (984, 92, 344),
        (425, 690, 689),
    ];

    #[test]
    fn test_example_input_parsing() {
        assert_eq!(Day08.parse_input(EXAMPLE_INPUT), KNOWN_POINTS);
    }

    #[test]
    fn test_example_input_sorting() {
        let points = &KNOWN_POINTS.map(Point::from);
        let known_pairs = [
            (&points[0], &points[19]),
            (&points[0], &points[7]),
            (&points[2], &points[13]),
            (&points[7], &points[19]),
        ];
        let mut values = Day08.generate_pairs_with_distance(points);
        values.sort_unstable_by_key(|x| x.0);
        for (value, pair) in values.iter().take(4).map(|x| x.1).zip(known_pairs) {
            assert_eq!(value, pair)
        }
    }

    // This requires a iteration count of 10, not 1000 as the actual input
    // #[test]
    // fn test_example_input_running_part1() {
    //     assert_eq!("40", Day08.solve_part_one(EXAMPLE_INPUT));
    // }

    #[test]
    fn test_example_input_running_part2() {
        assert_eq!("25272", Day08.solve_part_two(EXAMPLE_INPUT));
    }
}
