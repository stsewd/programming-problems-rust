// An over optimized (x1.5) solution for:
// https://www.hackerearth.com/problem/algorithm/monk-and-nice-strings-3-e5800d05/
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

fn solve(strings: &[String], output: &mut impl Write) {
    let mut sorted: Vec<&str> = strings.iter().map(|x| x.as_str()).collect();
    sorted.sort_unstable();
    let map: HashMap<&str, usize> = sorted
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    let mut numbers = Vec::with_capacity(strings.len());
    for string in strings {
        numbers.push(map.get(string.as_str()).unwrap());
    }
    for (i, number) in numbers.iter().enumerate() {
        let count = numbers
            .iter()
            .take(i)
            .filter(|&prev_number| prev_number < number)
            .count();
        writeln!(output, "{}", count).unwrap();
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
    let mut stdout = io::stdout();
    solve(&strings, &mut stdout);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut output = Vec::new();
        let strings: Vec<String> = ["a", "c", "d", "b"].iter().map(|&x| x.to_owned()).collect();
        solve(&strings, &mut output);
        assert_eq!(&output, b"0\n1\n2\n1\n");
    }
}
