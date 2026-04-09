struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: &Self) -> i64 {
        (1 + self.x - other.x).abs() * (1 + self.y - other.y)
    }
}

pub fn solution_1(input: &str) -> i64 {
    let points = input
        .trim()
        .split_whitespace()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Point { x, y }
        })
        .collect::<Vec<_>>();

    let pairs = points
        .iter()
        .enumerate()
        .map(|(i, a)| points[i + 1..].iter().map(move |b| (a, b)))
        .flatten()
        .collect::<Vec<_>>();

    pairs.iter().map(|(a, b)| a.area(b)).max().unwrap()
}

pub fn solution_2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn test_1_easy() {
        assert_eq!(solve(9, 1, solution_1), 50);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solve(9, 2, solution_1), 4769758290);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solve(9, 1, solution_2), 40);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(9, 2, solution_2), 40999072541589);
    }
}
