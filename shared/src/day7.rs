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

fn card_order(card: Card) -> u8{
    use Card::*;
    match card{
        Two => 0,
        Three => 1,
        Four  => 2,
        Five => 3,
        Six => 4,
        Seven => 5,
        Eight => 6,
        Nine => 7,
        T => 8,
        J => 9,
        Q => 10,
        K => 11,
        A => 12,
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

fn classify_hand(hand: &Hand) -> HandClassification{
    // Count unique cards in the hand
    let mut counts = StackVec::<[(Card, u8); 5]>::new();
    for card in hand.0{
        if let Some(counter) = counts.iter_mut().find(|counter| counter.0 == card){
            counter.1 = counter.1 + 1;
        }
        else{
            counts.push((card, 1));
        }
    }

    // Sort the unique cards by count
    counts.sort_by(|a, b| b.1.cmp(&a.1));

    if counts.len() == 1{
        return HandClassification::FiveOfAKind;
    }

    if counts.len() == 2 && counts[0].1 == 4{
        return HandClassification::FourOfAKind;
    }

    if counts.len() == 2 && counts[0].1 == 3{
        return HandClassification::FullHouse;
    }

    if counts.len() == 3 && counts[0].1 == 3{
        return HandClassification::ThreeOfAKind;
    }

    if counts.len() == 3 && counts[0].1 == 2{
        return HandClassification::TwoPair;
    }

    if counts.len() == 4 && counts[0].1 == 2{
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

fn bid_compare_score(a: &Bid, b: &Bid) -> std::cmp::Ordering{
    let classification_a = classify_hand(&a.hand);
    let classification_b = classify_hand(&b.hand);
    let classification_order = classification_a.cmp(&classification_b);
    if classification_order != Ordering::Equal{
        return classification_order;
    }

    a.hand.0.map(card_order).cmp(&b.hand.0.map(card_order))
}

fn order_bids_by_rank(bids: &mut Vec<Bid>){
    bids.sort_by(|a, b| bid_compare_score(a, b))
}

fn calculate_total_winnings<R: std::io::BufRead>(input: R) -> Option<usize>{
    let mut bids = parse_bids(input)?;
    order_bids_by_rank(&mut bids);
    Some(bids.iter().enumerate().map(|(rank, bid)| bid.bid * (rank + 1)).sum())
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

        assert_eq!(classify_hand(&hand_five_of_a_kind), HandClassification::FiveOfAKind);
        assert_eq!(classify_hand(&hand_four_of_a_kind), HandClassification::FourOfAKind);
        assert_eq!(classify_hand(&hand_full_house), HandClassification::FullHouse);
        assert_eq!(classify_hand(&hand_three_of_a_kind), HandClassification::ThreeOfAKind);
        assert_eq!(classify_hand(&hand_two_pair), HandClassification::TwoPair);
        assert_eq!(classify_hand(&hand_one_pair), HandClassification::OnePair);
        assert_eq!(classify_hand(&hand_high_card), HandClassification::HighCard);
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
        order_bids_by_rank(&mut bids);
        assert_eq!(bids[0].hand, Hand::try_from_str("32T3K").unwrap());
        assert_eq!(bids[1].hand, Hand::try_from_str("KTJJT").unwrap());
        assert_eq!(bids[2].hand, Hand::try_from_str("KK677").unwrap());
        assert_eq!(bids[3].hand, Hand::try_from_str("T55J5").unwrap());
        assert_eq!(bids[4].hand, Hand::try_from_str("QQQJA").unwrap());
    }
}