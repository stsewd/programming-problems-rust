// https://www.hackerearth.com/problem/algorithm/the-unlucky-13-d7aea1ff/
use std::collections::HashMap;
use std::io::{self, BufRead};

const MODULE: isize = 1000000009;

fn solve(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&value) = cache.get(&n) {
        return value;
    }

    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 10;
    }

    let a = n / 2;
    let solution = if n % 2 == 0 {
        let b = solve(a, cache) as isize;
        let c = solve(a - 1, cache) as isize;
        ((b * b) % MODULE - (c * c) % MODULE).rem_euclid(MODULE) as usize
    } else {
        let b = solve(a, cache) as isize;
        let c = solve(a + 1, cache) as isize;
        let d = solve(a - 1, cache) as isize;
        ((b * (c - d).rem_euclid(MODULE)) % MODULE) as usize
    };
    cache.insert(n, solution);
    solution
}

pub fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let t: usize = iter.next().unwrap().parse().unwrap();
    let mut cache = HashMap::new();
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        println!("{}", solve(n, &mut cache));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let mut cache = HashMap::new();
        assert_eq!(solve(1, &mut cache), 10);
        assert_eq!(solve(2, &mut cache), 99);
        assert_eq!(solve(3, &mut cache), 980);
        assert_eq!(solve(4, &mut cache), 9701);
        assert_eq!(solve(5, &mut cache), 96030);
        assert_eq!(solve(6, &mut cache), 950599);
        assert_eq!(solve(7, &mut cache), 9409960);
        assert_eq!(solve(8, &mut cache), 93149001);
        assert_eq!(solve(9, &mut cache), 922080050);
        assert_eq!(solve(279865406, &mut cache), 804072729);
        assert_eq!(solve(983179266, &mut cache), 993214316);
    }
}
