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

fn main() {
    println!("Advent of Code 2024 - Day 2");
    let file_path = "inputs/aoc2402.txt";
    let input = fs::read_to_string(file_path).unwrap();

    println!("Day 2, Part 1: {}", part1(&input));
    println!("Day 2, Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut safe_reports = 0;
    for line in input.lines() {
        let report = parse_line(line);
        match process_report(&report) {
            ReportResult::Safe => safe_reports += 1,
            ReportResult::Unsafe => continue,
        }
    }
    return safe_reports;
}

fn part2(input: &str) -> i32 {
    let mut safe_reports = 0;
    for line in input.lines() {
        let report = parse_line(line);
        match process_report(&report) {
            ReportResult::Safe => {
                safe_reports += 1;
            }
            ReportResult::Unsafe => {
                for i in 0..report.len() {
                    let mut shortened_report: Vec<i32> = Vec::new();
                    for j in 0..report.len() {
                        if i == j {
                            continue;
                        };
                        shortened_report.push(report[j])
                    }
                    match process_report(&shortened_report) {
                        ReportResult::Safe => {
                            safe_reports += 1;
                            break;
                        }
                        ReportResult::Unsafe => {}
                    }
                }
            }
        }
    }
    return safe_reports;
}

fn process_report(report: &Vec<i32>) -> ReportResult {
    let mut direction: Option<ReportDirection> = None;
    for i in 0..(report.len() - 1) {
        let diff = report[i] - report[i + 1];
        if (diff.abs() > 3) || (diff.abs() < 1) {
            return ReportResult::Unsafe;
        }
        match direction {
            None => {
                direction = if diff < 0 {
                    Some(ReportDirection::Descending)
                } else {
                    Some(ReportDirection::Ascending)
                };
            }
            Some(ref dirr) => match dirr {
                ReportDirection::Descending => {
                    if diff > 0 {
                        return ReportResult::Unsafe;
                    }
                }
                ReportDirection::Ascending => {
                    if diff < 0 {
                        return ReportResult::Unsafe;
                    }
                }
            },
        }
    }
    ReportResult::Safe
}

fn parse_line(line: &str) -> Vec<i32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut output: Vec<i32> = Vec::new();
    for (_, [digits]) in re.captures_iter(line).map(|c| c.extract()) {
        output.push(digits.parse::<i32>().unwrap())
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
        let result = part1(&input);
        assert_eq!(result, 2)
    }

    #[test]
    fn part2_test_input() {
        let input = get_test_input();
        let result = part2(&input);
        assert_eq!(result, 4)
    }
}
