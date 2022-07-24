// https://www.hackerearth.com/problem/algorithm/maximum-binary-number-cb9a58c1/

use std::cmp::Ordering;
use std::io::{self, BufRead};

fn get_iter(start: usize, max: usize) -> impl Iterator<Item = usize> {
    (start..max).chain(0..start)
}

fn compare(a: usize, b: usize, binary_number: &[char]) -> Ordering {
    let iter_a = get_iter(a, binary_number.len());
    let iter_b = get_iter(b, binary_number.len());
    for (ia, ib) in iter_a.zip(iter_b) {
        if binary_number[ia] > binary_number[ib] {
            return Ordering::Greater;
        }
        if binary_number[ia] < binary_number[ib] {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn solve(k: usize, binary_number: &str) -> usize {
    let binary_number: Vec<char> = binary_number.chars().collect();
    let len = binary_number.len();
    let mut max = 0;
    let mut period = 0;
    for i in 1..len {
        let result = compare(i, max, &binary_number);
        if result == Ordering::Greater {
            max = i;
        } else if result == Ordering::Equal {
            period = i - max;
            break;
        }
    }

    if period == 0 {
        max + (k - 1) * len
    } else {
        max + (k - 1) * period
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());

    let t: u32 = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = iter.next().unwrap();
        let mut numbers = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let k = numbers.nth(1).unwrap();
        let binary_number = iter.next().unwrap();
        println!("{}", solve(k, &binary_number));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(2, "10101"), 9);
        assert_eq!(solve(2, "010101"), 3);
    }
}
