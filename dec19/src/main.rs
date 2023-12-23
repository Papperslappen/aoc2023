use std::{collections::HashMap, iter::once};

use pom::utf8::*;
use util::parser::utf8::{posint, space};

#[derive(Debug, Clone, PartialEq)]
enum Property {
    X,
    M,
    A,
    S,
}
impl From<char> for Property {
    fn from(value: char) -> Self {
        use Property::*;
        match value {
            'x' => X,
            'm' => M,
            'a' => A,
            's' => S,
            _ => panic!("invalid character"),
        }
    }
}
#[derive(Debug, Clone)]
enum RuleCriteria {
    Less(Property, u64),
    More(Property, u64),
    Always,
}

#[derive(Clone, Debug)]
struct Rule {
    criteria: RuleCriteria,
    destination: Label,
}

impl Rule {
    fn filter(&self, part: &Part) -> Option<Label> {
        if match &self.criteria {
            RuleCriteria::Less(p, n) => part.get(p) < *n,
            RuleCriteria::More(p, n) => part.get(p) > *n,
            RuleCriteria::Always => true,
        } {
            Some(self.destination.clone())
        } else {
            None
        }
    }
}

type Rules = Vec<Rule>;
type Label = String;

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get(&self, property: &Property) -> u64 {
        match property {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }
}

fn label_parser<'a>() -> Parser<'a, Label> {
    is_a(|c| c.is_ascii_alphabetic())
        .repeat(1..)
        .collect()
        .map(|s| s.to_string())
}

fn rule_parser<'a>() -> Parser<'a, Rule> {
    (one_of("xmas") + one_of("<>") + posint() + sym(':') * label_parser()).map(
        |(((xmas, ltgt), number), label)| Rule {
            criteria: match ltgt {
                '<' => RuleCriteria::Less(xmas.into(), number),
                '>' => RuleCriteria::More(xmas.into(), number),
                _ => panic!("invalid character"),
            },
            destination: label,
        },
    ) | label_parser().map(|l| Rule {
        criteria: RuleCriteria::Always,
        destination: l,
    })
}

fn rules_parser<'a>() -> Parser<'a, (Label, Rules)> {
    label_parser() - sym('{') + list::<'a, char, _, _>(rule_parser(), sym(',')) - sym('}')
}

fn part_parser<'a>() -> Parser<'a, Part> {
    (sym('{') * seq("x=") * posint()
        + seq(",m=") * posint()
        + seq(",a=") * posint()
        + seq(",s=") * posint()
        - sym('}'))
    .map(|(((x, m), a), s)| Part { x, m, a, s })
}

fn follow_the_rules(rules: &HashMap<Label, Rules>, part: &Part) -> Label {
    let mut current_label = "in".to_string();
    while current_label != "A" && current_label != "R" {
        let rule = rules.get(&current_label).unwrap();
        current_label = rule.iter().find_map(|rule| rule.filter(part)).unwrap();
    }
    current_label
}

fn rules_tree_size(
    rules: &HashMap<Label, Rules>,
    label: Label,
    partition: Vec<RuleCriteria>,
) -> u64 {
    match label.as_str() {
        "R" => 0,
        "A" => partition_size(partition),
        _ => {
            let rule = rules.get(&label).unwrap();
            rule.iter()
                .scan(partition, |p, r| match &r.criteria {
                    RuleCriteria::Less(property, n) => {
                        let branch = p
                            .iter()
                            .cloned()
                            .chain(once(r.criteria.clone()))
                            .collect::<Vec<_>>();
                        p.push(RuleCriteria::More(property.clone(), n - 1));
                        Some(rules_tree_size(rules, r.destination.clone(), branch))
                    }
                    RuleCriteria::More(property, n) => {
                        let branch = p
                            .iter()
                            .cloned()
                            .chain(once(r.criteria.clone()))
                            .collect::<Vec<_>>();
                        p.push(RuleCriteria::Less(property.clone(), n + 1));
                        Some(rules_tree_size(rules, r.destination.clone(), branch))
                    }
                    RuleCriteria::Always => {
                        Some(rules_tree_size(rules, r.destination.clone(), p.clone()))
                    }
                })
                .sum()
        }
    }
}

fn partition_size(partition: Vec<RuleCriteria>) -> u64 {
    use Property::*;
    [X, M, A, S]
        .into_iter()
        .map(|property| {
            partition
                .iter()
                .filter(|criteria| match criteria {
                    RuleCriteria::Always => panic!("invalid criteria"),
                    RuleCriteria::Less(p, _) | RuleCriteria::More(p, _) => *p == property,
                })
                .fold((1, 4000), |(lower, upper), criteria| match criteria {
                    RuleCriteria::Less(_, n) => (lower, upper.min(*n - 1)),
                    RuleCriteria::More(_, n) => (lower.max(*n + 1), upper),
                    RuleCriteria::Always => panic!("invalid criteria"),
                })
        })
        .map(|(lower, upper)| (upper - lower + 1).max(0))
        .product()
}

fn solution_a(input: &str) -> u64 {
    let parser = list::<'_, char, _, _>(rules_parser(), space())
        .map(|rules| rules.into_iter().collect::<HashMap<_, _>>())
        + space() * list::<'_, char, _, _>(part_parser(), space());
    let (rules, parts) = parser.parse(input.as_bytes()).unwrap();
    parts
        .iter()
        .filter(|part| follow_the_rules(&rules, part) == "A")
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn solution_b(input: &str) -> u64 {
    let parser = list::<'_, char, _, _>(rules_parser(), space())
        .map(|rules| rules.into_iter().collect::<HashMap<_, _>>());
    let rules = parser.parse(input.as_bytes()).unwrap();
    rules_tree_size(&rules, "in".to_string(), vec![])
}

#[test]
fn test_solutions() {
    let input1 = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    assert_eq!(solution_a(input1), 19114);
    assert_eq!(solution_b(input1), 167409079868000);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows().join("\n");
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
