pub mod parser;

pub fn get_input_rows() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lines().map(|line| line.unwrap()).collect()
}

pub fn raw_to_strings(s: &str) -> Vec<String> {
    s.split('\n').map(|s| s.to_string()).collect()
}
