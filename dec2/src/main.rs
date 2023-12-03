use pom::parser::*;
use util::{self, parser::*};

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Draw {
    draw: Vec<(u32, Color)>,
}

impl Draw {
    fn total_rgb(&self) -> (u32, u32, u32) {
        self.draw.iter().fold(
            (0_u32, 0_u32, 0_u32),
            |(r, g, b), (number, color)| match color {
                Color::Red => (r + number, g, b),
                Color::Green => (r, g + number, b),
                Color::Blue => (r, g, b + number),
            },
        )
    }

    fn invalid(&self) -> bool {
        let colors = self.total_rgb();
        colors.0 > 12 || colors.1 > 13 || colors.2 > 14
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn has_invalid(&self) -> bool {
        self.draws.iter().any(|draw| draw.invalid())
    }

    fn max_rgb(&self) -> (u32, u32, u32) {
        self.draws
            .iter()
            .map(|draw| draw.total_rgb())
            .fold((0, 0, 0), |(r1, g1, b1), (r2, g2, b2)| {
                (r2.max(r1), g2.max(g1), b2.max(b1))
            })
    }

    fn power(&self) -> u32 {
        let (r, g, b) = self.max_rgb();
        r * g * b
    }
}

fn color<'a>() -> Parser<'a, u8, Color> {
    seq(b"red").map(|_| Color::Red)
        | seq(b"green").map(|_| Color::Green)
        | seq(b"blue").map(|_| Color::Blue)
}

fn draw<'a>() -> Parser<'a, u8, Draw> {
    list(
        posint() - space().opt() + color() - space().opt(),
        sym(b',') + space().opt(),
    )
    .map(|draw| Draw { draw })
}

fn game<'a>() -> Parser<'a, u8, Game> {
    (seq(b"Game") * space() * posint() - sym(b':') - space() + list(draw(), sym(b';') + space()))
        .map(|(id, draws)| Game { id, draws })
}

fn solution_a(input: &[String]) -> u32 {
    let parser = game();
    input
        .iter()
        .map(|input| parser.parse(input.as_bytes()).unwrap())
        .filter(|game| !game.has_invalid())
        .map(|game| game.id)
        .sum()
}

#[test]
fn test_solution_a() {
    let input = util::raw_to_strings(
        r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    );
    assert_eq!(solution_a(&input), 8)
}

fn solution_b(input: &[String]) -> u32 {
    let parser = game();
    input
        .iter()
        .map(|line| parser.parse(line.as_bytes()).unwrap().power())
        .sum()
}

#[test]
fn test_solution_b() {
    let input = util::raw_to_strings(
        r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    );
    assert_eq!(solution_b(&input), 2286)
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
