#[derive(Clone)]
enum Place {
    Ash,
    Rock,
}

impl From<char> for Place {
    fn from(value: char) -> Self {
        match value {
            '.' => Place::Ash,
            '#' => Place::Rock,
            _ => panic!(),
        }
    }
}

struct Square {
    places: Vec<Vec<Place>>,
}

impl From<String> for Square {
    fn from(value: String) -> Self {
        Square {
            places: value
                .split('\n')
                .map(|s| s.chars().map(Place::from).collect::<Vec<_>>())
                .collect(),
        }
    }
}

impl Square {
    fn row_values(&self) -> Vec<u64> {
        self.places
            .iter()
            .map(|row| {
                row.iter().fold(0_u64, |acc, place| {
                    (acc << 1)
                        + match place {
                            Place::Ash => 0,
                            Place::Rock => 1,
                        }
                })
            })
            .collect()
    }

    fn col_values(&self) -> Vec<u64> {
        (0..self.places[0].len())
            .map(|i| {
                self.places
                    .iter()
                    .map(|row| row[i].clone())
                    .fold(0_u64, |acc, place| {
                        (acc << 1)
                            + match place {
                                Place::Ash => 0,
                                Place::Rock => 1,
                            }
                    })
            })
            .collect()
    }

    fn get_value_smudge_removed(&self) -> u64 {
        (find_symmetry_with_single_bit_swap(&self.col_values()).unwrap_or(0)
            + 100 * find_symmetry_with_single_bit_swap(&self.row_values()).unwrap_or(0)) as u64
    }

    fn get_value(&self) -> u64 {
        (find_symmetry(&self.col_values()).unwrap_or(0)
            + 100 * find_symmetry(&self.row_values()).unwrap_or(0)) as u64
    }
}

fn find_symmetry(values: &[u64]) -> Option<usize> {
    (1..values.len()).find(|mid| {
        (1..=(values.len().abs_diff(*mid).min(*mid)))
            .all(|i| values[mid - i] == values[mid + i - 1])
    })
}

fn find_symmetry_with_single_bit_swap(values: &[u64]) -> Option<usize> {
    (1..values.len()).filter(|mid| {
        (1..=(values.len().abs_diff(*mid).min(*mid)))
        .map(|i| (values[mid-i],values[mid+i-1])).all(|(a,b)|{
            a==b || find_set_bit(a^b).is_some() 
        })
    }).find(|mid|{       
        (1..=(values.len().abs_diff(*mid).min(*mid)))
        .map(|i| (values[mid-i],values[mid+i-1]))
        .filter(|(a,b)| {
            find_set_bit(a^b).is_some()}).count() == 1})
}

fn find_set_bit(n: u64)->Option<usize>{
    if n.count_ones() == 1 {
        Some(n.ilog2() as usize)
    } else {
        None
    }
}

fn solution_a(input: &[String]) -> u64 {
    let squares = input
        .split(|s| s.is_empty())
        .map(|s| Square::from(s.join("\n")).get_value())
        .collect::<Vec<_>>();
    squares.iter().sum()
}

fn solution_b(input: &[String]) -> u64 {
    let squares = input
        .split(|s| s.is_empty())
        .map(|s| Square::from(s.join("\n")).get_value_smudge_removed())
        .collect::<Vec<_>>();
    squares.iter().sum()
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input = raw_to_strings(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    );
    assert_eq!(solution_a(&input), 405);
    assert_eq!(solution_b(&input), 400);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
