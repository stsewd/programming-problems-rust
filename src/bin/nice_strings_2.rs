// An over optimized (x1.5) solution for:
// https://www.hackerearth.com/problem/algorithm/monk-and-nice-strings-3-e5800d05/
use std::collections::HashMap;
use std::io::{self, BufRead};

fn solve(strings: &[String]) {
    let mut sorted: Vec<&str> = strings.iter().map(|x| x.as_str()).collect();
    sorted.sort_unstable();
    let map: HashMap<String, usize> = sorted
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v.to_owned(), i))
        .collect();
    let mut numbers = Vec::with_capacity(strings.len());
    for string in strings {
        numbers.push(map.get(string.as_str()).unwrap());
    }
    for (i, number) in numbers.iter().enumerate() {
        let mut count = 0;
        for prev_number in numbers.iter().take(i) {
            if prev_number < number {
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
