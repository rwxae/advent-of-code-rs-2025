enum Value {
    Add,
    Multiply,
    Number(u64),
}

pub fn solution_1(input: &str) -> u64 {
    let worksheet: Vec<Vec<Value>> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|v| match v {
                    "+" => Value::Add,
                    "*" => Value::Multiply,
                    _ => Value::Number(v.parse().unwrap()),
                })
                .collect()
        })
        .collect();
    let operators = worksheet.last().unwrap();
    let mut values = operators
        .iter()
        .map(|v| match v {
            Value::Add => 0,
            Value::Multiply => 1,
            _ => panic!("Last line must contain only operators"),
        })
        .collect::<Vec<_>>();
    for (col, operator) in operators.iter().enumerate() {
        for row in &worksheet[..worksheet.len() - 1] {
            let Value::Number(value) = row[col] else {
                panic!("Expected a number, but got an operator");
            };
            values[col] = match operator {
                Value::Add => values[col] + value,
                Value::Multiply => values[col] * value,
                _ => unreachable!(),
            };
        }
    }
    values.iter().sum()
}

pub fn solution_2(input: &str) -> u64 {
    let worksheet: Vec<Vec<char>> = input.split('\n').map(|row| row.chars().collect()).collect();
    let mut sum = 0;
    let mut numbers: Vec<u64> = Vec::new();
    for j in (0..worksheet[0].len()).rev() {
        let mut number: Option<u64> = None;
        for i in 0..worksheet.len() {
            let Some(token) = worksheet.get(i).and_then(|col| col.get(j)) else {
                continue;
            };
            match token {
                '0'..='9' => {
                    let digit = token.to_digit(10).unwrap() as u64;
                    let next = number.map_or(digit, |v| v * 10 + digit);
                    number.replace(next);
                }
                '+' | '*' => {
                    if let Some(number) = number.take() {
                        numbers.push(number);
                    }
                    let iter = numbers.drain(..);
                    sum += match token {
                        '+' => iter.sum::<u64>(),
                        '*' => iter.product::<u64>(),
                        _ => unreachable!(),
                    };
                }
                _ => (),
            }
        }
        if let Some(number) = number {
            numbers.push(number);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solve;

    #[test]
    fn test_1_easy() {
        assert_eq!(solve(6, 1, solution_1), 4277556);
    }

    #[test]
    fn test_1_hard() {
        assert_eq!(solve(6, 2, solution_1), 5381996914800);
    }

    #[test]
    fn test_2_easy() {
        assert_eq!(solve(6, 1, solution_2), 3263827);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(6, 2, solution_2), 9627174150897);
    }
}
