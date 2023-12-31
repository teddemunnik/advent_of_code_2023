use std::{io::{BufRead, BufReader}, fs::File};
use thiserror::Error;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct DiceCount{
    red: u32,
    green: u32,
    blue: u32
}

struct Game{
    id: u32,
    records: Vec<DiceCount>
}

#[derive(Error, Debug)]
enum ParseGameError{
    #[error("any")]
    Any
}

fn parse_roll(roll: &str) -> DiceCount{
    let mut record = DiceCount{
        red: 0,
        green: 0,
        blue: 0
    };

    let entries = roll.split(", ");
    for entry in entries{
        let mut segments = entry.split(' ');
        let test = segments.next().unwrap();
        let number = test.parse::<u32>().unwrap();
        let ty = segments.next().unwrap();

        if ty == "red"{
            record.red += number;
        }
        else if ty == "green"{
            record.green += number;
        }
        else if ty == "blue"{
            record.blue += number;
        }
        else{
            // error
        }
    }

    record
}

fn parse_game(line: &str) -> Result<Game, ParseGameError>{
    lazy_static!{
        static ref RE: Regex = Regex::new(r"Game (\d+): (.*)").unwrap();
    }

    let captures = RE.captures(line).unwrap();
    let id = captures[1].parse::<u32>().unwrap();

    let records = captures[2].split("; ").map(parse_roll).collect();
    Ok(Game{ id, records})
}

fn parse_games<R: BufRead>(reader: R) -> Result<Vec<Game>, ParseGameError>{
    reader.lines().map(|line| parse_game(&line.map_err(|e| ParseGameError::Any)?)).collect()
}

fn has_enough_dice(available_dice: &DiceCount, roll: &DiceCount) -> bool{
    roll.red <= available_dice.red &&
        roll.green <= available_dice.green &&
        roll.blue <= available_dice.blue
}

fn add_possible_games(games: &[Game], available_dice: &DiceCount) -> u32{
    let possible_games = games.iter().filter(|game| game.records.iter().all(|roll| has_enough_dice(available_dice, roll)));
    possible_games.map(|game| game.id).reduce(|a, b| a + b).unwrap_or(0)
}

#[aoc_2023_markup::aoc_task(2023, 2, 1)]
fn part1(input: &mut dyn BufRead){
    let input = parse_games(input);

    let available_dice = DiceCount{
        red: 12,
        green: 13,
        blue: 14
    };

    crate::run(input.map(|games| add_possible_games(&games, &available_dice)));
}

fn calculate_power(game: &Game) -> u32{
    game.records
        .iter()
        .copied()
        .reduce(|a, b|{
            DiceCount{
                red: a.red.max(b.red),
                green: a.green.max(b.green),
                blue: a.blue.max(b.blue),
            }
        })
        .map_or(0, |counts| counts.red * counts.green * counts.blue)
}

#[aoc_2023_markup::aoc_task(2023, 2, 2)]
fn part2(input: &mut dyn BufRead){
    let input = parse_games(input);
    crate::run(input.map(|games| games.iter().map(calculate_power).sum::<u32>()));
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_parse_game(){
        let game = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.records, [
            DiceCount{ blue: 3, red: 4, green: 0 },
            DiceCount{ red: 1, green: 2, blue: 6 },
            DiceCount{green: 2, red: 0, blue: 0 },
        ]);
    }

    #[test]
    fn add_possible_games(){
        const INPUT_GAMES : &[u8] =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
             Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
             Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
             Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
             Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".as_bytes();

        const INPUT_DICE : DiceCount = DiceCount{
            red: 12,
            green: 13,
            blue: 14
        };

        let games = parse_games(INPUT_GAMES).unwrap();
        let count = super::add_possible_games(&games, &INPUT_DICE);
        assert_eq!(count, 8);
    }

    #[test]
    fn test_power(){
        const INPUT : &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(INPUT).unwrap();
        let power = calculate_power(&game);
        assert_eq!(power, 48);
    }

}