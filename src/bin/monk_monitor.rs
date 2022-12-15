// https://www.hackerearth.com/problem/algorithm/monk-being-monitor-709e0fd3/
use std::io::{self, BufRead};

fn solve(heights: &[i32]) -> i32 {
    let mut counter: Vec<i32> = vec![0; 1000000 + 1];
    for &height in heights {
        counter[height as usize] += 1;
    }
    let mut values: Vec<bool> = vec![false; 100000 + 1];
    let mut n = 0;
    for &count in &counter {
        if count <= 0 {
            continue;
        }
        values[count as usize] = true;
        n += 1;
    }
    if n <= 1 {
        return -1;
    }
    let mut a = 0;
    for (i, &v) in values.iter().enumerate() {
        if v {
            a = i as i32;
            break;
        }
    }
    let mut b = 0;
    for (i, &v) in values.iter().enumerate().rev() {
        if v {
            b = i as i32;
            break;
        }
    }
    (a - b).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let t: u32 = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let heights: Vec<i32> = iter
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let solution = solve(&heights);
        println!("{}", solution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(&[3, 1, 3, 2, 3, 2]), 2);
        assert_eq!(solve(&[1]), -1);
    }
}
