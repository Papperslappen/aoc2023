use std::collections::{HashMap, HashSet};

use util::Direction;

#[derive(PartialEq)]
enum Feature {
    MirrorLeft,
    MirrorRight,
    VerticalSplit,
    HorisontalSplit,
    Empty,
}

impl From<char> for Feature {
    fn from(value: char) -> Self {
        match value {
            '\\' => Self::MirrorLeft,
            '/' => Self::MirrorRight,
            '-' => Self::HorisontalSplit,
            '|' => Self::VerticalSplit,
            '.' => Self::Empty,
            _ => panic!("Invalid character"),
        }
    }
}

struct Floor {
    width: usize,
    height: usize,
    features: HashMap<(usize, usize), Feature>,
}

impl From<&[String]> for Floor {
    fn from(value: &[String]) -> Self {
        let height = value.len();
        let width = value[0].len();
        let features = value
            .iter()
            .enumerate()
            .flat_map(|(row, s)| {
                s.chars()
                    .enumerate()
                    .map(move |(col, c)| ((col, row), c.into()))
            })
            .collect::<HashMap<_, _>>();
        Floor {
            width,
            height,
            features,
        }
    }
}

impl Floor {
    fn firing_my_lazor(&self, laser: ((usize, usize), Direction)) -> u64 {
        let mut lasers = vec![laser];
        let mut laser_visits = HashSet::<((usize, usize), Direction)>::new();

        while let Some((coordinate, direction)) = lasers.pop() {
            if laser_visits.insert((coordinate, direction)) {
                match self.features.get(&coordinate).unwrap() {
                    Feature::MirrorLeft => {
                        let new_direction = match direction {
                            Direction::E => Direction::S,
                            Direction::N => Direction::W,
                            Direction::W => Direction::N,
                            Direction::S => Direction::E,
                        };
                        if let Some(l) =
                            new_direction.move_in(coordinate, (self.width, self.height))
                        {
                            lasers.push((l, new_direction))
                        }
                    }
                    Feature::MirrorRight => {
                        let new_direction = match direction {
                            Direction::E => Direction::N,
                            Direction::N => Direction::E,
                            Direction::W => Direction::S,
                            Direction::S => Direction::W,
                        };
                        if let Some(l) =
                            new_direction.move_in(coordinate, (self.width, self.height))
                        {
                            lasers.push((l, new_direction))
                        }
                    }
                    Feature::VerticalSplit => match direction {
                        Direction::E | Direction::W => {
                            lasers.push((coordinate, Direction::N));
                            lasers.push((coordinate, Direction::S));
                        }
                        _ => {
                            if let Some(laser) = direction
                                .move_in(coordinate, (self.width, self.height))
                                .map(|c| (c, direction))
                            {
                                lasers.push(laser)
                            }
                        }
                    },
                    Feature::HorisontalSplit => match direction {
                        Direction::N | Direction::S => {
                            lasers.push((coordinate, Direction::E));
                            lasers.push((coordinate, Direction::W));
                        }
                        _ => {
                            if let Some(laser) = direction
                                .move_in(coordinate, (self.width, self.height))
                                .map(|c| (c, direction))
                            {
                                lasers.push(laser)
                            }
                        }
                    },
                    Feature::Empty => {
                        if let Some(laser) = direction
                            .move_in(coordinate, (self.width, self.height))
                            .map(|c| (c, direction))
                        {
                            lasers.push(laser)
                        }
                    }
                }
            }
        }

        laser_visits
            .iter()
            .map(|(c, _)| c)
            .collect::<HashSet<_>>()
            .len() as u64
    }
}

fn solution_a(input: &[String]) -> u64 {
    let floor: Floor = input.into();

    floor.firing_my_lazor(((0_usize, 0_usize), Direction::E))
}

fn solution_b(input: &[String]) -> u64 {
    let floor: Floor = input.into();
    (1..floor.width)
        .map(|col| ((col, 0), Direction::S))
        .chain((1..floor.width).map(|col| ((col, floor.height - 1), Direction::N)))
        .chain((1..floor.height).map(|row| ((0, row), Direction::E)))
        .chain((1..floor.height).map(|row| ((floor.width - 1, row), Direction::W)))
        .map(|laser| floor.firing_my_lazor(laser))
        .max()
        .unwrap()
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input1 = raw_to_strings(
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
    );
    assert_eq!(solution_a(&input1), 46);
    assert_eq!(solution_b(&input1), 51);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
