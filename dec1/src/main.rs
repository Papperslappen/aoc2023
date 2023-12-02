use pom::parser::*;
use util::{self};

fn solution_a(input: &[String]) -> String {
    let solution = input
        .iter()
        .map(|s| {
            let numbers = s.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
            let n = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
            n.parse::<i32>().unwrap()
        })
        .sum::<i32>();
    solution.to_string()
}

#[test]
fn test_solution_a() {
    let input = util::raw_to_strings(
        r"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet",
    );
    assert_eq!(solution_a(&input), "142".to_string())
}

fn number_parser<'a>() -> Parser<'a, u8, i32> {
    let zero = ((-seq(b"zero")) | (-sym(b'0'))).map(|_| 0);
    let one = ((-seq(b"one")) | (-sym(b'1'))).map(|_| 1);
    let two = ((-seq(b"two")) | (-sym(b'2'))).map(|_| 2);
    let three = ((-seq(b"three")) | (-sym(b'3'))).map(|_| 3);
    let four = ((-seq(b"four")) | (-sym(b'4'))).map(|_| 4);
    let five = ((-seq(b"five")) | (-sym(b'5'))).map(|_| 5);
    let six = ((-seq(b"six")) | (-sym(b'6'))).map(|_| 6);
    let seven = ((-seq(b"seven")) | (-sym(b'7'))).map(|_| 7);
    let eight = ((-seq(b"eight")) | (-sym(b'8'))).map(|_| 8);
    let nine = ((-seq(b"nine")) | (-sym(b'9'))).map(|_| 9);
    zero | one | two | three | four | five | six | seven | eight | nine
}

fn number_and_skip_one<'a>() -> Parser<'a, u8, Option<i32>> {
    number_parser().opt() - skip(1)
}

fn solution_b(input: &[String]) -> i32 {
    let parser = number_and_skip_one().repeat(1..) - any().repeat(0..);
    let nums = input
        .iter()
        .map(|s| {
            parser
                .parse(s.as_bytes())
                .unwrap()
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
        })
        .map(|r| r.first().unwrap() * 10 + r.last().unwrap())
        .sum();
    nums
}

#[test]
fn test_solution_b() {
    let input = util::raw_to_strings(
        r"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen",
    );
    println!("{:?}", solution_b(&input));
    assert_eq!(solution_b(&input), 281)
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
