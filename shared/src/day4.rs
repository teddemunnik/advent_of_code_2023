use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Card{
    id: u32,
    winning: Vec<u8>,
    have: Vec<u8>
}

#[derive(Debug)]
struct ParseCardsError{
    line: usize,
}

impl std::fmt::Display for ParseCardsError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "failed to parse card on line {}", self.line)
    }
} 

fn parse_number_list(list: &str) -> Option<Vec<u8>>{
    list.split_ascii_whitespace().map(|number| number.parse::<u8>().ok()).collect()
}

fn parse_card(line: &str) -> Option<Card>{
    lazy_static!{
        static ref RE: Regex = Regex::new(r"Card +(\d+): (.*) \| (.*)").unwrap();
    }

    let captures = RE.captures(line)?;
    let id = captures[1].parse::<u32>().ok()?;
    let winning = parse_number_list(&captures[2])?;
    let have = parse_number_list(&captures[3])?;
    Some(Card{
        id,
        winning,
        have
    })
}

fn parse_cards<R: std::io::BufRead>(input: R) -> Result<Vec<Card>, ParseCardsError>{
   input 
        .lines()
        .enumerate()
        .map(|(line_index, line)| line.ok().and_then(|line| parse_card(&line)).ok_or(ParseCardsError{ line: line_index }))
        .collect()
}

fn card_matches(card: &Card) -> usize{
    card.have.iter().filter(|value| card.winning.contains(&value)).count()
}

fn card_score(card: &Card) -> u32{
    let count = card_matches(card);
    if count > 0{
        1 << (count as u32 - 1)
    }
    else{
        0
    }
}

#[aoc_2023_markup::aoc_task(2023, 4, 1)]
fn calculate_total_score(input: &mut dyn BufRead) -> Result<u32, ParseCardsError>{
    parse_cards(input)
        .map(|cards| cards.iter().map(|card| card_score(&card)).sum())
}

#[aoc_2023_markup::aoc_task(2023, 4, 2)]
fn count_scratchcards(input: &mut dyn BufRead) -> Result<usize, ParseCardsError>{
    let cards = parse_cards(input)?;
    let mut card_counts : Vec<usize> = vec![1; cards.len()];

    for card_index in 0..cards.len(){
        let match_count = card_matches(&cards[card_index]);

        let begin_prize = card_index + 1;
        let end_prize = (begin_prize + match_count).min(cards.len());

        for i in begin_prize..end_prize{
            card_counts[i] += card_counts[card_index];
        }
    }

    Ok(card_counts.iter().copied().sum::<usize>())
}


#[cfg(test)]
mod tests{
    use std::io::Cursor;

    use indoc::indoc;
    use super::*;

    #[test]
    fn test_parse_card(){
        const INPUT : &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(INPUT).unwrap();
        assert_eq!(card, Card{
            id: 1,
            winning: vec![41, 48, 83, 86, 17],
            have: vec![83, 86, 6, 31, 17, 9, 48, 53]
        });
    }

    #[test]
    fn test_total_score(){
        const INPUT : &[u8] = indoc!{"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "}.as_bytes();

        let score = calculate_total_score(&mut Cursor::new(INPUT)).unwrap();
        assert_eq!(score, 13);
    }

    #[test]
    fn test_count_cards(){
        const INPUT : &[u8] = indoc!{"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "}.as_bytes();

        let count = count_scratchcards(&mut Cursor::new(INPUT)).unwrap();
        assert_eq!(count, 30);

    }

}