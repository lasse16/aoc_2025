use std::collections::HashMap;

use crate::day::Day;
pub struct Day11;

impl Day for Day11 {
    fn solve_part_one(&self, input: &str) -> String {
        let mut cache: HashMap<&str, usize> = HashMap::new();
        let start = "you";
        let end = "out";

        let edges = self.parse_input(input);
        let path_count = dfs(&edges, start, end, &mut cache);

        format!("{}", path_count)
    }

    fn solve_part_two(&self, input: &str) -> String {
        // Caching is specific to the end node
        let mut cache_out: HashMap<&str, usize> = HashMap::new();
        let mut cache_fft: HashMap<&str, usize> = HashMap::new();
        let mut cache_dac: HashMap<&str, usize> = HashMap::new();

        let start = "svr";
        let end = "out";

        let mut edges = self.parse_input(input);
        // "out" never appears on the left side and as such is never generated in the input parsing
        edges.insert("out", vec![]);

        let paths_dac_first = dfs(&edges, start, "dac", &mut cache_dac)
            * dfs(&edges, "dac", "fft", &mut cache_fft)
            * dfs(&edges, "fft", end, &mut cache_out);
        let paths_fft_first = dfs(&edges, start, "fft", &mut cache_fft)
            * dfs(&edges, "fft", "dac", &mut cache_dac)
            * dfs(&edges, "dac", end, &mut cache_out);

        format!("{}", paths_dac_first + paths_fft_first)
    }
}

fn dfs<'a>(
    edges: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(cached_value) = cache.get(start) {
        return *cached_value;
    }

    if start == end {
        return 1;
    }

    let count = edges[start]
        .iter()
        .map(|output| dfs(edges, output, end, cache))
        .sum();
    cache.insert(start, count);

    count
}

impl Day11 {
    fn parse_input<'a>(&self, input: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
        let mut result = HashMap::new();
        for line in input.lines() {
            let (name, outputs_raw) = line.split_once(":").unwrap();
            let output_devices: Vec<_> = outputs_raw.split_whitespace().collect();
            result.insert(name, output_devices);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    const EXAMPLE_INPUT_PART2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_example_input_parsing() {
        let known_edges = HashMap::from([
            ("aaa", vec!["you", "hhh"]),
            ("fff", vec!["out"]),
            ("iii", vec!["out"]),
            ("bbb", vec!["ddd", "eee"]),
            ("ccc", vec!["ddd", "eee", "fff"]),
            ("ddd", vec!["ggg"]),
            ("eee", vec!["out"]),
            ("hhh", vec!["ccc", "fff", "iii"]),
            ("you", vec!["bbb", "ccc"]),
            ("ggg", vec!["out"]),
        ]);
        assert_eq!(known_edges, Day11.parse_input(EXAMPLE_INPUT))
    }

    #[test]
    fn test_example_input_running_part1() {
        assert_eq!("5", Day11.solve_part_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_example_input_running_part2() {
        assert_eq!("2", Day11.solve_part_two(EXAMPLE_INPUT_PART2));
    }
}
