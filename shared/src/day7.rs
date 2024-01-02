use std::{path::Display, fmt::Write, cmp::Ordering};

use itertools::Itertools;
use stackvector::StackVec;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandClassification{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
enum Card{
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A
}

fn card_order<const USE_JOKERS: bool>(card: Card,) -> u8{
    use Card::*;

    match card{
        J if USE_JOKERS => 1,
        Two => 2,
        Three => 3,
        Four  => 4,
        Five => 5,
        Six => 6,
        Seven => 7,
        Eight => 8,
        Nine => 9,
        T => 10,
        J => 11,
        Q => 12,
        K => 13,
        A => 14,
    }
}

fn parse_card(char: char) -> Option<Card>{
    use Card::*;
    match char{
        '2' => Some(Two),
        '3' => Some(Three),
        '4' => Some(Four),
        '5' => Some(Five),
        '6' => Some(Six),
        '7' => Some(Seven),
        '8' => Some(Eight),
        '9' => Some(Nine),
        'T' => Some(T),
        'J' => Some(J),
        'Q' => Some(Q),
        'K' => Some(K),
        'A' => Some(A),
        _ => None
    }
}

#[derive(PartialEq, Eq)]
struct Hand([Card; 5]);

impl Hand{
    fn try_from_str(str: &str) -> Option<Hand>{
        let cards = str.chars().map(|c| parse_card(c)).collect::<Option<Vec<Card>>>()?;
        Some(Hand(cards.try_into().ok()?))
    }
}

impl std::fmt::Debug for Hand{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.0{
            let char = match card{
                Card::Two => '2',
                Card::Three => '3',
                Card::Four => '4',
                Card::Five => '5',
                Card::Six => '6',
                Card::Seven => '7',
                Card::Eight => '8',
                Card::Nine => '9',
                Card::T => 'T',
                Card::J => 'J',
                Card::Q => 'Q',
                Card::K => 'K',
                Card::A => 'A',
            };
            f.write_char(char)?;
        }

        Ok(())
    }
}

fn classify_hand<const USE_JOKERS: bool>(hand: &Hand) -> HandClassification{
    // Count unique cards in the hand
    let mut joker_count = 0;
    let mut counts = StackVec::<[(Card, u8); 5]>::new();
    for card in hand.0{
        if card == Card::J && USE_JOKERS{
            joker_count = joker_count + 1;
        } else if let Some(counter) = counts.iter_mut().find(|counter| counter.0 == card){
            counter.1 = counter.1 + 1;
        } else{
            counts.push((card, 1));
        }
    }


    // Sort the unique cards by count
    counts.sort_by(|a, b| b.1.cmp(&a.1));

    let highest_count = counts.get(0).map(|pair| pair.1).unwrap_or(0);
    let second_count = counts.get(1).map(|pair| pair.1).unwrap_or(0);

    if joker_count + highest_count == 5{
        return HandClassification::FiveOfAKind;
    }

    if joker_count + highest_count == 4{
        return HandClassification::FourOfAKind;
    }

    if joker_count + highest_count == 3{
        let jokers_left = joker_count - (3 - highest_count);
        if jokers_left + second_count == 2{
            return HandClassification::FullHouse;
        }
        else{
            return HandClassification::ThreeOfAKind;
        }
    }

    // Note: No joker checks for 2 pair, joker would always be consumed by the first pair to make a three of a kind
    if highest_count == 2 && second_count == 2{
        return HandClassification::TwoPair;
    }

    if joker_count + highest_count == 2{
        return HandClassification::OnePair;
    }

    HandClassification::HighCard
}

#[derive(Debug, Eq, PartialEq)]
struct Bid{
    hand: Hand,
    bid: usize,
}

fn parse_bid(line: &str) -> Option<Bid>{
    let mut parts = line.split_ascii_whitespace(); 
    let hand = Hand::try_from_str(parts.next()?)?;
    let bid = parts.next()?.parse::<usize>().ok()?;
    Some(Bid{
        hand, bid
    })
}

fn parse_bids<R: std::io::BufRead>(input: R) -> Option<Vec<Bid>>{
    input.lines().map(|line| line.ok().and_then(|line| parse_bid(&line))).collect()
}

fn bid_compare_score<const USE_JOKERS: bool>(a: &(&Bid, HandClassification), b: &(&Bid, HandClassification)) -> std::cmp::Ordering{
    let classification_order = a.1.cmp(&b.1);
    if classification_order != Ordering::Equal{
        return classification_order;
    }
    a.0.hand.0.map(|card| card_order::<USE_JOKERS>(card)).cmp(&b.0.hand.0.map(|card| card_order::<USE_JOKERS>(card)))
}

fn calculate_total_winnings<R: std::io::BufRead, const USE_JOKERS: bool>(input: R) -> Option<usize>{
    let bids = parse_bids(input)?;

    let classified_bids : Vec<(&Bid, HandClassification)>= bids.iter()
        .map(|bid| (bid, classify_hand::<USE_JOKERS>(&bid.hand)))
        .sorted_by(bid_compare_score::<USE_JOKERS>)
        .collect();
    
    Some(classified_bids.iter().enumerate().map(|(rank, bid)| bid.0.bid * (rank + 1)).sum())
}

#[aoc_2023_markup::aoc_task(2023, 7, 1)]
fn part1<R: std::io::BufRead>(input: R) -> Option<usize>{
    calculate_total_winnings::<_, false>(input)
}

#[aoc_2023_markup::aoc_task(2023, 7, 2)]
fn part2<R: std::io::BufRead>(input: R) -> Option<usize>{
    calculate_total_winnings::<_,true>(input)
}

#[cfg(test)]
mod tests{
    use indoc::indoc;
    use super::*;


    #[test]
    fn test_hand_from_str(){
        use Card::*;
        let hand = Hand::try_from_str("AAAAA").unwrap();
        assert_eq!(hand, Hand([A, A, A, A, A]));
        let hand = Hand::try_from_str("AA8AA").unwrap();
        assert_eq!(hand, Hand([A, A, Eight, A, A]));
        let hand = Hand::try_from_str("23332").unwrap();
        assert_eq!(hand, Hand([Two, Three, Three, Three, Two]));
        let hand = Hand::try_from_str("TTT98").unwrap();
        assert_eq!(hand, Hand([T, T, T, Nine, Eight]));
    }
    #[test]
    fn test_classify_hand(){
        let hand_five_of_a_kind : Hand = Hand::try_from_str("AAAAA").unwrap();
        let hand_four_of_a_kind : Hand= Hand::try_from_str("AA8AA").unwrap();
        let hand_full_house : Hand = Hand::try_from_str("23332").unwrap();
        let hand_three_of_a_kind : Hand = Hand::try_from_str("TTT98").unwrap();
        let hand_two_pair : Hand = Hand::try_from_str("23432").unwrap();
        let hand_one_pair : Hand = Hand::try_from_str("A23A4").unwrap();
        let hand_high_card : Hand = Hand::try_from_str("23456").unwrap();

        assert_eq!(classify_hand::<false>(&hand_five_of_a_kind), HandClassification::FiveOfAKind);
        assert_eq!(classify_hand::<false>(&hand_four_of_a_kind), HandClassification::FourOfAKind);
        assert_eq!(classify_hand::<false>(&hand_full_house), HandClassification::FullHouse);
        assert_eq!(classify_hand::<false>(&hand_three_of_a_kind), HandClassification::ThreeOfAKind);
        assert_eq!(classify_hand::<false>(&hand_two_pair), HandClassification::TwoPair);
        assert_eq!(classify_hand::<false>(&hand_one_pair), HandClassification::OnePair);
        assert_eq!(classify_hand::<false>(&hand_high_card), HandClassification::HighCard);
    }

    #[test]
    fn test_classify_hand_jokers(){
        assert_eq!(classify_hand::<true>(&Hand::try_from_str("T55J5").unwrap()), HandClassification::FourOfAKind);
        assert_eq!(classify_hand::<true>(&Hand::try_from_str("KTJJT").unwrap()), HandClassification::FourOfAKind);
        assert_eq!(classify_hand::<true>(&Hand::try_from_str("QQQJA").unwrap()), HandClassification::FourOfAKind);
    }

    const BIDS : &[u8] = indoc!{"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "}.as_bytes();

    #[test]
    fn test_parse_bids(){
        let bids = parse_bids(BIDS).unwrap();
        assert_eq!(bids, [
            Bid { hand: Hand::try_from_str("32T3K").unwrap(), bid: 765 },
            Bid { hand: Hand::try_from_str("T55J5").unwrap(), bid: 684},
            Bid { hand: Hand::try_from_str("KK677").unwrap(), bid: 28},
            Bid { hand: Hand::try_from_str("KTJJT").unwrap(), bid: 220 },
            Bid { hand: Hand::try_from_str("QQQJA").unwrap(), bid: 483 }, 
        ]);
    }

    #[test]
    fn test_winnings(){
        let winnings = calculate_total_winnings::<_, false>(BIDS).unwrap();
        assert_eq!(winnings, 6440);
    }

    #[test]
    fn test_winnings_with_jokers(){
        let winnings = calculate_total_winnings::<_, true>(BIDS).unwrap();
        assert_eq!(winnings, 5905);
    }
}