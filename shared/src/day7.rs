use std::{path::Display, fmt::Write, cmp::Ordering};

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

fn card_order(card: Card, use_jokers: bool) -> u8{
    use Card::*;

    match card{
        J if use_jokers => 1,
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

fn classify_hand(hand: &Hand, use_jokers: bool) -> HandClassification{
    // Count unique cards in the hand
    let mut joker_count = 0;
    let mut counts = StackVec::<[(Card, u8); 5]>::new();
    for card in hand.0{
        if card == Card::J && use_jokers{
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

fn bid_compare_score(a: &Bid, b: &Bid, use_jokers: bool) -> std::cmp::Ordering{
    let classification_a = classify_hand(&a.hand, use_jokers);
    let classification_b = classify_hand(&b.hand, use_jokers);
    let classification_order = classification_a.cmp(&classification_b);
    if classification_order != Ordering::Equal{
        return classification_order;
    }


    a.hand.0.map(|card| card_order(card, use_jokers)).cmp(&b.hand.0.map(|card| card_order(card, use_jokers)))
}

fn order_bids_by_rank(bids: &mut Vec<Bid>, use_jokers: bool){
    bids.sort_by(|a, b| bid_compare_score(a, b, use_jokers))
}

fn calculate_total_winnings<R: std::io::BufRead>(input: R, use_jokers: bool) -> Option<usize>{
    let mut bids = parse_bids(input)?;
    order_bids_by_rank(&mut bids, use_jokers);
    Some(bids.iter().enumerate().map(|(rank, bid)| bid.bid * (rank + 1)).sum())
}

#[aoc_2023_markup::aoc_task(2023, 7, 1)]
fn part1<R: std::io::BufRead>(input: R) -> Option<usize>{
    calculate_total_winnings(input, false)
}

#[aoc_2023_markup::aoc_task(2023, 7, 2)]
fn part2<R: std::io::BufRead>(input: R) -> Option<usize>{
    calculate_total_winnings(input, true)
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

        assert_eq!(classify_hand(&hand_five_of_a_kind, false), HandClassification::FiveOfAKind);
        assert_eq!(classify_hand(&hand_four_of_a_kind, false), HandClassification::FourOfAKind);
        assert_eq!(classify_hand(&hand_full_house, false), HandClassification::FullHouse);
        assert_eq!(classify_hand(&hand_three_of_a_kind, false), HandClassification::ThreeOfAKind);
        assert_eq!(classify_hand(&hand_two_pair, false), HandClassification::TwoPair);
        assert_eq!(classify_hand(&hand_one_pair, false), HandClassification::OnePair);
        assert_eq!(classify_hand(&hand_high_card, false), HandClassification::HighCard);
    }

    #[test]
    fn test_classify_hand_jokers(){
        assert_eq!(classify_hand(&Hand::try_from_str("T55J5").unwrap(), true), HandClassification::FourOfAKind);
        assert_eq!(classify_hand(&Hand::try_from_str("KTJJT").unwrap(), true), HandClassification::FourOfAKind);
        assert_eq!(classify_hand(&Hand::try_from_str("QQQJA").unwrap(), true), HandClassification::FourOfAKind);
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
    fn test_rank_hands(){
        let mut bids = parse_bids(BIDS).unwrap();
        order_bids_by_rank(&mut bids, false);
        assert_eq!(bids[0].hand, Hand::try_from_str("32T3K").unwrap());
        assert_eq!(bids[1].hand, Hand::try_from_str("KTJJT").unwrap());
        assert_eq!(bids[2].hand, Hand::try_from_str("KK677").unwrap());
        assert_eq!(bids[3].hand, Hand::try_from_str("T55J5").unwrap());
        assert_eq!(bids[4].hand, Hand::try_from_str("QQQJA").unwrap());
    }

    #[test]
    fn test_rank_hands_with_jokers(){
        let mut bids = parse_bids(BIDS).unwrap();
        order_bids_by_rank(&mut bids, true);
        assert_eq!(bids[0].hand, Hand::try_from_str("32T3K").unwrap());
        assert_eq!(bids[1].hand, Hand::try_from_str("KK677").unwrap());
        assert_eq!(bids[2].hand, Hand::try_from_str("T55J5").unwrap());
        assert_eq!(bids[3].hand, Hand::try_from_str("QQQJA").unwrap());
        assert_eq!(bids[4].hand, Hand::try_from_str("KTJJT").unwrap());
    }

    #[test]
    fn test_winnings(){
        let winnings = calculate_total_winnings(BIDS, false).unwrap();
        assert_eq!(winnings, 6440);
    }

    #[test]
    fn test_winnings_with_jokers(){
        let winnings = calculate_total_winnings(BIDS, true).unwrap();
        assert_eq!(winnings, 5905);
    }
}