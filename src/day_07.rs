use std::collections::HashSet;

type Position = (usize, usize);

fn follow_beam(diagram: &[&[u8]], seen: &mut HashSet<Position>, position: Position) {
    for i in position.0..diagram.len() {
        match diagram[i][position.1] as char {
            '^' => {
                if seen.contains(&(i, position.1)) {
                    return;
                }
                seen.insert((i, position.1));
                follow_beam(diagram, seen, (i, position.1 - 1));
                follow_beam(diagram, seen, (i, position.1 + 1));
                return;
            }
            _ => (),
        }
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

pub fn solution_2(_input: &str) -> usize {
    todo!()
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
        assert_eq!(solve(7, 2, solution_2), 170520923035051);
    }
}
