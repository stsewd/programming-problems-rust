// https://www.hackerearth.com/practice/algorithms/searching/linear-search/practice-problems/algorithm/wet-clothes-625348cf/

use std::cmp;
use std::io::{self, BufRead};

fn solve(_max_times: i32, times: &[i32], clothes: &[i32]) -> i32 {
    let max_gap: i32 = times
        .iter()
        .zip(&times[1..])
        .fold(0, |acc, (t1, t2)| cmp::max(acc, t2 - t1));
    clothes.iter().filter(|&&c| c <= max_gap).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());

    let line: String = iter.next().unwrap();
    let mut numbers = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    // Skip n and m
    let g = numbers.nth(3).unwrap();

    let line: String = iter.next().unwrap();
    let times: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let line: String = iter.next().unwrap();
    let clothes: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", solve(g, &times, &clothes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(2, &[3, 5, 8], &[4, 1, 3]), 2);
    }
}
