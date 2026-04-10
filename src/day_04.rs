const COMBINATIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn solution_1(input: &str) -> u32 {
    let grid: Vec<&str> = input.split_whitespace().collect();
    let mut result = 0;
    for i in 0..grid.len() {
        for (j, ch) in grid[i].char_indices() {
            if ch != '@' {
                continue;
            }
            let mut counter = 0;
            for (i1, j1) in COMBINATIONS {
                let i = ((i as i64) + i1) as usize;
                let j = ((j as i64) + j1) as usize;
                let is_paper = grid
                    .get(i)
                    .and_then(|line| line.chars().nth(j).map(|v| v == '@'))
                    .unwrap_or(false);
                if is_paper {
                    counter += 1;
                }
            }
            if counter < 4 {
                result += 1;
            }
        }
    }
    result
}

pub fn solution_2(input: &str) -> u64 {
    let mut grid: Vec<Vec<bool>> = input
        .split_whitespace()
        .map(|line| line.chars().map(|ch| matches!(ch, '@')).collect())
        .collect();
    let mut result = 0;
    let mut i = 0;
    'main: while i < grid.len() {
        for j in 0..grid[i].len() {
            if !grid[i][j] {
                continue;
            }
            let mut counter = 0;
            for (i1, j1) in COMBINATIONS {
                let i = ((i as i64) + i1) as usize;
                let j = ((j as i64) + j1) as usize;
                let is_paper = *grid.get(i).and_then(|line| line.get(j)).unwrap_or(&false);
                if is_paper {
                    counter += 1;
                }
            }
            if counter < 4 {
                result += 1;
                grid[i][j] = false;
                if i > 0 {
                    i -= 1;
                    continue 'main;
                }
            }
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn test_1_easy() {
        assert_eq!(solve(4, 1, solution_1), 13);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solve(4, 2, solution_1), 1411);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solve(4, 1, solution_2), 43);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(4, 2, solution_2), 8557);
    }
}
