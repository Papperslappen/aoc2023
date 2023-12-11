use pom::utf8::*;
use std::collections::{HashMap, HashSet, VecDeque};
struct Loopfinder {
    map: HashMap<(i64, i64), char>,
}

fn neighbors(pipe: char, coordnate: (i64, i64)) -> Vec<(i64, i64)> {
    match pipe {
        'S' => vec![],
        '.' => vec![],
        '|' => vec![(-1, 0), (1, 0)],
        '-' => vec![(0, -1), (0, 1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(1, 0), (0, -1)],
        'F' => vec![(1, 0), (0, 1)],
        _ => panic!("invalid character"),
    }
    .iter()
    .map(|(drow, dcol)| (coordnate.0 + drow, coordnate.1 + dcol))
    .collect()
}

impl Loopfinder {
    fn new(map: HashMap<(i64, i64), char>) -> Loopfinder {
        Loopfinder { map }
    }

    fn find_loop(self, start: (i64, i64)) -> HashMap<(i64, i64), u32> {
        let mut open = [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .iter()
            .map(|(drow, dcol)| (start.0 + drow, start.1 + dcol))
            .filter(|coordinte| {
                self.map
                    .get_key_value(coordinte)
                    .map(|(coordinate, c)| neighbors(*c, *coordinate).iter().any(|c| *c == start))
                    .unwrap_or(false)
            })
            .map(|coordinate| (coordinate, 0_u32))
            .collect::<VecDeque<_>>();
        let mut result = HashMap::new();
        result.insert(start, 0);
        while let Some((node, distance)) = open.pop_front() {
            let neighbors = neighbors(*self.map.get(&node).unwrap(), node)
                .into_iter()
                .filter(|c| !result.contains_key(c))
                .collect::<Vec<_>>();

            neighbors
                .into_iter()
                .for_each(|coordinate| open.push_back((coordinate, distance + 1)));

            result.entry(node).or_insert(distance + 1);
        }
        result
    }
}

fn solution_a(input: &[String]) -> u32 {
    let map = input
        .iter()
        .enumerate()
        .flat_map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(move |(col, c)| ((row as i64, col as i64), c))
        })
        .collect::<HashMap<(i64, i64), char>>();

    let start = map
        .iter()
        .find(|(_, v)| **v == 'S')
        .map(|(k, _)| *k)
        .unwrap();

    let pipeloop = Loopfinder::new(map).find_loop(start);
    *pipeloop.values().max().unwrap()
}

fn solution_b(input: &[String]) -> u32 {
    let map = input
        .iter()
        .enumerate()
        .flat_map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(move |(col, c)| ((row as i64, col as i64), c))
        })
        .collect::<HashMap<(i64, i64), char>>();

    let start = map
        .iter()
        .find(|(_, v)| **v == 'S')
        .map(|(k, _)| *k)
        .unwrap();

    let pipeloop = Loopfinder::new(map.clone()).find_loop(start);

    let loopcoords = pipeloop.keys().cloned().collect::<HashSet<_>>();
    let othercoords = map
        .keys()
        .cloned()
        .collect::<HashSet<_>>()
        .difference(&loopcoords)
        .cloned()
        .collect::<Vec<_>>();

    let result = othercoords
        .iter()
        .map(|(row, col)| {
            (1..)
                .map(|i| {
                    if col >= &start.1 {
                        (*row, *col + i)
                    } else {
                        (*row, *col - i)
                    }
                })
                .map(|coordinate| map.get_key_value(&coordinate))
                .take_while(Option::is_some)
                .map(|c| c.unwrap())
                .filter(|(coordinate, _)| loopcoords.contains(coordinate))
                .map(|(_, c)| *c)
                .filter(|c| *c != '-')
                .collect::<String>()
        })
        .filter(|s| !s.is_empty())
        .map(|s| {
            let parser = (sym('|').map(|_| 1_u32)
                | (seq("FJ") | seq("JF") | seq("7L") | seq("L7")).map(|_| 1)
                | (seq("LJ") | seq("JL") | seq("7F") | seq("F7")).map(|_| 0))
            .repeat(0..);
            parser.parse(s.as_bytes()).unwrap().iter().sum::<u32>() % 2
        })
        .sum();
    result
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input1 = raw_to_strings(
        ".....
.S-7.
.|.|.
.L-J.
.....",
    );
    let input2 = raw_to_strings(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
    );
    assert_eq!(solution_a(&input1), 4);
    assert_eq!(solution_a(&input2), 8);

    let input3 = raw_to_strings(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
    );

    assert_eq!(solution_b(&input1), 1);
    assert_eq!(solution_b(&input3), 8);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
