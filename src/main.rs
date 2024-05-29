use std::io::Read;

use aho_corasick::*;
use clap::Parser;
use regex::*;
use std::collections::*;
use std::ops::Range;

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

const MAX_PROBLEM: u8 = 3;

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
            2 => {
                if !args.subproblem {
                    println!("{}", problem_2a(&input))
                } else {
                    println!("{}", problem_2b(&input))
                }
            }
            3 => {
                if !args.subproblem {
                    println!("{}", problem_3a(&input))
                } else {
                    println!("{}", problem_3b(&input))
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

fn parse_digit(digit: &str) -> Result<u8, <u8 as std::str::FromStr>::Err> {
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

struct Rgb2 {
    red: i32,
    green: i32,
    blue: i32,
}
fn parse_rounds_2(input: &str) -> Rgb2 {
    let re = Regex::new(r"(?<count>\d+) (?<color>red|green|blue)").unwrap();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for round in input.split(';') {
        for (_, [count_str, color_str]) in re.captures_iter(round).map(|c| c.extract()) {
            let count: i32 = count_str.parse().unwrap();
            match color_str {
                "red" => red = red.max(count),
                "green" => green = green.max(count),
                "blue" => blue = blue.max(count),
                _ => panic!(),
            }
        }
    }
    Rgb2 { red, green, blue }
}

fn problem_2a(input: &str) -> u64 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum = 0u64;

    for line in input.lines() {
        let game: u64 = line
            .split(':') // Get "Game NN" and rest
            .next()
            .unwrap()
            .split(' ') // Get "Game" and "NN"
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let Rgb2 { red, green, blue } = parse_rounds_2(line);

        if red <= max_red && green <= max_green && blue <= max_blue {
            sum += game;
        } else {
            //println!("Rejecting  r: {red} g: {green} b: {blue} :'{line}'")
        }
    }

    sum
}
fn problem_2b(input: &str) -> u64 {
    let mut sum = 0u64;

    for line in input.lines() {
        let Rgb2 { red, green, blue } = parse_rounds_2(line);
        sum += (red * green * blue) as u64;
    }

    sum
}

fn catalog_numbers_3(input: &str) -> Vec<Vec<Range<usize>>> {
    let re = Regex::new(r"\d+").unwrap();

    input
        .lines()
        .map(|row| re.find_iter(row).map(|m| m.range()).collect())
        .collect()
}

fn find_adjacant_numbers_3(
    row_number: usize,
    column_number: usize,
    number_catalog: &[Vec<Range<usize>>],
) -> Vec<(usize, (usize, usize))> {
    let mut vec = Vec::new();
    for (number_row, numbers) in number_catalog.iter().enumerate() {
        if (row_number - 1..=row_number + 1).contains(&number_row) {
            for number_range in numbers {
                let expanded_range = (1.max(number_range.start) - 1)..(number_range.end + 1);
                if expanded_range.contains(&column_number) {
                    vec.push((number_row, (number_range.start, number_range.end)));
                }
            }
        }
    }
    vec
}

fn problem_3a(input: &str) -> u64 {
    let symbols = ['%', '+', '=', '$', '@', '#', '/', '*', '&', '-'];

    let mut sum = 0;
    let number_catalog = catalog_numbers_3(input);
    let mut to_process: HashSet<(usize, (usize, usize))> = HashSet::new();

    let rows: Vec<_> = input.lines().collect();

    for (row_number, &line) in rows.iter().enumerate() {
        for (column_number, _) in line.match_indices(symbols) {
            to_process.extend(find_adjacant_numbers_3(
                row_number,
                column_number,
                &number_catalog,
            ));
        }
    }

    for (row, range) in to_process {
        let tmp = &rows[row][range.0..range.1];
        // println!("{}", tmp);
        sum += tmp.parse::<u64>().unwrap();
    }
    sum
}
fn problem_3b(input: &str) -> u64 {
    let mut sum = 0;
    let number_catalog = catalog_numbers_3(input);
    let rows: Vec<_> = input.lines().collect();

    for (row_number, &line) in rows.iter().enumerate() {
        for (column_number, _) in line.match_indices('*') {
            let to_process: HashSet<(usize, (usize, usize))> = HashSet::from_iter(
                find_adjacant_numbers_3(row_number, column_number, &number_catalog).to_owned(),
            );

            if to_process.len() == 2 {
                let mut product = 1;
                for (row, range) in to_process {
                    product *= &rows[row][range.0..range.1].parse::<u64>().unwrap();
                }
                sum += product;
            }
        }
    }
    sum
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
    #[test]
    fn test_problem_2a() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = problem_2a(input);
        assert_eq!(result, 8)
    }
    #[test]
    fn test_problem_2b() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = problem_2b(input);
        assert_eq!(result, 2286)
    }
    #[test]
    fn test_problem_3a() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = problem_3a(input);
        assert_eq!(result, 4361)
    }
    #[test]
    fn test_problem_3b() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = problem_3b(input);
        assert_eq!(result, 467835)
    }
}
