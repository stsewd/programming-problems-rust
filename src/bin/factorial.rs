// https://www.hackerearth.com/practice/basic-programming/input-output/basics-of-input-output/practice-problems/algorithm/find-factorial/

use std::io::{self, BufRead};

fn solve(number: u32) -> u32 {
    (1..=number).product()
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());

    let number = iter.next().unwrap().parse().unwrap();
    println!("{}", solve(number));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(1), 1);
        assert_eq!(solve(2), 2);
        assert_eq!(solve(3), 6);
        assert_eq!(solve(4), 24);
        assert_eq!(solve(5), 120);
    }
}
