pub fn sum_invalid(input: &str, mut is_valid: impl FnMut(i64) -> bool) -> i64 {
    input
        .split(",")
        .flat_map(|range| {
            range
                .split_once('-')
                .map(|(from, to)| {
                    (
                        from.trim()
                            .parse::<i64>()
                            .expect("Start of the range should be an integer"),
                        to.trim()
                            .parse::<i64>()
                            .expect("End of the range should be an integer"),
                    )
                })
                .map(|(from, to)| from..=to)
                .expect("Expected a range")
        })
        .filter(|v| !is_valid(*v))
        .sum()
}

pub fn solution_1(input: &str) -> i64 {
    sum_invalid(input, |v| {
        if v < 11 {
            return true;
        }
        let v = v.to_string();
        if v.len() % 2 == 1 {
            return true;
        }
        let mid = v.len() / 2;
        v[..mid] != v[mid..]
    })
}

pub fn solution_2(input: &str) -> i64 {
    sum_invalid(input, |v| {
        if v < 11 {
            return true;
        }
        let v = v.to_string();
        let len = v.len();
        'main: for i in 2..=len {
            if len % i != 0 {
                continue;
            }
            let mut iter = v.as_bytes().chunks_exact(len / i).peekable();
            while let Some(left) = iter.next() {
                let Some(right) = iter.peek() else {
                    continue;
                };
                if left != *right {
                    continue 'main;
                }
            }
            return false;
        }
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn test_1_easy() {
        assert_eq!(solve(2, 1, solution_1), 1227775554);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solve(2, 2, solution_1), 40055209690);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solve(2, 1, solution_2), 4174379265);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(2, 2, solution_2), 50857215650);
    }
}
