use pom::parser::*;
use util::parser::{int, space};

fn sequence_extrapolation(sequence: Vec<i64>) -> (i64, i64) {
    let first = *sequence.first().unwrap();
    let last = *sequence.last().unwrap();
    let iter1 = sequence.iter();
    let differences = iter1
        .clone()
        .zip(iter1.clone().skip(1))
        .map(|(x, y)| y - x)
        .collect::<Vec<_>>();
    if differences.iter().any(|x| *x != 0) {
        let (f, l) = sequence_extrapolation(differences);
        (first - f, last + l)
    } else {
        (first, last)
    }
}

fn solution_a(input: &[String]) -> i64 {
    let parser = space() * (int().name("int") - space()).repeat(1..) - end();
    input
        .iter()
        .map(|s| sequence_extrapolation(parser.parse(s.as_bytes()).unwrap()).1)
        .sum()
}

fn solution_b(input: &[String]) -> i64 {
    let parser = space() * (int().name("int") - space()).repeat(1..) - end();
    input
        .iter()
        .map(|s| sequence_extrapolation(parser.parse(s.as_bytes()).unwrap()).0)
        .sum()
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input = raw_to_strings(
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    );
    assert_eq!(solution_a(&input), 114);
    assert_eq!(solution_b(&input), 2);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
