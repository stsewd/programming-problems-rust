// https://www.hackerearth.com/practice/basic-programming/input-output/basics-of-input-output/practice-problems/algorithm/mojtaba-prepares-contest-29b2a044/

use std::cmp;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let tests: i32 = iter.next().unwrap().parse().unwrap();
    for _ in 0..tests {
        let line = iter.next().unwrap();
        let mut numbers = line.split_whitespace().map(|x| x.parse().unwrap());
        let green: i32 = numbers.next().unwrap();
        let purple: i32 = numbers.next().unwrap();

        let n_users: i32 = iter.next().unwrap().parse().unwrap();
        let mut a = 0;
        let mut b = 0;
        for __ in 0..n_users {
            let line = iter.next().unwrap();
            let mut points = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
            a += points.next().unwrap();
            b += points.next().unwrap();
        }
        let result =
            cmp::max(green, purple) * cmp::min(a, b) + cmp::min(green, purple) * cmp::max(a, b);
        println!("{}", result);
    }
}
