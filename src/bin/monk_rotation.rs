// https://www.hackerearth.com/practice/codemonk/

use std::io::{self, BufRead};

fn solve(array: &[i32], k: u32) -> Vec<i32> {
    let mut solution: Vec<i32> = Vec::new();
    let n = array.len() as u32;
    let k = n - (k % n);
    for i in k..n {
        solution.push(array[i as usize]);
    }
    for i in 0..k {
        solution.push(array[i as usize]);
    }
    solution
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let t = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = iter.next().unwrap();
        let mut line = line.split_whitespace().map(|x| x.parse().unwrap());
        // skip reading n
        let k: u32 = line.nth(1).unwrap();
        let array: Vec<i32> = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let result = solve(&array, k)
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", &result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(&[1, 2, 3, 4, 5], 2), vec![4, 5, 1, 2, 3]);
        assert_eq!(solve(&[1, 2, 3, 4, 5], 5), vec![1, 2, 3, 4, 5]);
        assert_eq!(solve(&[1, 2, 3, 4, 5], 12), vec![4, 5, 1, 2, 3]);
    }
}
