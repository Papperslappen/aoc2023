use itertools::Itertools;

#[derive(Default)]
struct StarMap {
    stars: Vec<(u64, u64)>,
    width: u64,
    height: u64,
    empty_space: (Vec<u64>, Vec<u64>),
}

impl StarMap {
    fn empty_cols_rows(&mut self) {
        self.empty_space = (
            (0..self.width)
                .filter(|n| self.stars.iter().all(|c| c.0 != *n))
                .collect(),
            (0..self.height)
                .filter(|n| self.stars.iter().all(|c| c.1 != *n))
                .collect(),
        );
    }

    fn empty_between(&self, (col1, row1): (u64, u64), (col2, row2): (u64, u64)) -> u64 {
        let (empty_cols, empty_rows) = &self.empty_space;
        empty_cols
            .iter()
            .filter(|col| col1.min(col2) < **col && **col < col1.max(col2))
            .count() as u64
            + empty_rows
                .iter()
                .filter(|row| row1.min(row2) < **row && **row < row1.max(row2))
                .count() as u64
    }

    fn total_pair_distance(&mut self, scale: u64) -> u64 {
        self.empty_cols_rows();
        self.stars
            .iter()
            .combinations(2)
            .map(|s| {
                s[1].0.abs_diff(s[0].0)
                    + s[1].1.abs_diff(s[0].1)
                    + (scale - 1) * self.empty_between(*s[0], *s[1])
            })
            .sum::<u64>()
    }
}

fn solution_a(input: &[String]) -> u64 {
    let height = input.len() as u64;
    let width = input.first().unwrap().len() as u64;
    let stars = input
        .iter()
        .enumerate()
        .flat_map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(move |(col, c)| ((col as u64, row as u64), c))
                .filter(|(_, c)| *c == '#')
                .map(|(coordinate, _)| coordinate)
        })
        .collect::<Vec<_>>();
    let mut starmap = StarMap {
        stars,
        height,
        width,
        ..Default::default()
    };

    starmap.total_pair_distance(2)
}

fn solution_b(input: &[String]) -> u64 {
    let height = input.len() as u64;
    let width = input.first().unwrap().len() as u64;
    let stars = input
        .iter()
        .enumerate()
        .flat_map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(move |(col, c)| ((col as u64, row as u64), c))
                .filter(|(_, c)| *c == '#')
                .map(|(coordinate, _)| coordinate)
        })
        .collect::<Vec<_>>();

    let mut starmap = StarMap {
        stars,
        height,
        width,
        ..Default::default()
    };
    starmap.total_pair_distance(1000000)
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input = raw_to_strings(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
    );
    assert_eq!(solution_a(&input), 374);
    //assert_eq!(solution_b(&input), 0);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
