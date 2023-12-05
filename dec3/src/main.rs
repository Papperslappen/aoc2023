use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn solution_a(input: &[String]) -> u32 {
    let mut numbers: Vec<(i32, i32, u32)> = vec![];
    let mut symbols: HashMap<(i32, i32), char> = HashMap::new();
    let mut numberbuffer: Vec<char> = vec![];
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                numberbuffer.push(c);
            } else {
                if !numberbuffer.is_empty() {
                    let number = numberbuffer
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    numbers.push((row as i32, col as i32, number));
                    numberbuffer.clear();
                }
                if c == '.' {
                } else {
                    symbols.insert((row as i32, col as i32), c);
                }
            }
        }
        if !numberbuffer.is_empty() {
            let number = numberbuffer
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            numbers.push((row as i32, line.len() as i32, number));
            numberbuffer.clear();
        }
    }

    numbers
        .iter()
        .filter(|(row, col, number)| {
            let length = number.checked_ilog10().unwrap_or(0) + 1;
            let rows = row - 1..=row + 1;
            let cols = (*col - length as i32 - 1)..=*col;
            rows.cartesian_product(cols)
                .any(|(row, col)| symbols.contains_key(&(row, col)))
        })
        .map(|(_, _, number)| number)
        .sum()
}

#[test]
fn test_solution_a() {
    let input = util::raw_to_strings(
        r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    );
    assert_eq!(solution_a(&input), 4361)
}

fn solution_b(input: &[String]) -> u32 {
    let mut stars = Vec::<(i32, i32)>::new();
    let mut serial_number = 0;
    let mut numbers = HashMap::<(i32, i32), (u32, i32)>::new();
    let mut numberbuffer: Vec<char> = vec![];
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_numeric() {
                numberbuffer.push(c);
            } else {
                if !numberbuffer.is_empty() {
                    let number = numberbuffer
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    (1..=numberbuffer.len()).for_each(|i| {
                        numbers.insert((row as i32, (col - i) as i32), (number, serial_number));
                    });
                    serial_number += 1;
                    numberbuffer.clear();
                }
                if c == '*' {
                    stars.push((row as i32, col as i32));
                }
            }
        }
        if !numberbuffer.is_empty() {
            let number = numberbuffer
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            (1..=numberbuffer.len()).for_each(|i| {
                numbers.insert(
                    (row as i32, (line.len() - i) as i32),
                    (number, serial_number),
                );
            });
            serial_number += 1;
            numberbuffer.clear();
        }
    }

    stars
        .iter()
        .map(|(row, col)| {
            let adjecent_numbers: HashSet<(u32, i32)> = ((row - 1)..=(row + 1))
                .cartesian_product((col - 1)..=(col + 1))
                .filter_map(|(row, col)| numbers.get(&(row, col)))
                .cloned()
                .collect();
            adjecent_numbers
        })
        .filter(|set| set.len() == 2)
        .map(|set| {
            set.iter()
                .fold(1_u32, |product, (number, _)| product * number)
        })
        .sum()
}

#[test]
fn test_solution_b() {
    let input = util::raw_to_strings(
        r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    );
    assert_eq!(solution_b(&input), 467835)
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
