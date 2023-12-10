use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

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
