use std::io::{self, BufRead};

fn solve(a: i32, b: i32, c: i32) -> i32 {
    return (a..=b).filter(|n| n % c == 0).count() as i32;
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let line = iter.next().unwrap();
    let mut numbers = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let a = numbers.next().unwrap();
    let b = numbers.next().unwrap();
    let c = numbers.next().unwrap();
    println!("{}", solve(a, b, c));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, solve(1, 10, 1));
    }
}
