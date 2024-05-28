use std::io::Read;

use aho_corasick::*;
use clap::Parser;

/// Program that solves Advent of Code 2023 problems
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Problem to solve
    #[arg(short, long)]
    problem: u8,

    // Is this the second part?
    #[arg(short, long, action)]
    subproblem: bool,

    /// Filename of problem input
    #[arg(short, long)]
    file: clio::Input,
}

const MAX_PROBLEM: u8 = 1;

fn main() {
    let mut args = Args::parse();
    if args.problem > MAX_PROBLEM {
        println!("Problem given is greater than maximum allowed ({MAX_PROBLEM})");
        return;
    }

    if let Some(file) = args.file.get_file() {
        let mut input = String::new();
        if let Err(err) = file.read_to_string(&mut input) {
            println!("Error reading file {err}")
        }
        match args.problem {
            1 => {
                if !args.subproblem {
                    println!("{}", problem_1a(&input))
                } else {
                    println!("{}", problem_1b(&input))
                }
            }
            _ => {}
        }
    } else {
        let path = args.file.path();
        println!("File does not exist ({path})!");
    }
}

fn problem_1a(input: &str) -> u64 {
    let mut numbers = Vec::<u64>::new();
    for line in input.lines() {
        let mut num_str = String::new();
        for char in line.chars() {
            if char.is_ascii_digit() {
                num_str.push(char);
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_ascii_digit() {
                num_str.push(char);
                break;
            }
        }
        numbers.push(num_str.parse().unwrap());
    }
    numbers.iter().sum()
}

fn parse_digit(digit: &str) -> core::result::Result<u8, <u8 as std::str::FromStr>::Err> {
    match digit {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        _ => digit.parse::<u8>(),
    }
}

fn problem_1b(input: &str) -> u64 {
    let mut total = 0u64;
    let ac = AhoCorasick::new([
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ])
    .unwrap();

    for line in input.lines() {
        let mut num = 0u64;
        let vec: Vec<_> = ac.find_overlapping_iter(line).collect();

        if !vec.is_empty() {
            num += 10 * parse_digit(&line[vec.first().unwrap().span()]).unwrap_or(0) as u64;
            num += parse_digit(&line[vec.last().unwrap().span()]).unwrap_or(0) as u64;
        }
        total += num;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_1a_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = problem_1a(input);
        assert_eq!(result, 142u64)
    }
    #[test]
    fn test_problem_1b_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = problem_1b(input);
        assert_eq!(result, 281u64)
    }
    #[test]
    fn test_problem_1b_one_digit() {
        let input = "5";
        let result = problem_1b(input);
        assert_eq!(result, 55)
    }
    #[test]
    fn test_problem_1b_oneight() {
        let input = "oneight";
        let result = problem_1b(input);
        assert_eq!(result, 18)
    }
}
