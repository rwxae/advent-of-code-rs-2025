pub fn solution_1(input: &str) -> u32 {
    input
        .split_whitespace()
        .map(|line| {
            let (ai, a) = line
                .char_indices()
                .max_by(|&(_, ch), &(_, ch2)| match ch.cmp(&ch2) {
                    std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
                    other => other,
                })
                .expect("Line cannot be empty");
            if ai == line.len() - 1 {
                let b = line[..ai].chars().max().unwrap();
                format!("{b}{a}")
            } else {
                let b = line[(ai + 1)..].chars().max().unwrap();
                format!("{a}{b}")
            }
        })
        .map(|v| v.parse::<u32>().unwrap())
        .sum()
}

pub fn solution_2(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn test_1_easy() {
        assert_eq!(solve(3, 1, solution_1), 357);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solve(3, 2, solution_1), 17229);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solve(3, 1, solution_2), 0);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(3, 2, solution_2), 0);
    }
}
