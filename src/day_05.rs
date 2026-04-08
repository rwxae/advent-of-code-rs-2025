pub fn solution_1(input: &str) -> usize {
    let (ranges, ingridients) = input
        .split_once("\n\n")
        .expect("Database should be separated by a blank line");
    let ranges = ranges
        .split_whitespace()
        .map(|range| {
            range
                .split_once('-')
                .map(|(from, to)| (from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap()))
                .expect("Each line should contain a valid range")
        })
        .collect::<Vec<_>>();
    ingridients
        .split_whitespace()
        .map(|id| id.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|(from, to)| id >= from && id <= to))
        .count()
}

fn concat_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut ranges = ranges.iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>();
    ranges.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut result = Vec::new();
    let mut current = ranges[0];
    for range in &ranges[1..] {
        if current.1 >= range.0 {
            current.1 = current.1.max(range.1);
        } else {
            result.push(current);
            current = *range;
        }
    }
    result.push(current);
    result
}

pub fn solution_2(input: &str) -> u64 {
    let ranges = input
        .find("\n\n")
        .map(|end| &input[..end])
        .expect("Database should be separated by a blank line")
        .split_whitespace()
        .map(|range| {
            range
                .split_once('-')
                .map(|(from, to)| (from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap()))
                .expect("Each line should contain a valid range")
        })
        .collect::<Vec<_>>();
    concat_ranges(&ranges)
        .iter()
        .map(|(from, to)| to - from + 1)
        .sum()
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
        assert_eq!(solve(5, 1, solution_2), 14);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(5, 2, solution_2), 333892124923577);
    }
}
