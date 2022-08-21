// https://www.hackerearth.com/problem/algorithm/monk-and-sorting-algorithm-3aa7826d/

use std::cmp;
use std::io::{self, BufRead, Write};

fn getkey(item: &str, i: usize) -> usize {
    let to = i * 5;
    let from = to - 4;
    if item.len() < from {
        return 0;
    }
    let end = item.len() - from;
    let start = item.len() - cmp::min(item.len(), to);
    item[start..end + 1].parse().unwrap()
}

fn solve(array: &[String], output: &mut impl Write) {
    let mut i = 1;
    let mut array = array.to_vec();
    loop {
        let mut sum = 0;
        array.sort_by_key(|x| {
            let r = getkey(x, i);
            sum += r;
            r
        });
        if sum == 0 {
            break;
        }
        writeln!(output, "{}", array.join(" ")).unwrap();
        i += 1;
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let line = iter.nth(1).unwrap();
    let array: Vec<String> = line.split_whitespace().map(|x| x.to_owned()).collect();
    solve(&array, &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut output = Vec::new();
        let array: Vec<String> = ["213456789", "167890", "123456789"]
            .iter()
            .map(|&x| x.to_owned())
            .collect();
        solve(&array, &mut output);
        assert_eq!(
            &output,
            b"213456789 123456789 167890\n167890 123456789 213456789\n"
        );
    }
}
