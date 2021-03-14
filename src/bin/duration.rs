// https://www.hackerearth.com/practice/basic-programming/input-output/basics-of-input-output/practice-problems/algorithm/duration/

use std::io::{self, BufRead};

fn solve(sh: i32, sm: i32, eh: i32, em: i32) -> (i32, i32) {
    let mut hours: i32 = eh - sh;
    let mut minutes: i32 = em - sm;

    if minutes < 0 {
        hours -= 1;
        minutes += 60;
    }
    (hours, minutes)
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let n: u32 = iter.next().unwrap().parse().unwrap();
    for _ in 0..n {
        let line = iter.next().unwrap();
        let line = line.split_whitespace();
        let array: Vec<i32> = line.map(|x| x.parse().unwrap()).collect();
        let (sh, sm, eh, em) = (array[0], array[1], array[2], array[3]);
        let solution = solve(sh, sm, eh, em);
        println!("{} {}", solution.0, solution.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(1, 44, 2, 14), (0, 30));
        assert_eq!(solve(2, 42, 8, 23), (5, 41));
    }
}
