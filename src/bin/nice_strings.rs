// https://www.hackerearth.com/problem/algorithm/monk-and-nice-strings-3-e5800d05/
use std::io::{self, BufRead, Write};

fn solve(strings: &[String], output: &mut impl Write) {
    for (i, string) in strings.iter().enumerate() {
        let count = strings
            .iter()
            .take(i)
            .filter(|&prev_string| prev_string < string)
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
    solve(&strings, &mut io::stdout());
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
