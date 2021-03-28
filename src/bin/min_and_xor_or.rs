// https://www.hackerearth.com/practice/data-structures/arrays/1-d/practice-problems/algorithm/minimum-and-xor-or-6a05bbd4/

use std::io::{self, BufRead};

fn solve(array: &[u32]) -> u32 {
    let mut sorted: Vec<u32> = array.to_vec();
    sorted.sort_unstable();
    let n = sorted.len();
    let mut min = sorted.get(0).unwrap() ^ sorted.get(1).unwrap();
    for i in 0..n - 1 {
        let (a, b) = (sorted.get(i).unwrap(), sorted.get(i + 1).unwrap());
        let result = a ^ b;
        if result == 0 {
            return 0;
        }
        if result < min {
            min = result;
        }
    }
    min
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let t = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let array: Vec<u32> = iter
            // skip n
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        println!("{}", solve(&array));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(&[1, 2, 3, 4, 5]), 1);
        assert_eq!(solve(&[2, 4, 7]), 3);
    }
}
