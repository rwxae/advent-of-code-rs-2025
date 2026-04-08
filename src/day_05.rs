pub fn solution_1(input: &str) -> usize {
    let (ranges, ingridients) = input
        .split_once("\n\n")
        .expect("Database should be separated by a blank line");
    let ranges = ranges
        .split_whitespace()
        .map(|range| {
            range
                .split_once('-')
                .map(|(from, to)| from.parse::<u64>().unwrap()..=to.parse::<u64>().unwrap())
                .expect("Each line should contain a valid range")
        })
        .collect::<Vec<_>>();
    ingridients
        .split_whitespace()
        .map(|id| id.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

pub fn solution_2(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn test_1_easy() {
        assert_eq!(solve(5, 1, solution_1), 3);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solve(5, 2, solution_1), 525);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solve(5, 1, solution_2), 42);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(5, 2, solution_2), 42);
    }
}
