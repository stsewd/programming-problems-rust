// https://www.hackerearth.com/practice/basic-programming/input-output/basics-of-input-output/practice-problems/algorithm/arithmetic-progression-1-81131fa7/

use std::io::{self, BufRead};

fn move_middle(a: i32, b: i32, x: i32) -> i32 {
    let mut middle = ((b - a) / 2) + a;
    let mut steps = 0;

    // If we can't have a perfect middle, force it!
    if (a - b) % 2 != 0 {
        if x > middle {
            middle += 1;
        }
        steps += 1;
    }
    steps += (middle - x).abs();
    return steps;
}

fn solve(a: i32, b: i32, c: i32) -> i32 {
    if a <= c {
        // cases:
        // - a b c
        // - a c b
        // - b a c
        return move_middle(a, c, b);
    } else {
        // cases:
        // - c b a
        // - c a b
        // - b c a
        return move_middle(c, a, b);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let number_tests: i32 = iter.next().unwrap().parse().unwrap();
    for _ in 0..number_tests {
        let line = iter.next().unwrap();
        let mut line_iter = line.split_whitespace().map(|x| x.parse().unwrap());
        let a: i32 = line_iter.next().unwrap();
        let b: i32 = line_iter.next().unwrap();
        let c: i32 = line_iter.next().unwrap();
        println!("{}", solve(a, b, c));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, solve(2, 2, 2));
        assert_eq!(0, solve(-5, 0, 5));
        assert_eq!(7, solve(-5, 7, 6));
        assert_eq!(105, solve(-10, -100, 20));
        assert_eq!(2, solve(1, -1, 1));
        assert_eq!(8, solve(51, 23, 10));
        assert_eq!(2, solve(3, 7, 8));
        assert_eq!(0, solve(3, 6, 9));
        assert_eq!(0, solve(9, 6, 3));
        assert_eq!(1, solve(5, 7, 8));
        assert_eq!(2, solve(5, 5, 8));
        assert_eq!(2, solve(5, 8, 8));
        assert_eq!(4, solve(8, 3, 5));
        assert_eq!(3, solve(5, 7, 3));
        assert_eq!(1, solve(3, 5, 8));
        assert_eq!(1, solve(3, 5, 9));
        assert_eq!(2, solve(-8, -4, -3));
        assert_eq!(2, solve(-3, -4, -8));
        assert_eq!(2, solve(-4, 0, 1));
        assert_eq!(2, solve(-3, 1, 2));
        assert_eq!(2, solve(-2, 2, 3));
        assert_eq!(2, solve(-1, 3, 4));
        assert_eq!(2, solve(0, 4, 5));
        assert_eq!(2, solve(1, 5, 6));
    }
}
