use pom::parser::*;
use util::parser::{posint, space};

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        let time = self.time as f64;
        let distance = self.distance as f64;

        let lower = time / 2.0 - ((time / 2.0) * (time / 2.0) - distance).sqrt();
        let upper = time / 2.0 + ((time / 2.0) * (time / 2.0) - distance).sqrt();
        (largest_int_less_than(upper) - smallest_int_bigger_than(lower)) as u64 + 1
    }
}

fn largest_int_less_than(x: f64) -> i32 {
    let y = x.floor() as i32;

    if (y as f64 - x).abs() < f64::EPSILON {
        y - 1
    } else {
        y
    }
}

fn smallest_int_bigger_than(x: f64) -> i32 {
    let y = x.ceil() as i32;
    if (y as f64 - x).abs() < f64::EPSILON {
        y + 1
    } else {
        y
    }
}

fn solution_a(input: &[String]) -> u64 {
    let time_parser = seq(b"Time:") * space() * list(posint(), space());
    let distance_parser = seq(b"Distance:") * space() * list(posint(), space());
    let time = time_parser.parse(input[0].as_bytes()).unwrap();
    let distance = distance_parser.parse(input[1].as_bytes()).unwrap();
    let races = time
        .into_iter()
        .zip(distance)
        .map(|(time, distance)| Race {
            time: time as u64,
            distance: distance as u64,
        })
        .collect::<Vec<_>>();
    races
        .iter()
        .map(|race| race.ways_to_win())
        .reduce(|a, b| a * b)
        .unwrap()
}

fn solution_b(input: &[String]) -> u64 {
    let time = input[0]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = input[1]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let race = Race { time, distance };
    race.ways_to_win()
}

#[test]
fn test_solutions() {
    let input = util::raw_to_strings(
        "Time:      7  15   30
Distance:  9  40  200",
    );
    assert_eq!(solution_a(&input), 288);
    assert_eq!(solution_b(&input), 71503);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
