use std::collections::HashMap;

#[derive(Debug)]
struct PuzzleInputError;

fn part1(input: &str) -> Result<u32, PuzzleInputError> {
    let result: Result<Vec<u32>, PuzzleInputError> = input
        .lines()
        .map(|line| {
            // NOTE: As we are summing the results
            //       just return 0 on invalid lines
            if line.is_empty() {
                return Ok(0);
            }

            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            // NOTE: Return an error if no valid digit is in the line
            let first_digit: u32 = *digits.first().ok_or(PuzzleInputError)?;
            let last_digit: u32 = *digits.last().ok_or(PuzzleInputError)?;

            Ok(first_digit * 10 + last_digit)
        })
        .collect();

    match result {
        Ok(v) => Ok(v.into_iter().sum()),
        Err(e) => Err(e),
    }
}

fn part2(input: &str) -> Result<u32, PuzzleInputError> {
    let token_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let result: Result<Vec<u32>, PuzzleInputError> = input
        .lines()
        .map(|line| {
            if line.is_empty() {
                // NOTE: As we are summing the results
                //       just return 0 on invalid lines
                return Ok(0);
            }

            let mut digits: Vec<u32> = Vec::new();
            for index in 0..line.len() {
                for (pattern, digit) in token_map.iter() {
                    if line[index..].starts_with(pattern) {
                        digits.push(*digit);
                        break;
                    }
                }
            }

            // NOTE: Return an error if no valid digit is in the line
            let first_digit: u32 = *digits.first().ok_or(PuzzleInputError)?;
            let last_digit: u32 = *digits.last().ok_or(PuzzleInputError)?;

            Ok(first_digit * 10 + last_digit)
        })
        .collect();

    match result {
        Ok(v) => Ok(v.into_iter().sum()),
        Err(e) => Err(e),
    }
}

fn main() {
    // NOTE: Input file is not part of the repo. Please check out AOC 2023.
    let file: String =
        std::fs::read_to_string("res/day01/input.txt").expect("The given input file should exist.");

    let result1 = part1(&file).expect("Invalid puzzle format error");
    let result2 = part2(&file).expect("Invalid puzzle format error");

    println!("Result Part 1: {}", result1);
    println!("Result Part 2: {}", result2);
}
