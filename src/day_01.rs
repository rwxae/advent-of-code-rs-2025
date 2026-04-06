const START_STATE: i32 = 50;
const START_COUNT: u32 = 0;

pub fn solution_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line[1..].parse::<u32>().unwrap(),
            )
        })
        .fold((START_COUNT, START_STATE), |acc, rotation| {
            let state = match rotation.0 {
                'L' => ((acc.1 - 100 - (rotation.1 as i32)) % 100 + 100) % 100,
                'R' => (acc.1 + (rotation.1 as i32)) % 100,
                _ => panic!("Unsupported rotation"),
            };
            let count = if state == 0 { acc.0 + 1 } else { acc.0 };
            (count, state)
        })
        .0
}

pub fn solution_2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line[1..].parse::<u32>().unwrap(),
            )
        })
        .fold((START_COUNT, START_STATE), |acc, rotation| {
            match rotation.0 {
                'L' => (
                    (acc.0 as i32 - ((((acc.1 - 100) % 100) - (rotation.1 as i32)) / 100)) as u32,
                    (((acc.1 - 100 - (rotation.1 as i32)) % 100) + 100) % 100,
                ),
                'R' => (
                    acc.0 + ((acc.1 + rotation.1 as i32) / 100) as u32,
                    (acc.1 + (rotation.1 as i32)) % 100,
                ),
                _ => panic!("Unsupported rotation"),
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn test_1_easy() {
        assert_eq!(solve(1, 1, solution_1), 3);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solve(1, 2, solution_1), 1180);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solve(1, 1, solution_2), 6);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(1, 2, solution_2), 6892);
    }
}
