// https://www.hackerearth.com/practice/basic-programming/input-output/basics-of-input-output/practice-problems/algorithm/is-zoo-f6f309e7/description/

use std::io::{self, BufRead};

fn solve(word: &str) -> bool {
    let z = word.chars().filter(|x| *x == 'z').count();
    let o = word.chars().count() - z;
    return z * 2 == o;
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let word = iter.next().unwrap();
    println!(
        "{}",
        match solve(&word) {
            true => "Yes",
            false => "No",
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, solve("zzzoooooo"));
        assert_eq!(false, solve("zzzooooooo"));
        assert_eq!(true, solve("zzzzoooooooo"));
        assert_eq!(false, solve("zzzooooo"));
    }
}
