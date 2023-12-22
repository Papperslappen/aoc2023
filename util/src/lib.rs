use std::cmp::{Eq, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::repeat;

pub mod parser;

pub fn get_input_rows() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lines().map(|line| line.unwrap()).collect()
}

pub fn raw_to_strings(s: &str) -> Vec<String> {
    s.split('\n').map(|s| s.to_string()).collect()
}

pub fn count_unique<T>(values: &[T]) -> Vec<(T, usize)>
where
    T: Clone + Eq + Hash,
{
    values
        .iter()
        .fold(HashMap::<T, usize>::new(), |mut m, x| {
            *m.entry(x.clone()).or_default() += 1;
            m
        })
        .into_iter()
        .collect::<Vec<_>>()
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum Direction {
    E,
    N,
    W,
    S,
}

impl Direction {
    pub fn move_in(
        &self,
        coordinate: (usize, usize),
        size: (usize, usize),
    ) -> Option<(usize, usize)> {
        self.move_steps(coordinate, 1, size)
    }

    pub fn move_steps(
        &self,
        (col, row): (usize, usize),
        steps: usize,
        (width, height): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::E => {
                Some((col + steps, row)).filter(|(col, row)| *col < width && *row < height)
            }
            Direction::N => row.checked_sub(steps).map(|row| (col, row)),
            Direction::W => col.checked_sub(steps).map(|col| (col, row)),
            Direction::S => {
                Some((col, row + steps)).filter(|(col, row)| *col < width && *row < height)
            }
        }
    }
}

pub struct Map<T> {
    fields: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Map<T> {
    pub fn new_from_strings(strings: &[String]) -> Self
    where
        T: From<char>,
    {
        let width = strings[0].len();
        let height = strings.len();
        let fields = strings
            .iter()
            .flat_map(|s| s.chars().map(|c| c.into()))
            .collect();
        Map {
            fields,
            width,
            height,
        }
    }

    pub fn at(&self, (col, row): (usize, usize)) -> Option<T>
    where
        T: Clone,
    {
        if col < self.width && row < self.height {
            self.fields.get(col + self.width * row).cloned()
        } else {
            None
        }
    }

    pub fn move_direction(
        &self,
        coordinate: (usize, usize),
        d: Direction,
    ) -> Option<((usize, usize), T)>
    where
        T: Clone,
    {
        d.move_in(coordinate, (self.width, self.height))
            .and_then(|c| self.at(c).map(|v| (c, v)))
    }
}

pub trait Dijkstra {
    type State: Clone + Eq + Hash + Debug + Ord;

    fn neighbors(&self, state: &Self::State) -> Vec<(Self::State, u64)>;

    fn solve(
        &self,
        start: Self::State,
        is_end: impl Fn(&Self::State) -> bool,
    ) -> (u64, Vec<Self::State>) {
        let mut costs: HashMap<Self::State, (Option<Self::State>, u64)> =
            HashMap::from([(start.clone(), (None, 0))]);
        // let mut stack: VecDeque<Self::State> = vec![start].into();
        let mut heap: BinaryHeap<(Reverse<u64>, Self::State)> =
            BinaryHeap::from(vec![(Reverse(0), start)]);

        while let Some((current_cost, current_state)) = heap.pop() {
            let neighbors = self
                .neighbors(&current_state)
                .into_iter()
                .collect::<Vec<_>>();

            for (neighbor, cost) in neighbors.into_iter() {
                match costs.entry(neighbor.clone()) {
                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        let cost_entry = o.get_mut();
                        let path_cost = current_cost.0 + cost;
                        if path_cost < cost_entry.1 {
                            *cost_entry = (Some(current_state.clone()), path_cost);
                            heap.push((Reverse(path_cost), neighbor));
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert((Some(current_state.clone()), current_cost.0 + cost));
                        heap.push((Reverse(current_cost.0 + cost), neighbor));
                    }
                }
            }
        }
        let end = costs
            .iter()
            .filter(|(s, _)| is_end(*s))
            .min_by(|(_, (_, costa)), (_, (_, costb))| costa.cmp(costb))
            .unwrap()
            .0;
        let cost = costs.get(end).unwrap().1;
        let path = repeat(())
            .scan(end.clone(), |s, _| {
                let ret = costs.get(s).unwrap().0.clone();
                if ret.is_some() {
                    *s = ret.clone().unwrap();
                }
                ret
            })
            .collect();
        (cost, path)
    }
}
