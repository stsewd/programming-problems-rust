// https://www.hackerearth.com/practice/basic-programming/input-output/basics-of-input-output/practice-problems/algorithm/is-zoo-f6f309e7/description/

use std::io::{self, BufRead};

fn solve(word: &str) -> bool {
    let z = word.chars().filter(|&x| x == 'z').count();
    let o = word.chars().count() - z;
    z * 2 == o
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let word = iter.next().unwrap();
    println!("{}", if solve(&word) { "Yes" } else { "No" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(solve("zzzoooooo"));
        assert!(!solve("zzzooooooo"));
        assert!(solve("zzzzoooooooo"));
        assert!(!solve("zzzooooo"));
    }
}
