use regex::Regex;
use std::collections::HashMap;
use std::fs;

mod aoc2401 {
    fn main() {
        println!("Advent of Code 2024 - Day 1");
        let file_path = "inputs/aoc2401.txt";
        let input = fs::read_to_string(file_path).unwrap();

        println!("Day 1, Part 1: {}", part1(input.clone()));
        println!("Day 1, Part 2: {}", part2(input.clone()));
    }

    fn part1(input: String) -> u32 {
        let (mut x, mut y) = parse_lines(input);
        x.sort();
        y.sort();

        let mut diffs: Vec<u32> = Vec::new();
        for i in 0..x.len() {
            diffs.push(x[i].abs_diff(y[i]))
        }

        let output = diffs.iter().fold(0u32, |sum, i| sum + i);
        return output;
    }

    fn part2(input: String) -> u32 {
        let (x, y) = parse_lines(input);
        let mut y_dict: HashMap<u32, u32> = HashMap::new();
        for i in y {
            let count = y_dict.entry(i).or_insert(0);
            *count += 1;
        }
        let mut output: Vec<u32> = Vec::new();
        for i in x {
            output.push(i * y_dict.get(&i).unwrap_or(&0))
        }
        return output.iter().fold(0u32, |sum, i| sum + i);
    }

    fn parse_line(line: &str) -> [u32; 2] {
        let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let output: [u32; 2] = [caps[1].parse().unwrap(), caps[2].parse().unwrap()];
        return output;
    }

    fn parse_lines(input: String) -> (Vec<u32>, Vec<u32>) {
        let mut x: Vec<u32> = Vec::new();
        let mut y: Vec<u32> = Vec::new();
        for line in input.lines() {
            let [next_x, next_y] = parse_line(line);
            x.push(next_x);
            y.push(next_y);
        }
        return (x, y);
    }
}

#[cfg(test)]
mod aoc2401_tests {
    use super::aoc2401::*;

    fn get_test_input() -> String {
        let file_path = "test_inputs/aoc2401a.txt";
        return fs::read_to_string(file_path).unwrap();
    }

    #[test]
    fn part1_test_input() {
        let input = get_test_input();
        let result = part1(input);
        assert_eq!(result, 11)
    }

    #[test]
    fn part2_test_input() {
        let input = get_test_input();
        let result = part2(input);
        assert_eq!(result, 31)
    }
}
