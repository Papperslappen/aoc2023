use pom::utf8::*;
use util::parser::space;

mod solution_a {
    use util::count_unique;

    #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
    pub struct Card(u32);

    #[derive(Debug)]
    pub struct CardError(String);

    impl TryFrom<char> for Card {
        type Error = CardError;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '2' => Ok(Card(2)),
                '3' => Ok(Card(3)),
                '4' => Ok(Card(4)),
                '5' => Ok(Card(5)),
                '6' => Ok(Card(6)),
                '7' => Ok(Card(7)),
                '8' => Ok(Card(8)),
                '9' => Ok(Card(9)),
                'T' => Ok(Card(10)),
                'J' => Ok(Card(11)),
                'Q' => Ok(Card(12)),
                'K' => Ok(Card(13)),
                'A' => Ok(Card(14)),
                c => Err(CardError(format!("invalid character: {}", c))),
            }
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub enum Hand {
        HighCard(Vec<Card>),
        Pair(Vec<Card>),
        TwoPair(Vec<Card>),
        Three(Vec<Card>),
        House(Vec<Card>),
        FourOfAKind(Vec<Card>),
        FiveOfAKind(Vec<Card>),
    }

    impl Hand {
        fn valid(&self) -> bool {
            match self {
                Hand::HighCard(_) => true,
                Hand::Pair(cards) => count_unique(cards).iter().any(|(_, v)| *v == 2),
                Hand::TwoPair(cards) => {
                    count_unique(cards).iter().fold(
                        0,
                        |number, (_, v)| {
                            if *v == 2 {
                                number + 1
                            } else {
                                number
                            }
                        },
                    ) == 2
                }
                Hand::Three(cards) => count_unique(cards).iter().any(|(_, v)| *v == 3),
                Hand::House(cards) => {
                    count_unique(cards).iter().any(|(_, v)| *v == 3)
                        && count_unique(cards).iter().any(|(_, v)| *v == 2)
                }
                Hand::FourOfAKind(cards) => count_unique(cards).iter().any(|(_, v)| *v == 4),
                Hand::FiveOfAKind(cards) => count_unique(cards).iter().any(|(_, v)| *v == 5),
            }
        }
        pub fn from_cards(cards: Vec<Card>) -> Hand {
            [
                Hand::FiveOfAKind(cards.clone()),
                Hand::FourOfAKind(cards.clone()),
                Hand::House(cards.clone()),
                Hand::Three(cards.clone()),
                Hand::TwoPair(cards.clone()),
                Hand::Pair(cards.clone()),
                Hand::HighCard(cards.clone()),
            ]
            .into_iter()
            .find(|hand| hand.valid())
            .unwrap()
        }
    }
}

mod solution_b {
    use std::collections::HashMap;

    #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
    pub struct Card(u32);

    #[derive(Debug)]
    pub struct CardError(String);

    impl TryFrom<char> for Card {
        type Error = CardError;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'J' => Ok(Card(0)),
                '2' => Ok(Card(2)),
                '3' => Ok(Card(3)),
                '4' => Ok(Card(4)),
                '5' => Ok(Card(5)),
                '6' => Ok(Card(6)),
                '7' => Ok(Card(7)),
                '8' => Ok(Card(8)),
                '9' => Ok(Card(9)),
                'T' => Ok(Card(10)),
                'Q' => Ok(Card(12)),
                'K' => Ok(Card(13)),
                'A' => Ok(Card(14)),
                c => Err(CardError(format!("invalid character: {}", c))),
            }
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub enum Hand {
        HighCard(Vec<Card>),
        Pair(Vec<Card>),
        TwoPair(Vec<Card>),
        Three(Vec<Card>),
        House(Vec<Card>),
        FourOfAKind(Vec<Card>),
        FiveOfAKind(Vec<Card>),
    }

    fn count_unique_and_sub_jokers(cards: &[Card])->Vec<(Card,usize)> {
        let mut map = cards.iter()
        .fold(HashMap::<Card, usize>::new(), |mut m, x| {
            *m.entry(*x).or_default() += 1;
            m
        });
        if let Some(njokers) = map.remove(&Card(0)) {
            //AAAAaaaaah
            if !map.is_empty(){
                *map.iter_mut().max_by(|(_,v1),(_,v2)| v1.cmp(v2)).unwrap().1 += njokers;
            }else{
                map.insert(Card(0),njokers);
            }
        }
        map.into_iter().collect()
    }

    impl Hand {
        fn valid(&self) -> bool {
            match self {
                Hand::HighCard(_) => true,
                Hand::Pair(cards) => count_unique_and_sub_jokers(cards).iter().any(|(_, v)| *v == 2),
                Hand::TwoPair(cards) => {
                    count_unique_and_sub_jokers(cards).iter().fold(
                        0,
                        |number, (_, v)| {
                            if *v == 2 {
                                number + 1
                            } else {
                                number
                            }
                        },
                    ) == 2
                }
                Hand::Three(cards) => count_unique_and_sub_jokers(cards).iter().any(|(_, v)| *v == 3),
                Hand::House(cards) => {
                    count_unique_and_sub_jokers(cards).iter().any(|(_, v)| *v == 3)
                        && count_unique_and_sub_jokers(cards).iter().any(|(_, v)| *v == 2)
                }
                Hand::FourOfAKind(cards) => count_unique_and_sub_jokers(cards).iter().any(|(_, v)| *v == 4),
                Hand::FiveOfAKind(cards) => count_unique_and_sub_jokers(cards).iter().any(|(_, v)| *v == 5),
            }
        }

        pub fn from_cards(cards: Vec<Card>) -> Hand {
            [
                Hand::FiveOfAKind(cards.clone()),
                Hand::FourOfAKind(cards.clone()),
                Hand::House(cards.clone()),
                Hand::Three(cards.clone()),
                Hand::TwoPair(cards.clone()),
                Hand::Pair(cards.clone()),
                Hand::HighCard(cards.clone()),
            ]
            .into_iter()
            .find(|hand| hand.valid())
            .unwrap()
        }
    }
}

fn solution_a(input: &[String]) -> u64 {
    let digit = one_of("0123456789");
    let integer = digit.discard().repeat(1..);
    let parser = space()
        * one_of("23456789TJQKA")
            .convert(solution_a::Card::try_from)
            .repeat(5)
        + space() * integer.collect().convert(|x| x.parse::<u64>());
    let mut hands = input
        .iter()
        .map(|s| parser.parse(s.as_bytes()).unwrap())
        .map(|(cards, bet)| (solution_a::Hand::from_cards(cards), bet))
        .collect::<Vec<_>>();
    hands.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));
    hands
        .iter()
        .enumerate()
        .map(|(n, (_, bet))| (n as u64 + 1) * bet)
        .sum()
}

fn solution_b(input: &[String]) -> u64 {
    let digit = one_of("0123456789");
    let integer = digit.discard().repeat(1..);
    let parser = space()
        * one_of("23456789TJQKA")
            .convert(solution_b::Card::try_from)
            .repeat(5)
        + space() * integer.collect().convert(|x| x.parse::<u64>());
    let mut hands = input
        .iter()
        .map(|s| parser.parse(s.as_bytes()).unwrap())
        .map(|(cards, bet)| (solution_b::Hand::from_cards(cards), bet))
        .collect::<Vec<_>>();
    hands.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));
    hands
        .iter()
        .enumerate()
        .map(|(n, (_, bet))| (n as u64 + 1) * bet)
        .sum()
}

#[test]
fn test_solutions() {
    let input = util::raw_to_strings(
        "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483",
    );
    assert_eq!(solution_a(&input), 6440);
    assert_eq!(solution_b(&input), 5905);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
