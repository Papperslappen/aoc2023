use std::collections::HashSet;

use pom::parser::{seq, sym};
use util::parser::{posint, space};

#[derive(Debug)]
struct Card {
    ticket: Vec<u32>,
    winning: HashSet<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let parser = (seq(b"Card")*space() * posint() - sym(b':') - space()
            + (posint() - space()).repeat(1..)
            - sym(b'|')
            - space()
            + (posint() - space()).repeat(1..))
        .map(|((_n, v1), v2)| Card {
            ticket: v2,
            winning: HashSet::from_iter(v1),
        });
        parser.parse(value.as_bytes()).unwrap()
    }
}

impl Card {
    fn matches(&self)->u32 {
        self
            .ticket
            .iter()
            .filter(|n| self.winning.contains(n))
            .collect::<Vec<_>>()
            .len() as u32
    }
    fn score(&self) -> u32 {
        match self.matches()
        {
            0 => 0,
            n => 2_u32.pow(n - 1),
        }
    }
}

fn solution_a(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| {
            Card::from(s.as_str()).score()
        })
        .sum()
}

#[test]
fn test_solution_a() {
    let input = util::raw_to_strings(
        r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    );
    assert_eq!(solution_a(&input), 13)
}

#[allow(clippy::needless_range_loop)]
fn solution_b(input: &[String]) -> u32 {
    let cards_numbers = std::iter::repeat(1_u32).take(input.len()).collect::<Vec<_>>();
    let matches = input.iter().map(|s| Card::from(s.as_str()).matches()).collect::<Vec<_>>();
    let cards = matches.iter().enumerate().fold(cards_numbers,|mut card_numbers, (index,number)| {
        //println!("index: {}",index);
        let ncards = card_numbers[index];
        //println!("Card {}: {} copies with {} wins", index, ncards, number);
        for j in (index+1)..=index+*number as usize {
            card_numbers[j]+=ncards;
        }
        card_numbers
    });

    cards.iter().sum()
}

#[test]
fn test_solution_b() {
    let input = util::raw_to_strings(
        r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    );
    assert_eq!(solution_b(&input), 30)
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
