use std::collections::*;
use std::fmt::Display;
use std::io::Read;
use std::ops::{Deref, Range};

use aho_corasick::*;
use clap::Parser;
use nutype::nutype;
use regex::*;

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

const MAX_PROBLEM: u8 = 5;

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
            4 => {
                if !args.subproblem {
                    println!("{}", problem_4a(&input))
                } else {
                    println!("{}", problem_4b(&input))
                }
            }
            5 => {
                if !args.subproblem {
                    println!("{}", problem_5a(&input))
                } else {
                    println!("{}", problem_5b(&input))
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

fn get_winners_4(input: &str) -> HashMap<u32, u32> {
    let mut winners = HashMap::new();
    for line in input.lines() {
        let mut total_wins = 0;
        let splits: Vec<_> = line.split([':', '|']).collect();
        assert_eq!(splits.len(), 3);
        let card_number: u32 = splits[0].split(' ').last().unwrap().parse().unwrap();
        let winning_numbers: HashSet<i32> = splits[1].split(' ').flat_map(|s| s.parse()).collect();
        let numbers_i_have: Vec<i32> = splits[2].split(' ').flat_map(|s| s.parse()).collect();
        for number in numbers_i_have {
            if winning_numbers.contains(&number) {
                total_wins += 1;
            }
        }
        winners.insert(card_number, total_wins);
        //println!("{card_number} has {total_wins} wins");
    }
    winners
}

fn problem_4a(input: &str) -> u64 {
    let mut sum = 0;
    for (_, total) in get_winners_4(input) {
        let points = if total > 0 { 2u64.pow(total - 1) } else { 0u64 };
        //println!("{points} points");
        sum += points;
    }

    sum
}

fn problem_4b(input: &str) -> u64 {
    let cards = get_winners_4(input);
    let mut card_totals: HashMap<u32, u32> = HashMap::with_capacity(cards.len());
    let min = cards.keys().min().unwrap_or(&0).to_owned();
    let max = cards.keys().max().unwrap_or(&0).to_owned();

    for &card in cards.keys() {
        card_totals.insert(card, 1); // Every card starts with one copy
    }

    for card in min..=max {
        let total = cards[&card];
        let copies = card_totals[&card];
        for c in card + 1..=card + total {
            card_totals.entry(c).and_modify(|cp| *cp += copies);
        }
        //println!("Card {} has {} copies", card, card_totals[&card]);
    }

    card_totals.values().sum::<u32>() as u64
}

struct IdMap5<Dest, Src>
where
    Dest: Into<u64> + From<u64> + Deref + Copy,
    Src: Into<u64> + From<u64> + Deref + Copy,
{
    destination: Dest,
    source: Src,
    count: u64,
}
impl<Dest, Src> IdMap5<Dest, Src>
where
    Dest: Into<u64> + From<u64> + Deref + Copy,
    Src: Into<u64> + From<u64> + Deref + Copy,
{
    fn contains(&self, id: &Src) -> bool {
        (self.source.into()..self.source.into() + self.count).contains(&(*id).into())
    }

    fn map(&self, id: Src) -> Option<Dest> {
        if self.contains(&id) {
            Some((id.into() - self.source.into() + self.destination.into()).into())
        } else {
            None
        }
    }

    fn map_chain(maps: impl IntoIterator<Item = Self>, id: u64) -> u64 {
        maps.into_iter()
            .flat_map(|m| m.map(id.into()))
            .next()
            .unwrap_or(id.into())
            .into()
    }

    fn from_iter(iter_: impl IntoIterator<Item = u64>) -> Self {
        let mut iter = iter_.into_iter();
        Self {
            destination: iter.next().unwrap_or(u64::MAX).into(),
            source: iter.next().unwrap_or(u64::MAX).into(),
            count: iter.next().unwrap_or(u64::MAX),
        }
    }
}

trait MappableId5<Dest>
where
    Dest:
        Ord + PartialOrd + Eq + PartialEq + From<u64> + Into<u64> + Copy + Clone + Deref + Display,
    Self:
        Ord + PartialOrd + Eq + PartialEq + From<u64> + Into<u64> + Copy + Clone + Deref + Display,
{
    fn map(self, maps: impl IntoIterator<Item = IdMap5<Self, Dest>>) -> Dest {
        IdMap5::map_chain(maps, self.into()).into()
        //println!("{self} -> {tmp}");
        //tmp
    }
}
#[nutype(derive(
    Debug, Ord, PartialOrd, Eq, PartialEq, From, Into, Copy, Clone, Deref, Display
))]
struct SeedID5(u64);
#[nutype(derive(
    Debug, Ord, PartialOrd, Eq, PartialEq, From, Into, Copy, Clone, Deref, Display
))]
struct SoilID5(u64);
#[nutype(derive(
    Debug, Ord, PartialOrd, Eq, PartialEq, From, Into, Copy, Clone, Deref, Display
))]
struct FertilizerID5(u64);
#[nutype(derive(
    Debug, Ord, PartialOrd, Eq, PartialEq, From, Into, Copy, Clone, Deref, Display
))]
struct WaterID5(u64);
#[nutype(derive(
    Debug, Ord, PartialOrd, Eq, PartialEq, From, Into, Copy, Clone, Deref, Display
))]
struct LightID5(u64);
#[nutype(derive(
    Debug, Ord, PartialOrd, Eq, PartialEq, From, Into, Copy, Clone, Deref, Display
))]
struct TemperatureID5(u64);
#[nutype(derive(
    Debug, Ord, PartialOrd, Eq, PartialEq, From, Into, Copy, Clone, Deref, Display
))]
struct HumidityID5(u64);
#[nutype(derive(
    Debug, Ord, PartialOrd, Eq, PartialEq, From, Into, Copy, Clone, Deref, Display
))]
struct LocationID5(u64);

impl MappableId5<SoilID5> for SeedID5 {}
impl MappableId5<FertilizerID5> for SoilID5 {}
impl MappableId5<WaterID5> for FertilizerID5 {}
impl MappableId5<LightID5> for WaterID5 {}
impl MappableId5<TemperatureID5> for LightID5 {}
impl MappableId5<HumidityID5> for TemperatureID5 {}
impl MappableId5<LocationID5> for HumidityID5 {}

fn make_map_vec<Src, Dest>(str: &str) -> Vec<IdMap5<Src, Dest>>
where
    Src: Ord + PartialOrd + Eq + PartialEq + From<u64> + Into<u64> + Copy + Clone + Deref,
    Dest: Ord + PartialOrd + Eq + PartialEq + From<u64> + Into<u64> + Copy + Clone + Deref,
{
    str.lines()
        .map(|line| line.split(' ').flat_map(|num| num.parse::<u64>()))
        .map(IdMap5::from_iter)
        .collect()
}

fn parse5(input: &str) -> Captures {
    let re = Regex::new(
        r"(?x)
seeds:(?<seeds>(?:\s\d+)+)\n
\n
seed-to-soil\smap:\n
(?<seed_to_soil>(?:\d+\s\d+\s\d+\n)+)
\n
soil-to-fertilizer\smap:\n
(?<soil_to_fertilizer>(?:\d+\s\d+\s\d+\n)+)
\n
fertilizer-to-water\smap:
\n
(?<fertilizer_to_water>(?:\d+\s\d+\s\d+\n)+)
\n
water-to-light\smap:\n
(?<water_to_light>(?:\d+\s\d+\s\d+\n)+)
\n
light-to-temperature\smap:\n
(?<light_to_temperature>(?:\d+\s\d+\s\d+\n)+)
\n
temperature-to-humidity\smap:\n
(?<temperature_to_humidity>(?:\d+\s\d+\s\d+\n)+)
\n
humidity-to-location\smap:\n
(?<humidity_to_location>(?:\d+\s\d+\s\d+\n?)+)",
    )
    .unwrap();

    re.captures(input).unwrap()
}

fn find_min_seed_5(captures: &Captures, seeds: impl IntoIterator<Item = SeedID5>) -> LocationID5 {
    seeds
        .into_iter()
        .map(|x| x.map(make_map_vec::<SeedID5, SoilID5>(&captures["seed_to_soil"])))
        .map(|x| {
            x.map(make_map_vec::<SoilID5, FertilizerID5>(
                &captures["soil_to_fertilizer"],
            ))
        })
        .map(|x| {
            x.map(make_map_vec::<FertilizerID5, WaterID5>(
                &captures["fertilizer_to_water"],
            ))
        })
        .map(|x| {
            x.map(make_map_vec::<WaterID5, LightID5>(
                &captures["water_to_light"],
            ))
        })
        .map(|x| {
            x.map(make_map_vec::<LightID5, TemperatureID5>(
                &captures["light_to_temperature"],
            ))
        })
        .map(|x| {
            x.map(make_map_vec::<TemperatureID5, HumidityID5>(
                &captures["temperature_to_humidity"],
            ))
        })
        .map(|x| {
            x.map(make_map_vec::<HumidityID5, LocationID5>(
                &captures["humidity_to_location"],
            ))
        })
        .min()
        .unwrap()
}

fn problem_5a(input: &str) -> u64 {
    let captures = parse5(input);


    find_min_seed_5(&captures, captures["seeds"]
        .split(' ')
        .flat_map(|num| num.parse::<u64>())
        .map(SeedID5::new)).into()
}
fn problem_5b(input: &str) -> u64 {
    let captures = parse5(input);

    let re = Regex::new(r" (?<seed>\d+) (?<count>\d+)").unwrap();

    find_min_seed_5(
        &captures,
        re.captures_iter(&captures["seeds"])
            .flat_map(|capture| {
                let seed = capture["seed"].parse::<u64>().unwrap();
                let count = capture["count"].parse::<u64>().unwrap();
                seed..seed + count
            })
            .map(SeedID5::new),
    )
    .into()
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
    #[test]
    fn test_problem_4a() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = problem_4a(input);
        assert_eq!(result, 13)
    }
    #[test]
    fn test_problem_4b() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = problem_4b(input);
        assert_eq!(result, 30)
    }

    #[test]
    fn test_problem_5a() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = problem_5a(input);
        assert_eq!(result, 35)
    }
    #[test]
    fn test_problem_5b() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = problem_5b(input);
        assert_eq!(result, 46)
    }
}
