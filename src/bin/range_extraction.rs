// https://www.codewars.com/kata/51ba717bb08c1cd60f00002f/train/rust

fn range_extraction(a: &[i32]) -> String {
    let mut result: Vec<String> = Vec::with_capacity(a.len());
    let mut i = 0;
    while i < a.len() {
        let mut end = i;
        for j in (i + 1)..a.len() {
            if a[end] + 1 == a[j] {
                end = j;
            } else {
                break;
            }
        }
        if end - i >= 2 {
            result.push(format!("{}-{}", a[i], a[end]));
            i = end + 1;
        } else {
            result.push(a[i].to_string());
            i += 1;
        }
    }
    result.join(",")
}

pub fn main() {
    range_extraction(&[1, 2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }
}
