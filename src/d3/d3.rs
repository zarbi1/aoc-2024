use std::fmt::Error;

use crate::{traits::DayActivity, utils::utils::read_input_no_separator};

pub struct Day3<'a> {
    file_path: &'a str,
}
impl<'a> Day3<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Day3 { file_path }
    }
}

impl<'a> DayActivity for Day3<'a> {
    fn step_1(&self) -> Result<i32, Error> {
        let content = read_input_no_separator(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });
        let mut final_value = 0;
        for line in content {
            let mut start_index = 0;
            while let Some(pos) = line[start_index..].find("mul(") {
                let mul_pos = start_index + pos;
                //now we parse

                let res = parse_mul(&line[mul_pos..]).unwrap_or((0, 4));
                final_value += res.0;
                start_index += res.1 as usize;
            }
        }
        Ok(final_value as i32)
    }
    fn step_2(&self) -> Result<i32, Error> {
        Ok(12)
    }
}

fn parse_mul(text: &str) -> Option<(u32, u32)> {
    //we start at the (
    if text.len() < 5 {
        return None;
    }
    let mut current_left = 0;
    let mut current_right = 0;
    let mut is_left = true;
    let mut len = 0;
    println!("{}", &text[4..]);
    for c in text[4..].chars() {
        if c.is_numeric() {
            let digit = c.to_digit(10)?;
            if is_left {
                current_left *= 10;
                current_left += digit
            } else {
                //currently on right
                current_right *= 10;
                current_left += digit;
            }
        } else if c == ',' {
            is_left = false;
        } else if c == ')' {
            break;
        } else {
            return None; //invalid char found
        }
        len += 1;
    }
    if is_left {
        return None; // we never saw a ','
    }
    return Some((current_left * current_right, len + 4));
}
