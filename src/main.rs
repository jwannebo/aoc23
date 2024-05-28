use std::io::Read;

use clap::Parser;
use clio::*;

/// Program that solves Advent of Code 2023 problems
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Problem to solve
    #[arg(short, long)]
    problem: u8,

    // Is this the subproblem?
    #[arg(short, long, action)]
    subproblem: bool,

    /// Filename of problem input
    #[arg(short, long)]
    file: Input,
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
        match file.read_to_string(&mut input) {
            Err(err) => println!("Error reading file {err}"),
            _ => {}
        }
        match args.problem {
            1 => {
                if !args.subproblem {
                    println!("{}", problem_1a(&input))
                } else {
                    problem_1b(&input)
                }
            }
            _ => {}
        }
    } else {
        let path = args.file.path();
        println!("File does not exist ({path})!");
        return;
    }
}

fn problem_1a(input: &str) -> u64 {
    let mut numbers = Vec::<u64>::new();
    for line in input.lines() {
        let mut num_str = String::new();
        for char in line.chars() {
            if char.is_digit(10) {
                num_str.push(char);
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_digit(10) {
                num_str.push(char);
                break;
            }
        }
        numbers.push(num_str.parse().unwrap());
    }
    numbers.iter().sum()
}

fn problem_1b(_input: &String) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_1a() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = problem_1a(input);
        assert_eq!(result, 142u64)
    }
}
