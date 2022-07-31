// https://www.hackerearth.com/problem/algorithm/monk-and-nice-strings-3-e5800d05/
use std::io::{self, BufRead};

fn solve(strings: &[String]) {
    for (i, string) in strings.iter().enumerate() {
        let mut count = 0;
        for prev_string in strings.iter().take(i) {
            if prev_string < string {
                count += 1;
            }
        }
        println!("{}", count);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut strings = Vec::with_capacity(n);
    for _ in 0..n {
        strings.push(iter.next().unwrap());
    }
    solve(&strings);
}
