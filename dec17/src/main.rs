use std::ops::{Add, RangeInclusive};

use util::{Dijkstra, Direction, Map};

#[derive(Clone, Debug)]
struct MapCost(u64);

impl Add for MapCost {
    type Output = MapCost;

    fn add(self, rhs: Self) -> Self::Output {
        MapCost(self.0 + rhs.0)
    }
}

impl From<char> for MapCost {
    fn from(value: char) -> Self {
        let cost = value.to_digit(10).unwrap() as u64;
        MapCost(cost)
    }
}

struct CrucibleMap {
    map: Map<MapCost>,
    steps: RangeInclusive<usize>,
}

impl Dijkstra for CrucibleMap {
    type State = ((usize, usize), Option<Direction>);

    fn neighbors(
        &self,
        (coordinate, direction): &Self::State,
    ) -> std::vec::Vec<(Self::State, u64)> {
        let coordinate = *coordinate;
        let directions = if let Some(previous_direction) = direction {
            match previous_direction {
                Direction::E | Direction::W => vec![Direction::S, Direction::N],
                Direction::N | Direction::S => vec![Direction::E, Direction::W],
            }
        } else {
            vec![Direction::S, Direction::E, Direction::N, Direction::W]
        };

        let neighbors = directions
            .iter()
            .flat_map(|direction| {
                (1..)
                    .scan(
                        ((coordinate, direction), 0),
                        |((coordinate, direction), cost), _step| {
                            if let Some(new) = self.map.move_direction(*coordinate, **direction) {
                                *coordinate = new.clone().0;
                                *cost += new.1 .0;
                                Some(((*coordinate, **direction), *cost))
                            } else {
                                None
                            }
                        },
                    )
                    .skip(self.steps.start() - 1)
                    .take(self.steps.end() - self.steps.start() + 1)
                    .map(|((c, direction), cost)| ((c, Some(direction)), cost))
            })
            .collect();
        neighbors
    }
}

fn solution_a(input: &[String]) -> u64 {
    let map = CrucibleMap {
        map: Map::<MapCost>::new_from_strings(input),
        steps: 1..=3,
    };
    let (cost, _path) = map.solve(((0, 0), None), |(c, _)| {
        *c == (map.map.width - 1, map.map.height - 1)
    });
    println!("path: {:?}", _path);
    cost
}

fn solution_b(input: &[String]) -> u64 {
    let map = CrucibleMap {
        map: Map::<MapCost>::new_from_strings(input),
        steps: 4..=10,
    };
    let (cost, _path) = map.solve(((0, 0), None), |(c, _)| {
        *c == (map.map.width - 1, map.map.height - 1)
    });
    cost
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input1 = raw_to_strings(
        r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
    );
    assert_eq!(solution_a(&input1), 102);
    assert_eq!(solution_b(&input1), 94);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
