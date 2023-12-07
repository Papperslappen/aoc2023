use pom::{char_class::alpha, parser::*};
use util::{
    self,
    parser::{posint, space},
};

#[derive(Debug)]
struct MapRange {
    dest: u64,
    source: u64,
    range: u64,
}

impl MapRange {
    fn map(&self, input: u64) -> Option<u64> {
        if input >= self.source && input - self.source <= self.range {
            Some(self.dest + (input - self.source))
        } else {
            None
        }
    }

    fn range_overlap(&self, seed_range: &SeedRange) -> Option<SeedRange> {
        if seed_range.source < self.source && self.source < seed_range.source + seed_range.range {
            Some(SeedRange {
                source: self.source,
                range: self
                    .range
                    .min(seed_range.range - (self.source - seed_range.source)),
            })
        } else if seed_range.source >= self.source && seed_range.source < self.source + self.range {
            Some(SeedRange {
                source: seed_range.source,
                range: seed_range
                    .range
                    .min(self.range - (seed_range.source - self.source)),
            })
        } else {
            None
        }
    }

    fn map_range(&self, seed_range: &SeedRange) -> Option<Vec<SeedRange>> {
        if let Some(overlap) = self.range_overlap(seed_range) {
            let mut result = seed_range.difference(&overlap);
            result.push(SeedRange {
                source: self.map(overlap.source).unwrap(),
                range: overlap.range,
            });
            Some(result)
        } else {
            None
        }
    }
}

#[test]
fn test_overlap() {
    let range = MapRange {
        dest: 10000,
        source: 5,
        range: 5,
    };
    let seed_range1 = SeedRange {
        source: 1,
        range: 5,
    };
    let seed_range2 = SeedRange {
        source: 10,
        range: 100,
    };
    let seed_range3 = SeedRange {
        source: 6,
        range: 100,
    };
    let seed_range4 = SeedRange {
        source: 0,
        range: 100,
    };
    let seed_range5 = SeedRange {
        source: 5,
        range: 1,
    };
    assert_eq!(
        range.range_overlap(&seed_range1),
        Some(SeedRange {
            source: 5,
            range: 1
        })
    );
    assert_eq!(range.range_overlap(&seed_range2), None);
    assert_eq!(
        range.range_overlap(&seed_range3),
        Some(SeedRange {
            source: 6,
            range: 4
        })
    );
    assert_eq!(
        range.range_overlap(&seed_range4),
        Some(SeedRange {
            source: 5,
            range: 5
        })
    );
    assert_eq!(
        range.range_overlap(&seed_range5),
        Some(SeedRange {
            source: 5,
            range: 1
        })
    );
    assert_eq!(
        range.range_overlap(&SeedRange {
            source: 1,
            range: 4
        }),
        None
    );
    assert_eq!(
        range.range_overlap(&SeedRange {
            source: 11,
            range: 100
        }),
        None
    );
}

#[derive(Debug)]
struct Map {
    #[allow(unused)]
    name: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, input: u64) -> u64 {
        let result = self
            .ranges
            .iter()
            .filter_map(|range| range.map(input))
            .next()
            .unwrap_or(input);
        result
    }

    fn map_range(&self, seed_range: &SeedRange) -> Vec<SeedRange> {
        let overlap = self
            .ranges
            .iter()
            .filter_map(|range| {
                range
                    .range_overlap(seed_range)
                    .map(|seed_range| (seed_range, range))
            })
            .collect::<Vec<_>>();
        let mut uncovered = overlap
            .iter()
            .fold(vec![*seed_range], |uncovered, overlap| {
                uncovered
                    .iter()
                    .flat_map(|range| range.difference(&overlap.0))
                    .collect()
            });
        let mut ranges = overlap
            .iter()
            .flat_map(|(seed_range, map_range)| map_range.map_range(seed_range).unwrap())
            .collect::<Vec<_>>();
        println!(
            "OVERLAP: {:?}, UNCOVERED: {:?}, RANGES: {:?}",
            overlap, uncovered, ranges
        );
        ranges.append(&mut uncovered);

        println!(
            "MAP {:?} is mapping {:?} onto {:?} ",
            self, seed_range, ranges
        );
        ranges
    }

    fn map_ranges(&self, seed_ranges: &[SeedRange]) -> Vec<SeedRange> {
        let result = seed_ranges
            .iter()
            .flat_map(|seed_range| self.map_range(seed_range))
            .collect::<Vec<_>>();
        result
    }
}

#[test]
fn test_map_range() {
    let map = Map {
        name: "light-to-temperature".to_owned(),
        ranges: vec![
            MapRange {
                dest: 45,
                source: 77,
                range: 23,
            },
            MapRange {
                dest: 81,
                source: 45,
                range: 19,
            },
            MapRange {
                dest: 68,
                source: 64,
                range: 13,
            },
        ],
    };
    let range = SeedRange {
        source: 74,
        range: 14,
    };
    let _result = map.map_range(&range);
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SeedRange {
    source: u64,
    range: u64,
}

impl SeedRange {
    fn difference(&self, other: &SeedRange) -> Vec<SeedRange> {
        let mut result: Vec<SeedRange> = vec![];
        if self.source < other.source {
            result.push(SeedRange {
                source: self.source,
                range: self.range.min(other.source - self.source),
            });
        }
        if other.source + other.range < self.source + self.range {
            result.push(SeedRange {
                source: other.source + other.range,
                range: (self.source + self.range) - (other.source + other.range),
            })
        }
        result
    }
}

#[test]
fn test_difference() {
    let a = SeedRange {
        source: 10,
        range: 3,
    };
    let b = SeedRange {
        source: 11,
        range: 1,
    };
    assert_eq!(
        a.difference(&b),
        vec![
            SeedRange {
                source: 10,
                range: 1
            },
            SeedRange {
                source: 12,
                range: 1
            }
        ]
    );
    let a = SeedRange {
        source: 1,
        range: 100,
    };
    let b = SeedRange {
        source: 101,
        range: 100,
    };
    assert_eq!(a.difference(&b), vec![a]);
}

impl Almanac {
    fn seeds_to_soil(&self) -> Vec<u64> {
        self.maps.iter().fold(self.seeds.clone(), |numbers, map| {
            numbers.iter().map(|n| map.map(*n)).collect()
        })
    }
}

#[derive(Debug)]
struct NewAlmanac {
    seed_ranges: Vec<SeedRange>,
    maps: Vec<Map>,
}

impl From<Almanac> for NewAlmanac {
    fn from(val: Almanac) -> Self {
        let seed_ranges = val
            .seeds
            .chunks(2)
            .map(|chunk| SeedRange {
                source: chunk[0],
                range: chunk[1],
            })
            .collect();
        NewAlmanac {
            seed_ranges,
            maps: val.maps,
        }
    }
}

impl NewAlmanac {
    fn seeds_to_soil(&self) -> Vec<SeedRange> {
        self.maps
            .iter()
            .fold(self.seed_ranges.clone(), |seed_ranges, map| {
                println!("SEED RANGES {:?}", &seed_ranges);
                map.map_ranges(&seed_ranges)
            })
    }
}

fn map_range<'a>() -> Parser<'a, u8, MapRange> {
    (posint() - space() + posint() - space() + posint() - space()).map(
        |((first, second), third)| MapRange {
            dest: first as u64,
            source: second as u64,
            range: third as u64,
        },
    )
}

fn map<'a>() -> Parser<'a, u8, Map> {
    ((is_a(alpha) | sym(b'-'))
        .repeat(1..)
        .map(|r| String::from_utf8(r).unwrap())
        - (space() + seq(b"map:") + space())
        - space()
        + map_range().repeat(1..)
        - space())
    .map(|(name, ranges)| Map { name, ranges })
}

fn almanac<'a>() -> Parser<'a, u8, Almanac> {
    (seq(b"seeds:") * space() * (posint() - space()).map(|s| s as u64).repeat(1..)
        + (map() - space()).repeat(1..)
        - end())
    .map(|(seeds, maps)| Almanac { seeds, maps })
}

fn solution_a(input: &[String]) -> u64 {
    let input = input.join("\n");
    let parser = almanac();
    let almanac = parser.parse(input.as_bytes()).unwrap();
    let soil = almanac.seeds_to_soil();
    *soil.iter().min().unwrap()
}

fn solution_b(input: &[String]) -> u64 {
    let input = input.join("\n");
    let parser = almanac();
    let new_almanac: NewAlmanac = parser.parse(input.as_bytes()).unwrap().into();
    let mut location = new_almanac.seeds_to_soil();
    location.sort_by(|a, b| a.source.cmp(&b.source));
    println!("location range {:?}", location);
    location.iter().map(|s| s.source).min().unwrap()
}

// 74 -= 87 (77-=89,45-=55,)

#[test]
fn test_solutions() {
    let input = util::raw_to_strings(
        r"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4
        ",
    );
    assert_eq!(solution_a(&input), 35);
    assert_eq!(solution_b(&input), 46);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
