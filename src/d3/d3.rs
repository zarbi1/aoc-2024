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
                start_index += (res.1 as usize) + pos;
            }
        }
        Ok(final_value as i32)
    }
    fn step_2(&self) -> Result<i32, Error> {
        let content = read_input_no_separator(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });
        let mut final_value = 0;
        let mut mul_active = true;

        for line in content {
            let mut start_index = 0;

            while start_index < line.len() {
                match find_first_match(&line[start_index..], &["do()", "don't()", "mul("]) {
                    Some(matched) => {
                        let current_pos = start_index + matched.0;

                        match matched.1 {
                            "don't()" => {
                                mul_active = false;
                                start_index = current_pos + matched.1.len();
                            }
                            "do()" => {
                                mul_active = true;
                                start_index = current_pos + matched.1.len();
                            }
                            "mul(" => {
                                if mul_active {
                                    let mul = parse_mul(&line[current_pos..]).unwrap_or((0, 4));
                                    final_value += mul.0;
                                    start_index = current_pos + mul.1 as usize;
                                } else {
                                    // If mul is not active, just move past "mul("
                                    start_index = current_pos + 4;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => break, // No more matches in this line
                }
            }
        }
        Ok(final_value as i32)
    }
}

fn find_first_match<'a>(text: &'a str, patterns: &'a [&str]) -> Option<(usize, &'a str)> {
    let mut earliest_match: Option<(usize, &str)> = None;

    for (i, _c) in text.char_indices() {
        for &pattern in patterns {
            if text[i..].starts_with(pattern) {
                // If this is the first match, or found earlier than previous matches
                if earliest_match.map_or(true, |m| i < m.0) {
                    earliest_match = Some((i, pattern));
                    break; // Move to next character after finding a match
                }
            }
        }
    }

    earliest_match
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
    for c in text[4..].chars() {
        if c.is_numeric() {
            let digit = c.to_digit(10)?;
            if is_left {
                current_left *= 10;
                current_left += digit
            } else {
                //currently on right
                current_right *= 10;
                current_right += digit;
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
    return Some((current_left * current_right, len + 5));
}
