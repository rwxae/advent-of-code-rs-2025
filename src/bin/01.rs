use advent_of_code_2025::read_input;

const START_STATE: i32 = 50;
const START_COUNT: u32 = 0;

fn main() {
    let input_easy = read_input("01-easy.txt");
    let input_hard = read_input("01-hard.txt");

    let answer_1_easy = solution_1(&input_easy);
    let answer_1_hard = solution_1(&input_hard);
    let answer_2_easy = solution_2(&input_easy);
    let answer_2_hard = solution_2(&input_hard);

    println!("Answer 1 (easy): {answer_1_easy}");
    println!("Answer 1 (hard): {answer_1_hard}");
    println!("Answer 2 (easy): {answer_2_easy}");
    println!("Answer 2 (hard): {answer_2_hard}");
}

fn solution_1(input: &str) -> u32 {
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

fn solution_2(input: &str) -> u32 {
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

    #[test]
    fn test_1_easy() {
        assert_eq!(solution_1(&read_input("01-easy.txt")), 3);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solution_1(&read_input("01-hard.txt")), 1180);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solution_2(&read_input("01-easy.txt")), 6);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solution_2(&read_input("01-hard.txt")), 6892);
    }
}
