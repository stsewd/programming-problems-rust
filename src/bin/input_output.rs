// https://www.hackerearth.com/practice/basic-programming/input-output/basics-of-input-output/tutorial/

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines().map(|x| x.unwrap());

    let number: i32 = iter.next().unwrap().parse().unwrap();
    let string: String = iter.next().unwrap();

    println!("{}", number * 2);
    println!("{}", string);
}
