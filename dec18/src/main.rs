use itertools::Itertools;
use pom::utf8::*;
use std::collections::HashSet;
use util::parser::utf8::{posint, space};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        use Direction::*;
        match value {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!("Invalid char"),
        }
    }
}

impl Direction {
    fn step(&self, steps: i64) -> (i64, i64) {
        match self {
            Direction::Up => (0, -steps),
            Direction::Right => (steps, 0),
            Direction::Down => (0, steps),
            Direction::Left => (-steps, 0),
        }
    }
}

#[allow(unused)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(unused)]
struct ContourCommand {
    direction: Direction,
    length: i64,
    color: Color,
}

fn byte<'a>() -> Parser<'a, u8> {
    one_of("01234567889ABCDEFabcdef")
        .repeat(2)
        .collect()
        .map(|s| u8::from_str_radix(s, 16).unwrap())
}

fn color<'a>() -> Parser<'a, Color> {
    (sym('#') * byte() + byte() + byte()).map(|((r, g), b)| Color { r, g, b })
}

fn parser<'a>() -> Parser<'a, ContourCommand> {
    (one_of("UDLR").map(Direction::from) - space() + posint() - space() - sym('(') + color()
        - sym(')'))
    .map(|((direction, length), color)| ContourCommand {
        direction,
        length: length as i64,
        color,
    })
}

fn parser_b<'a>() -> Parser<'a, ContourCommand> {
    ((one_of("UDLR").map(Direction::from) - space() + posint() - space() - sym('('))
        * (sym('#')
            + one_of("01234567889ABCDEFabcdef")
                .repeat(5)
                .collect()
                .map(|s| i64::from_str_radix(s, 16).unwrap())
            + one_of("0123").map(|c| {
                use Direction::*;
                match c {
                    '3' => Up,
                    '1' => Down,
                    '2' => Left,
                    '0' => Right,
                    _ => panic!("Invalid char"),
                }
            }))
        - sym(')'))
    .map(|((_, length), direction)| ContourCommand {
        length,
        direction,
        color: Color { r: 0, g: 0, b: 0 },
    })
}

fn flood_fill(
    start: (i64, i64),
    edge: impl Fn((i64, i64)) -> bool,
    in_bounds: impl Fn((i64, i64)) -> bool,
) -> HashSet<(i64, i64)> {
    let mut stack = vec![start];
    let mut result = HashSet::new();
    while let Some((x, y)) = stack.pop() {
        if !edge((x, y)) && in_bounds((x, y)) && result.insert((x, y)) {
            stack.push((x + 1, y));
            stack.push((x - 1, y));
            stack.push((x, y + 1));
            stack.push((x, y - 1));
        } else if !in_bounds((x, y)) {
            return HashSet::new();
        }
    }
    result
}

fn area(commands: Vec<ContourCommand>) -> i64 {
    let corners = commands
        .iter()
        .scan((0, 0), |coordinate, command| {
            let dir = command.direction.step(command.length);
            coordinate.0 += dir.0;
            coordinate.1 += dir.1;
            Some(*coordinate)
        })
        .collect::<Vec<_>>();

    commands
        .iter()
        .zip(corners)
        .map(|(command, (_endx, endy))| match command.direction {
            Direction::Up => command.length,
            Direction::Right => -2 * command.length * endy + command.length,
            Direction::Down => command.length,
            Direction::Left => 2 * (command.length) * (endy) + command.length,
        })
        .sum::<i64>()
        .abs()
        / 2
        + 1
}

fn solution_a(input: &[String]) -> u64 {
    let parser = parser();
    let contour = input
        .iter()
        .map(|s| parser.parse(s.as_bytes()).unwrap())
        .scan((0, 0), |coordinate, command| {
            let old_c = *coordinate;
            let dir = command.direction.step(command.length);
            coordinate.0 += dir.0;
            coordinate.1 += dir.1;
            Some((1..=command.length).map(move |s| {
                let dir = command.direction.step(s);
                (old_c.0 + dir.0, old_c.1 + dir.1)
            }))
        })
        .flatten()
        .collect::<HashSet<_>>();

    let (minx, maxx, miny, maxy) =
        contour
            .iter()
            .fold((i64::MAX, i64::MIN, i64::MAX, i64::MIN), |acc, c| {
                (
                    acc.0.min(c.0),
                    acc.1.max(c.0),
                    acc.2.min(c.1),
                    acc.3.max(c.1),
                )
            });
    println!("minx: {minx}, maxx: {maxx} miny: {miny} maxy: {maxy}");

    let fill = (minx..maxx)
        .cartesian_product(miny..maxy)
        .map(|coordinate| {
            flood_fill(
                coordinate,
                |c| contour.contains(&c),
                |(x, y)| minx <= x && x <= maxx && miny <= y && y <= maxy,
            )
        })
        .find(|v| !v.is_empty())
        .unwrap();

    (contour.len() + fill.len()) as u64
}

fn solution_b(input: &[String]) -> i64 {
    let parser = parser_b();
    let commands = input
        .iter()
        .map(|s| parser.parse(s.as_bytes()).unwrap())
        .collect::<Vec<_>>();
    area(commands)
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input1 = raw_to_strings(
        r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
    );
    assert_eq!(solution_a(&input1), 62);
    assert_eq!(solution_b(&input1), 952408144115);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
