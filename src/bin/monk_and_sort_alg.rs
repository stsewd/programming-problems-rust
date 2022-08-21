// https://www.hackerearth.com/problem/algorithm/monk-and-sorting-algorithm-3aa7826d/

use std::io::{self, BufRead, Write};

fn getkey(n: usize, i: usize) -> usize {
    let base: usize = 10;
    let n = n / base.pow(((i - 1) * 5) as u32);
    n % 100000
}

fn solve(array: &mut [usize], output: &mut impl Write) {
    let mut result = vec![0; array.len()];
    let max = array.iter().max().unwrap().to_string().len();
    let (a, b) = (max / 5, max % 5);
    let n = a + (if b != 0 { 1 } else { 0 });
    for i in 1..=n {
        let mut counter: [usize; 100000] = [0; 100000];
        // Assign.
        for &element in array.iter() {
            let k = getkey(element, i);
            counter[k] += 1;
        }
        // Calculate positions.
        for j in 1..counter.len() {
            counter[j] += counter[j - 1];
        }
        // Put elements in position respecting the current order.
        for j in (0..array.len()).rev() {
            let k = getkey(array[j], i);
            result[counter[k] - 1] = array[j];
            counter[k] -= 1;
        }
        // Print answer.
        for element in &result {
            write!(output, "{} ", element).unwrap();
        }
        writeln!(output).unwrap();
        // Swap array.
        for (j, &item) in result.iter().enumerate() {
            array[j] = item;
        }
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let line = iter.nth(1).unwrap();
    let mut array: Vec<usize> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    solve(&mut array, &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut output = Vec::new();
        let mut array: Vec<usize> = vec![213456789, 167890, 123456789];
        solve(&mut array, &mut output);
        assert_eq!(
            &output,
            b"213456789 123456789 167890 \n167890 123456789 213456789 \n"
        );
    }
}
