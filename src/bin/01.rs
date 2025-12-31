use advent_of_code_2025::read_input;

fn main() {
    let answer_1_easy = solution_1(&read_input("01-easy.txt"));
    let answer_1_hard = solution_1(&read_input("01-hard.txt"));

    println!("Answer 1 (easy): {answer_1_easy}");
    println!("Answer 1 (hard): {answer_1_hard}");
}

fn solution_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| (&line[0..1], line[1..].parse::<u32>().unwrap()))
        .fold((50i32, 0), |acc, rotation| {
            let state = match rotation.0 {
                "L" => ((acc.0 - 100 - (rotation.1 as i32)) % 100 + 100) % 100,
                "R" => (acc.0 + (rotation.1 as i32)) % 100,
                _ => panic!("Unsupported rotation"),
            };
            let count = if state == 0 { acc.1 + 1 } else { acc.1 };
            (state, count)
        })
        .1
}

// fn solution_2(input: &str) -> u32 {
//     42
// }

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
}
