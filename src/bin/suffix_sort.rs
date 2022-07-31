// https://www.hackerearth.com/problem/algorithm/monk-and-suffix-sort-ebacdaf5/
use std::io::{self, BufRead};

fn solve(string: &str, k: usize) -> &str {
    let mut suffixes: Vec<&str> = Vec::with_capacity(string.len());
    for i in 0..string.len() {
        let suffix = &string[i..];
        suffixes.push(suffix);
    }
    suffixes.sort_unstable();
    suffixes[k - 1]
}

pub fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().map(|x| x.unwrap()).next().unwrap();
    let mut iter = line.split_whitespace();
    let string = iter.next().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    println!("{}", solve(string, k));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("aacb", 3), "b");
        assert_eq!(solve("aabbcc", 3), "bbcc");
    }
}
