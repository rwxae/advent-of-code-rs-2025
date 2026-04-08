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

pub fn solution_2(_input: &str) -> i64 {
    todo!()
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
        assert_eq!(solve(6, 1, solution_2), 4174379265);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(solve(6, 2, solution_2), 50857215650);
    }
}
