// https://www.hackerearth.com/practice/basic-programming/input-output/basics-of-input-output/practice-problems/algorithm/roy-and-profile-picture/

use std::io::{self, BufRead};

fn solve(l: u32, w: u32, h: u32) -> String {
    if w < l || h < l {
        "UPLOAD ANOTHER".to_owned()
    } else if w == h {
        "ACCEPTED".to_owned()
    } else {
        "CROP IT".to_owned()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());
    let l: u32 = iter.next().unwrap().parse().unwrap();
    let n: u32 = iter.next().unwrap().parse().unwrap();
    for _ in 0..n {
        let line = iter.next().unwrap();
        let mut line = line.split_whitespace().map(|x| x.parse().unwrap());
        let w: u32 = line.next().unwrap();
        let h: u32 = line.next().unwrap();
        println!("{}", solve(l, w, h));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve(180, 640, 480), String::from("CROP IT"));
        assert_eq!(solve(180, 120, 300), String::from("UPLOAD ANOTHER"));
        assert_eq!(solve(180, 180, 180), String::from("ACCEPTED"));
    }
}
