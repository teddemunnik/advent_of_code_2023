use std::{fs::File, io::BufReader};
use lazy_static::lazy_static;
use regex::Regex;


#[derive(Debug, PartialEq, Eq)]
pub struct Card{
    pub id: u32,
    pub winning: Vec<u8>,
    pub have: Vec<u8>
}

fn parse_number_list(list: &str) -> Vec<u8>{
    list.split_ascii_whitespace().map(|number| number.parse::<u8>().unwrap()).collect()
}

pub fn parse_card(line: &str) -> Card{
    lazy_static!{
        static ref RE: Regex = Regex::new(r"Card +(\d+): (.*) \| (.*)").unwrap();
    }

    let captures = RE.captures(line).unwrap();
    let id = captures[1].parse::<u32>().unwrap();
    let winning = parse_number_list(&captures[2]);
    let have = parse_number_list(&captures[3]);
    Card{
        id,
        winning,
        have
    }
}

pub fn card_score(card: &Card) -> u32{
    let count = card.have.iter().filter(|value| card.winning.contains(&value)).count();
    if count > 0{
        1 << (count as u32 - 1)
    }
    else{
        0
    }
}

pub fn read_input() -> Result<impl std::io::BufRead, std::io::Error>{
    File::open("input_day4.txt")
        .map(|file| BufReader::new(file))
}