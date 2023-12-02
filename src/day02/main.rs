use std::collections::HashMap;

#[derive(Debug)]
struct PuzzleInputError;

fn part1(input: &str) -> Result<usize, PuzzleInputError> {
    let max_amounts_per_color = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let result: Result<Vec<usize>, PuzzleInputError> = input
        .lines()
        .map(|line| {
            if line.is_empty() {
                return Ok(0);
            }

            let (game_prefix, game_record) = line.split_once(':').ok_or(PuzzleInputError)?;

            // NOTE: Input should look like this "Game {Number}" and we assume a correct puzzle input
            // format, so we can just skip the first 5 chars
            let index: usize = game_prefix[5..].parse().map_err(|_| PuzzleInputError)?;

            for reveal in game_record.split(';') {
                for num_of_color in reveal.split(',') {
                    let mut tokens = num_of_color.split_whitespace();

                    let amount: usize = tokens
                        .next()
                        .ok_or(PuzzleInputError)?
                        .parse()
                        .map_err(|_| PuzzleInputError)?;

                    let color = tokens.next().ok_or(PuzzleInputError)?;

                    let max_amount = *max_amounts_per_color.get(color).ok_or(PuzzleInputError)?;

                    // NOTE: It is okay to just return 0 here, as the result is summed and this line is
                    // therefore ignored
                    if amount > max_amount {
                        return Ok(0);
                    }
                }
            }

            Ok(index)
        })
        .collect();

    match result {
        Ok(v) => Ok(v.into_iter().sum()),
        Err(e) => Err(e),
    }
}

fn part2(input: &str) -> Result<usize, PuzzleInputError> {
    let result: Result<Vec<usize>, PuzzleInputError> = input
        .lines()
        .map(|line| {
            if line.is_empty() {
                return Ok(0);
            }

            let mut amounts_per_color = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            // NOTE: The game index is not used in part 2
            let (_, game_record) = line.split_once(':').ok_or(PuzzleInputError)?;

            for reveal in game_record.split(';') {
                for num_of_color in reveal.split(',') {
                    let mut tokens = num_of_color.split_whitespace();

                    let amount: usize = tokens
                        .next()
                        .ok_or(PuzzleInputError)?
                        .parse()
                        .map_err(|_| PuzzleInputError)?;

                    let color = tokens.next().ok_or(PuzzleInputError)?;

                    let max_amount = amounts_per_color.get_mut(color).ok_or(PuzzleInputError)?;

                    // NOTE: Collect the max amount of each color into the map
                    if amount > *max_amount {
                        *max_amount = amount;
                    }
                }
            }

            // NOTE: Multiply the max amounts for each color for each line
            Ok(amounts_per_color.values().product())
        })
        .collect();

    match result {
        Ok(v) => Ok(v.into_iter().sum()),
        Err(e) => Err(e),
    }
}

fn main() {
    // NOTE: Input file is not part of the repo. Please check out AOC 2023.
    let f: String =
        std::fs::read_to_string("res/day02/input.txt").expect("The given input file should exist.");

    let result1: usize = part1(&f).expect("Invalid puzzle format error");
    let result2: usize = part2(&f).expect("Invalid puzzle format error");

    println!("Result Part 1: {}", result1);
    println!("Result Part 2: {}", result2);
}
