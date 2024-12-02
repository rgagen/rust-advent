use regex::Regex;
use std::fs;

enum ReportResult {
    Safe,
    Unsafe,
}

enum ReportDirection {
    Ascending,
    Descending,
}

struct Report {
    // report: Vec<u32>,
    // direction: ReportDirection,
    result: ReportResult,
}

impl Report {
    pub fn new(input: Vec<u32>) -> Report {
        if input.len() < 2 {
            panic!();
        }

        let direction: ReportDirection = if input[0] < input[1] {
            ReportDirection::Ascending
        } else {
            ReportDirection::Descending
        };

        let mut result = ReportResult::Safe;
        for i in 0..(input.len() - 1) {
            let diff = input[i] as i32 - input[i + 1] as i32;
            if diff.abs() < 1 || diff.abs() > 3 {
                result = ReportResult::Unsafe;
                break;
            }
            match direction {
                ReportDirection::Ascending => {
                    if diff > 0 {
                        result = ReportResult::Unsafe;
                        break;
                    }
                }
                ReportDirection::Descending => {
                    if diff < 0 {
                        result = ReportResult::Unsafe;
                        break;
                    }
                }
            }
        }

        return Report {
            // report: input,
            // direction: direction,
            result: result,
        };
    }
}

fn main() {
    println!("Advent of Code 2024 - Day 2");
    let file_path = "inputs/aoc2402.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("Day 2, Part 1: {}", part1(input.clone()));
    // println!("Day 2, Part 2: {}", part2(input.clone()));
}

fn part1(input: String) -> u32 {
    let mut num_safe: u32 = 0;
    for line in input.lines() {
        let report = Report::new(parse_line(line));
        num_safe += match report.result {
            ReportResult::Safe => 1,
            ReportResult::Unsafe => 0,
        }
    }
    return num_safe;
}

fn part2(input: String) -> u32 {
    return 1;
}

fn parse_line(line: &str) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut output: Vec<u32> = Vec::new();
    for (_, [digits]) in re.captures_iter(line).map(|c| c.extract()) {
        output.push(digits.parse::<u32>().unwrap())
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        let file_path = "test_inputs/aoc2402a.txt";
        return fs::read_to_string(file_path).unwrap();
    }

    #[test]
    fn part1_test_input() {
        let input = get_test_input();
        let result = part1(input);
        assert_eq!(result, 2)
    }

    #[test]
    fn part2_test_input() {
        let input = get_test_input();
        let result = part2(input);
        assert_eq!(result, 4)
    }
}
