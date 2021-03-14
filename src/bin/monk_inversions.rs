// https://www.hackerearth.com/practice/codemonk/

use std::io::{self, BufRead};

fn solve(array: &[Vec<i32>], n: u32) -> u32 {
    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            let current = array.get(i as usize).unwrap().get(j as usize).unwrap();
            for x in i..n {
                for y in j..n {
                    let compare = array.get(x as usize).unwrap().get(y as usize).unwrap();
                    if current > compare {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let t = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: u32 = iter.next().unwrap().parse().unwrap();
        let mut array: Vec<Vec<i32>> = Vec::new();
        for __ in 0..n {
            let row: Vec<i32> = iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            array.push(row);
        }
        println!("{}", solve(&array, n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, solve(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 3));
        assert_eq!(2, solve(&[vec![4, 3], vec![1, 4]], 2));
    }
}
