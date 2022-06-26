// https://www.hackerearth.com/problem/algorithm/maximum-binary-number-cb9a58c1/

use std::cmp::Ordering;
use std::io::{self, BufRead};

fn get_iter(start: usize, max: usize) -> impl Iterator<Item = usize> {
    (start..max).chain(0..start)
}

fn compare(a: usize, b: usize, binary_number: &[u8]) -> Ordering {
    let iter_a = get_iter(a, binary_number.len());
    let iter_b = get_iter(b, binary_number.len());
    for (ia, ib) in iter_a.zip(iter_b) {
        if binary_number[ia] > binary_number[ib] {
            return Ordering::Greater;
        }
        if binary_number[ia] < binary_number[ib] {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn solve(k: usize, binary_number: &[u8]) -> usize {
    let len = binary_number.len();
    let mut max = 0;
    for i in 1..len {
        if compare(i, max, binary_number) == Ordering::Greater {
            max = i;
        }
    }
    let mut results: Vec<usize> = Vec::new();

    for i in 0..len {
        if compare(i, max, binary_number) == Ordering::Equal {
            results.push(i);
        }
    }

    let first_element = results.remove(0);
    let mut last: usize = first_element;
    let mut total: usize = 0;
    let mut steps = Vec::with_capacity(results.len());
    for &index in results.iter() {
        let r = index - last;
        total += r;
        steps.push(r);
        last = index;
    }

    let last_round = len - last + first_element;
    total += last_round;
    steps.push(last_round);

    let k = k - 1;
    total *= k / steps.len();

    total += steps.iter().take(k % steps.len()).sum::<usize>();
    total + first_element
}

pub fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());

    let t: u32 = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = iter.next().unwrap();
        let mut numbers = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let k = numbers.nth(1).unwrap();
        let binary_number: Vec<u8> = iter
            .next()
            .unwrap()
            .chars()
            .map(|x| if x == '1' { 1 } else { 0 })
            .collect();
        println!("{}", solve(k, &binary_number));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(solve(2, &[1, 0, 1, 0, 1]), 9);
        assert_eq!(solve(2, &[0, 1, 0, 1, 0, 1]), 3);
    }
}
