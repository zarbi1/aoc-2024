use crate::{traits::DayActivity, utils::utils::read_input_no_separator};

pub struct Day7<'a> {
    file_path: &'a str,
}
impl<'a> Day7<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Day7 { file_path }
    }
}

impl<'a> DayActivity for Day7<'a> {
    fn step_1(&self) -> Result<i32, std::fmt::Error> {
        let content = read_input_no_separator(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });

        let mut result = 0;
        for line in content {
            //now parse the expression
            let exp = parse_expression(&line);
            //now we found the potential operand
            match find_expression(&exp.1, exp.0, false) {
                Some(_) => result += exp.0,
                None => {}
            }
        }
        println!("{:?}", result);
        Ok(result as i32)
    }
    fn step_2(&self) -> Result<i32, std::fmt::Error> {
        let content = read_input_no_separator(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });

        let mut result = 0;
        for line in content {
            //now parse the expression
            let exp = parse_expression(&line);
            //now we found the potential operand
            match find_expression(&exp.1, exp.0, true) {
                Some(_) => result += exp.0,
                None => {}
            }
        }
        println!("{:?}", result);
        Ok(result as i32)
    }
}

fn find_expression(numbers: &[i32], target: i128, step2: bool) -> Option<String> {
    fn backtrack(
        numbers: &[i32],
        target: i128,
        current_index: usize,
        current_value: i128,
        current_expr: &str,
        last_op: Option<char>,
        step2: bool,
    ) -> Option<String> {
        // Base case: reached end of numbers
        if current_index == numbers.len() {
            return if current_value == target {
                Some(current_expr.to_string())
            } else {
                None
            };
        }

        let current_num = numbers[current_index] as i128;

        // Try addition
        if let Some(result) = backtrack(
            numbers,
            target,
            current_index + 1,
            match last_op {
                Some('+') => current_value + current_num,
                Some('*') => current_value * current_num,
                Some('|') => format!("{}{}", current_value, current_num).parse().unwrap(),
                _ => current_num,
            },
            &format!(
                "{}{}{}",
                current_expr,
                if current_index > 0 { "+" } else { "" },
                current_num
            ),
            Some('+'),
            step2,
        ) {
            return Some(result);
        }

        // Try multiplication
        if let Some(result) = backtrack(
            numbers,
            target,
            current_index + 1,
            match last_op {
                Some('+') => current_value + (current_num),
                Some('*') => current_value * current_num,
                Some('|') => format!("{}{}", current_value, current_num).parse().unwrap(),
                _ => current_num,
            },
            &format!(
                "{}{}{}",
                current_expr,
                if current_index > 0 { "*" } else { "" },
                current_num
            ),
            Some('*'),
            step2,
        ) {
            return Some(result);
        }
        if step2 {
            // now the | op
            if let Some(result) = backtrack(
                numbers,
                target,
                current_index + 1,
                match last_op {
                    Some('+') => current_value + (current_num),
                    Some('*') => current_value * current_num,
                    Some('|') => format!("{}{}", current_value, current_num).parse().unwrap(),
                    _ => current_num,
                },
                &format!(
                    "{}{}{}",
                    current_expr,
                    if current_index > 0 { "|" } else { "" },
                    current_num
                ),
                Some('|'),
                step2,
            ) {
                return Some(result);
            }
        }

        None
    }

    backtrack(numbers, target, 0, 0, "", None, step2)
}
fn parse_expression(exp: &String) -> (i128, Vec<i32>) {
    //first we detect the final res
    let mut res_i = 0;
    let mut current_res: i128 = 0;
    for (i, c) in exp.char_indices() {
        if c == ':' {
            res_i = i;
            break;
        }
        current_res *= 10;
        current_res += c.to_digit(10).unwrap() as i128;
    }
    //now we parse the rest
    let op = exp[res_i + 1..]
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    return (current_res, op);
}
