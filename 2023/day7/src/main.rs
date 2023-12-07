use core::panic;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    writeln!(io::stdout(), "{}", part1(&input)?)?;
    writeln!(io::stdout(), "{}", part2(&input)?)?;
    Ok(())
}

fn part1(input: &str) -> io::Result<u32> {
    let mut hands: Vec<Hand> = input.lines().map(Hand::from).collect();
    hands.sort();
    let sum = hands
        .into_iter()
        .enumerate()
        .map(|(i, card)| card.bid * (i as u32 + 1))
        .sum();

    Ok(sum)
}

fn part2(input: &str) -> io::Result<u32> {
    let mut hands: Vec<Hand> = input.lines().map(Hand::from).collect();
    hands.sort();
    let sum = hands
        .into_iter()
        .enumerate()
        .map(|(i, card)| card.bid * (i as u32 + 1))
        .sum();

    Ok(sum)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    typ: HandType,
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    #[cfg(test)]
    fn new(cards: Vec<char>, bid: u32, typ: HandType) -> Self {
        let cards = cards.iter().map(Card::from).collect();
        Self { cards, bid, typ }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Jack,
    Qeen,
    King,
    Ace,
}

impl From<&char> for Card {
    fn from(value: &char) -> Self {
        match value {
            'A' => Card::Ace,
            'Q' => Card::Qeen,
            'K' => Card::King,
            'J' => Card::Jack,
            'T' => Card::T,
            '9' => Card::N9,
            '8' => Card::N8,
            '7' => Card::N7,
            '6' => Card::N6,
            '5' => Card::N5,
            '4' => Card::N4,
            '3' => Card::N3,
            '2' => Card::N2,
            _ => panic!("invalid card"),
        }
    }
}

impl From<&Vec<char>> for HandType {
    fn from(value: &Vec<char>) -> Self {
        let mut occurrencies: HashMap<char, u32> = HashMap::new();
        for card in value {
            match occurrencies.get_mut(&card) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    occurrencies.insert(*card, 1);
                }
            }
        }

        if let Some(js) = occurrencies.remove(&'J') {
            if js == 5 {
                occurrencies.insert('A', 5);
            } else {
                let (max_key, max_value) =
                    occurrencies.iter().max_by_key(|&(_, value)| value).unwrap();
                occurrencies.insert(*max_key, max_value + js);
            }
        }

        let values: Vec<u32> = occurrencies.into_values().collect();
        let r#type = match values.len() {
            1 => HandType::FiveOfAKind,
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            3 => match values.contains(&3) {
                true => HandType::ThreeOfAKind,
                false => HandType::TwoPair,
            },
            2 => match values.contains(&4) {
                true => HandType::FourOfAKind,
                false => HandType::FullHouse,
            },
            _ => panic!("hand with too many cards"),
        };

        r#type
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards, bid) = value.split_once(" ").unwrap();

        let cards = cards.chars().collect();
        let typ = HandType::from(&cards);
        let cards = cards.iter().map(Card::from).collect();

        Self {
            cards,
            bid: bid.parse().unwrap(),
            typ,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handtype_from() {
        assert_eq!(
            HandType::FiveOfAKind,
            HandType::from(&vec!['A', 'A', 'A', 'A', 'A'])
        );
        assert_eq!(
            HandType::FourOfAKind,
            HandType::from(&vec!['A', 'Q', 'Q', 'Q', 'Q'])
        );
        assert_eq!(
            HandType::FullHouse,
            HandType::from(&vec!['A', 'A', 'J', 'J', 'J'])
        );
        assert_eq!(
            HandType::TwoPair,
            HandType::from(&vec!['A', 'A', 'K', 'K', 'Q'])
        );
        assert_eq!(
            HandType::OnePair,
            HandType::from(&vec!['A', 'T', 'A', 'K', 'Q'])
        );
        assert_eq!(
            HandType::HighCard,
            HandType::from(&vec!['A', 'T', 'Q', 'J', 'K'])
        );
    }

    #[test]
    fn test_hand_from() {
        assert_eq!(
            Hand::new(vec!['A', 'Q', 'J', 'K', 'T'], 10, HandType::HighCard),
            Hand::from("AQJKT 10")
        );
        assert_eq!(
            Hand::new(vec!['A', 'A', 'K', 'K', 'T'], 35, HandType::TwoPair),
            Hand::from("AAKKT 35")
        );
    }

    #[test]
    fn test_card_ord() {
        let mut rnd = vec![
            Card::Ace,
            Card::Qeen,
            Card::King,
            Card::Jack,
            Card::T,
            Card::N6,
            Card::N9,
        ];

        rnd.sort();

        assert_eq!(
            vec![
                Card::N6,
                Card::N9,
                Card::T,
                Card::Jack,
                Card::Qeen,
                Card::King,
                Card::Ace,
            ],
            rnd
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            6440,
            part1("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483").unwrap()
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            5905,
            part1("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483").unwrap()
        )
    }
}
