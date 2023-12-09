use std::collections::HashMap;

use pom::utf8::*;
use util::gcd;

type Tag = String;

fn space<'a>() -> Parser<'a, ()> {
    one_of("\n \t\r").repeat(0..).discard()
}

fn directions<'a>() -> Parser<'a, Vec<Direction>> {
    (sym('L').map(|_| Direction::Left) | sym('R').map(|_| Direction::Right)).repeat(1..)
}

fn tag<'a>() -> Parser<'a, Tag> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")
        .repeat(3)
        .map(|s| s.into_iter().collect())
}

fn pair_of_tags<'a>() -> Parser<'a, (Tag, Tag)> {
    sym('(') * space() * tag() - space() - sym(',') - space() + tag() - space() - sym(')')
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct WastelandMap {
    directions: Vec<Direction>,
    map: HashMap<Tag, (Tag, Tag)>,
}

impl WastelandMap {
    fn new_from_str(s: &str) -> WastelandMap {
        let parser = directions()
            + space()
                * ((tag() + space() * sym('=') * space() * pair_of_tags() - space()).repeat(1..));
        let (directions, entries) = parser.parse(s.as_bytes()).unwrap();

        WastelandMap {
            directions,
            map: entries.into_iter().collect(),
        }
    }

    fn walk_until(&self, start_node: Tag, end_condition: impl Fn(&String) -> bool) -> u64 {
        let (walk, _tag) = self
            .directions
            .iter()
            .cycle()
            .scan(start_node, |tag, direction| {
                if end_condition(tag) {
                    None
                } else {
                    let pair = self.map.get(tag).unwrap();
                    let newtag = match direction {
                        Direction::Left => pair.0.clone(),
                        Direction::Right => pair.1.clone(),
                    };
                    *tag = newtag;
                    Some(tag.clone())
                }
            })
            .enumerate()
            .last()
            .unwrap();
        walk as u64 + 1
    }

    fn walk(&self, start_node: Tag) -> u64 {
        self.walk_until(start_node, |s| s == "ZZZ")
    }
}

fn solution_a(input: &str) -> u64 {
    let map = WastelandMap::new_from_str(input);
    map.walk("AAA".to_string())
}

fn solution_b(input: &str) -> u64 {
    let map = WastelandMap::new_from_str(input);
    let start_nodes = map
        .map
        .keys()
        .filter(|s| s.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();
    start_nodes
        .iter()
        .map(|tag| map.walk_until(tag.clone(), |s| s.ends_with('Z')))
        .reduce(|x, y| x * y / gcd(x, y))
        .unwrap()
}

#[test]
fn test_solutions() {
    let input1 = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

    assert_eq!(solution_a(input1), 2);
    let input2 = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
    assert_eq!(solution_a(input2), 6);

    let input3 = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";
    assert_eq!(solution_b(input3), 6);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows().join("\n");
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
