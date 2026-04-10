use std::collections::{HashMap, HashSet};

type Position = (usize, usize);

fn follow_beam(diagram: &[&[u8]], seen: &mut HashSet<Position>, position: Position) {
    for i in position.0..diagram.len() {
        if diagram[i][position.1] as char != '^' {
            continue;
        }
        if seen.contains(&(i, position.1)) {
            return;
        }
        seen.insert((i, position.1));
        follow_beam(diagram, seen, (i, position.1 - 1));
        follow_beam(diagram, seen, (i, position.1 + 1));
        return;
    }
}

pub fn solution_1(input: &str) -> usize {
    let mut seen: HashSet<Position> = HashSet::new();
    let diagram = input
        .trim()
        .split('\n')
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let start = diagram
        .first()
        .and_then(|line| line.iter().enumerate().find(|(_, v)| **v == b'S'))
        .map(|(i, _)| i)
        .unwrap();
    follow_beam(&diagram, &mut seen, (1, start));
    seen.len()
}

fn follow_beam_2(diagram: &[&[u8]], seen: &mut HashMap<Position, u64>, position: Position) -> u64 {
    for i in position.0..diagram.len() {
        if diagram[i][position.1] as char != '^' {
            continue;
        }
        if let Some(v) = seen.get(&(i, position.1)) {
            return *v;
        }
        let left = follow_beam_2(diagram, seen, (i, position.1 - 1));
        let right = follow_beam_2(diagram, seen, (i, position.1 + 1));
        let value = left + right;
        seen.insert((i, position.1), left + right);
        return value;
    }
    1
}

pub fn solution_2(input: &str) -> u64 {
    let mut seen: HashMap<Position, u64> = HashMap::new();
    let diagram = input
        .trim()
        .split('\n')
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let start = diagram
        .first()
        .and_then(|line| line.iter().enumerate().find(|(_, v)| **v == b'S'))
        .map(|(i, _)| i)
        .unwrap();
    follow_beam_2(&diagram, &mut seen, (1, start))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn test_1_easy() {
        assert_eq!(solve(7, 1, solution_1), 21);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solve(7, 2, solution_1), 1640);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solve(7, 1, solution_2), 40);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(7, 2, solution_2), 40999072541589);
    }
}
