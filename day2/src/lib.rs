use std::{io::{BufRead, BufReader}, fs::File};
use thiserror::Error;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct DiceCount{
    pub red: u32,
    pub green: u32,
    pub blue: u32
}

pub struct Game{
    pub id: u32,
    pub records: Vec<DiceCount>
}

#[derive(Error, Debug)]
pub enum ParseGameError{
    #[error("failed to open input file: {inner}")]
    FailedToOpenInputFile{ inner: std::io::Error },
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

pub fn parse_game(line: &str) -> Result<Game, ParseGameError>{
    lazy_static!{
        static ref RE: Regex = Regex::new(r"Game (\d+): (.*)").unwrap();
    }

    let captures = RE.captures(line).unwrap();
    let id = captures[1].parse::<u32>().unwrap();

    let records = captures[2].split("; ").map(parse_roll).collect();
    Ok(Game{ id, records})
}

pub fn parse_games<R: BufRead>(reader: R) -> Result<Vec<Game>, ParseGameError>{
    reader.lines().map(|line| parse_game(&line.map_err(|e| ParseGameError::Any)?)).collect()
}

pub fn has_enough_dice(available_dice: &DiceCount, roll: &DiceCount) -> bool{
    roll.red <= available_dice.red &&
        roll.green <= available_dice.green &&
        roll.blue <= available_dice.blue
}

pub fn read_input() -> Result<Vec<Game>, ParseGameError>{
  File::open("input_day2.txt")
        .map_err(|e| ParseGameError::FailedToOpenInputFile {inner: e})
        .map(|file| BufReader::new(file))
        .and_then(|file| parse_games(file))
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
}