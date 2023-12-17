use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{Debug, Display},
    hash::Hash,
    iter::{once, repeat},
};

use iter_tools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Boulder,
    Block,
    Empty,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            'O' => Space::Boulder,
            '#' => Space::Block,
            '.' => Space::Empty,
            _ => panic!(),
        }
    }
}

impl From<Space> for char {
    fn from(value: Space) -> Self {
        match value {
            Space::Boulder => 'O',
            Space::Block => '#',
            Space::Empty => '.',
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn move_one(&self, (col, row): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::North => row.checked_sub(1).map(|row| (col, row)),
            Direction::South => Some((col, row + 1)),
            Direction::East => Some((col + 1, row)),
            Direction::West => col.checked_sub(1).map(|col| (col, row)),
        }
    }
}

#[derive(PartialEq, Clone, Hash, Eq)]
struct Platform {
    spaces: Vec<Space>,
    width: usize,
    height: usize,
}

impl From<&[String]> for Platform {
    fn from(value: &[String]) -> Self {
        let height = value.len();
        let width = value[0].len();
        let spaces = value
            .iter()
            .flat_map(|s| s.chars().map(|c| c.into()))
            .collect();
        Platform {
            spaces,
            width,
            height,
        }
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}\n", self)
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self
            .spaces
            .chunks(self.width)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|c| std::convert::Into::<char>::into(*c))
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        writeln!(f, "{}", b)
    }
}

impl Platform {
    fn at(&self, (col, row): (usize, usize)) -> Option<&Space> {
        self.spaces.get(col + self.width * row)
    }
    fn at_mut(&mut self, (col, row): (usize, usize)) -> Option<&mut Space> {
        self.spaces.get_mut(col + self.width * row)
    }

    fn move_one(&mut self, at: (usize, usize), direction: Direction) {
        let space = self.at_mut(at).unwrap();
        if *space == Space::Boulder {
            *space = Space::Empty;
            let coordinates = once(at)
                .chain(
                    repeat(direction)
                        .scan(at, |coordinate, d| {
                            let new_coordinate = d.move_one(*coordinate)?;
                            *coordinate = new_coordinate;
                            if new_coordinate.0 >= self.width || new_coordinate.1 >= self.height {
                                None
                            } else {
                                Some(new_coordinate)
                            }
                        })
                        .take_while(|c| *self.at(*c).unwrap() == Space::Empty),
                )
                .last()
                .unwrap();

            *self.at_mut(coordinates).unwrap() = Space::Boulder
        }
    }

    fn move_all(&mut self, direction: Direction) -> &mut Self {
        let coordinates: Vec<(usize, usize)> = match direction {
            Direction::North => (0..self.width).cartesian_product(0..self.height).collect(),
            Direction::South => (0..self.width)
                .cartesian_product((0..self.height).rev())
                .collect(),
            Direction::East => ((0..self.width).rev())
                .cartesian_product(0..self.height)
                .collect(),
            Direction::West => (0..self.width)
                .cartesian_product((0..self.height).rev())
                .collect(),
        };
        coordinates
            .into_iter()
            .for_each(|c| self.move_one(c, direction));
        self
    }

    fn load(&self) -> u64 {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|c| {
                if self.at(c) == Some(&Space::Boulder) {
                    (self.height - c.1) as u64
                } else {
                    0
                }
            })
            .sum()
    }
}

fn solution_a(input: &[String]) -> u64 {
    let mut platform = Platform::from(input);
    platform.move_all(Direction::North);
    platform.load()
}

fn solution_b(input: &[String], steps: usize) -> u64 {
    let platform = Platform::from(input);
    let mut map = HashMap::<Platform, (Platform, usize)>::new();
    let (loop_p, loop_i) = (1..=steps)
        .scan((&mut map, platform), |(map, p), i| {
            if let Entry::Vacant(e) = map.entry(p.clone()) {
                p.move_all(Direction::North)
                    .move_all(Direction::West)
                    .move_all(Direction::South)
                    .move_all(Direction::East);
                e.insert((p.clone(), i));
                Some((p.clone(), i))
            } else {
                None
            }
        })
        .last()
        .unwrap();

    let (_back_p, back_i) = map.get(&loop_p).unwrap();

    let steps_from_back_i = (steps - loop_i) % (loop_i - back_i + 1);

    (0..steps_from_back_i)
        .fold(loop_p.clone(), |p, _| map.get(&p).unwrap().0.clone())
        .load()
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input = raw_to_strings(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    );
    assert_eq!(solution_a(&input), 136);
    let n = 99;
    let mut platform_s = Platform::from(input.as_slice());
    for _ in 0..n {
        platform_s
            .move_all(Direction::North)
            .move_all(Direction::West)
            .move_all(Direction::South)
            .move_all(Direction::East);
    }
    assert_eq!(solution_b(&input, n), platform_s.load());
    assert_eq!(solution_b(&input, 1000000000), 64);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input, 1000000000));
}
