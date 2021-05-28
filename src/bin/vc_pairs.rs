// https://www.hackerearth.com/practice/basic-programming/input-output/basics-of-input-output/practice-problems/algorithm/vc-pairs/

use std::collections::HashSet;
use std::io::{self, BufRead};

fn solve(string: &str) -> i32 {
    let mut n = 0;
    let vocals: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().copied().collect();
    let chars: Vec<char> = string.chars().collect();
    for (i, c) in chars.iter().take(chars.len() - 1).enumerate() {
        let next_vocal = chars.get(i + 1).unwrap();
        if !vocals.contains(c) && vocals.contains(next_vocal) {
            n += 1;
        }
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());

    let number: i32 = iter.next().unwrap().parse().unwrap();

    for _ in 0..number {
        let _n: i32 = iter.next().unwrap().parse().unwrap();
        let string: String = iter.next().unwrap();
        println!("{}", solve(&string));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, solve("bazeci"));
        assert_eq!(1, solve("abu"));
        assert_eq!(0, solve("o"));
    }
}
