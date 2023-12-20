use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use pom::utf8::*;
use util::parser::space;

#[derive(PartialEq, Eq, Clone, Hash)]
enum Spring {
    Working,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Working,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            c => panic!("invalid character '{c}'"),
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Spring::Working => '.',
                Spring::Damaged => '#',
                Spring::Unknown => '?',
            }
        )
    }
}

#[derive(PartialEq, Eq, Hash)]
struct SpringPuzzle {
    springs: Vec<Spring>,
    groups: Vec<u64>,
}

impl From<&String> for SpringPuzzle {
    fn from(value: &String) -> Self {
        let springs_parser = (sym('.').repeat(1..).map(|_| Spring::Working)
            | one_of("#?").map(Spring::from))
        .repeat(1..);
        let groups_parser = list::<'_, char, u64, char>(util::parser::utf8::posint(), sym(','));
        let puzzleparser = space()
            * (springs_parser + space() * groups_parser).map(|(springs, groups)| SpringPuzzle {
                springs: [&[Spring::Working], springs.as_slice(), &[Spring::Working]].concat(),
                groups: [&[0], groups.as_slice()].concat(),
            });
        puzzleparser.parse(value.as_bytes()).unwrap()
    }
}

impl Display for SpringPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.springs.iter() {
            write!(f, "{}", s)?;
        }
        write!(f, " ")?;
        write!(f, "{}", self.groups.iter().map(|g| g.to_string()).join(","))
    }
}

impl SpringPuzzle {
    fn slice(&self, skip_springs: usize, skip_groups: usize) -> SpringPuzzle {
        SpringPuzzle {
            springs: self.springs.iter().skip(skip_springs).cloned().collect(),
            groups: {
                let mut groups = self.groups.clone();

                for _ in 0..skip_groups {
                    let first = groups.first_mut().unwrap();

                    if *first == 0 {
                        groups = groups.into_iter().skip(1).collect();
                    } else {
                        *first -= 1
                    }
                }
                groups
            },
        }
    }

    fn solve(self) -> u64 {
        let mut cache = HashMap::<SpringPuzzle, u64>::new();
        self._solve(&mut cache)
    }

    fn _solve(self, cache: &mut HashMap<SpringPuzzle, u64>) -> u64 {
        if let Some(cached_value) = cache.get(&self) {
            *cached_value
        } else {
            let value = match (self.springs.as_slice(), self.groups.as_slice()) {
                ([], []) => 1,
                (_, []) => 0,
                ([Spring::Damaged], [1]) => 1,
                ([Spring::Working], [0]) => 1,
                ([Spring::Damaged, ..], [a, ..]) => {
                    if *a == 0 {
                        0
                    } else {
                        self.slice(1, 1)._solve(cache)
                    }
                }
                ([Spring::Working, ..], [a, ..]) => {
                    if *a > 0 {
                        0
                    } else {
                        self.slice(1, 0)._solve(cache) + self.slice(1, 1)._solve(cache)
                    }
                }
                ([Spring::Unknown, rest @ ..], groups) => {
                    SpringPuzzle {
                        springs: [&[Spring::Damaged], rest].concat(),
                        groups: groups.to_vec(),
                    }
                    ._solve(cache)
                        + SpringPuzzle {
                            springs: [&[Spring::Working], rest].concat(),
                            groups: groups.to_vec(),
                        }
                        ._solve(cache)
                }
                ([], _) => 0,
            };
            //println!("{} value: {}", &self, value);
            cache.insert(self, value);
            value
        }

        // if self.springs.is_empty() && self.groups.is_empty() {
        //     1
        // }else if self.springs.is_empty() || self.groups.is_empty() {
        //     0
        // } else if *self.springs.first().unwrap() == Spring::Working {
        //     if *self.groups.first().unwrap() == 0 {
        //         self.slice(1,1)._solve(cache)
        //     } else {
        //         0
        //     }
        // } else if *self.springs.first().unwrap() == Spring::Damaged {
        //     if *self.groups.first().unwrap() == 0 {
        //         0
        //     } else {
        //         self.slice(1,1)._solve(cache)
        //     }
        // } else {
        //     let springs = self.springs.clone()
        // }
    }
}

fn solution_a(input: &[String]) -> u64 {
    input
        .iter()
        .map(SpringPuzzle::from)
        .map(|puzzle| puzzle.solve())
        .sum()
}

fn solution_b(input: &[String]) -> u64 {
    input
        .iter()
        .map(|s| {
            let mut iter = s.split(' ');
            let springstring = [iter.next().unwrap()].iter().cycle().take(5).join("?");
            let groupstring = [iter.next().unwrap()].iter().cycle().take(5).join(",");
            SpringPuzzle::from(&format!("{springstring} {groupstring}"))
        })
        .map(|puzzle| {
            //println!("PUZZLE: {} ", puzzle);
            puzzle.solve()
        })
        .sum()
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input = raw_to_strings("?###???????? 3,2,1");
    assert_eq!(solution_a(&input), 10);
    assert_eq!(solution_b(&input), 506250);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
