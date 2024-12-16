use std::{collections::VecDeque, ops::MulAssign};

use crate::{traits::DayActivity, utils::utils::read_input_no_separator};

pub struct Day9<'a> {
    file_path: &'a str,
}
impl<'a> Day9<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Day9 { file_path }
    }
}

impl<'a> DayActivity for Day9<'a> {
    fn step_1(&self) -> Result<i32, std::fmt::Error> {
        // I know that there were a more optimal way of doing this without building the String by determining wether we were on a free space or not by switching loops instead of detecting '.''s
        // But this implem is yes less optimal but more readable for me and it's already 1am and I still got 3 ex to catch up
        let content = read_input_no_separator(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });
        println!("{:?}", content.len());
        let line = build_line(&content[0]);
        let mut queue = build_queue(&line);
        let mut i = 0;
        //now we dequeue
        let mut result: i128 = 0;

        while let Some(current) = queue.pop_front() {
            result += (current * i);
            i += 1;
        }
        println!("{:?}", result);
        Ok(result as i32)
    }

    fn step_2(&self) -> Result<i32, std::fmt::Error> {
        Ok(-1)
    }
}

fn build_line(input: &String) -> String {
    let mut f_string = String::new();
    let mut i = 0;
    let mut current_id = 0;

    while i < input.len() {
        let current = input.as_bytes()[i] as char;

        // File blocks
        for _ in 0..current.to_digit(10).unwrap() {
            f_string.push_str(&current_id.to_string());
        }
        current_id += 1;
        i += 1;

        if i >= input.len() {
            break;
        }

        let free_space = input.as_bytes()[i] as char;
        // Free space blocks
        for _ in 0..free_space.to_digit(10).unwrap() {
            f_string.push('.');
        }
        i += 1;
    }

    f_string
}

fn build_queue(line: &String) -> VecDeque<i128> {
    let mut queue: VecDeque<i128> = VecDeque::new();
    let mut start = 0;
    let mut end = line.len() - 1;

    while start <= end {
        let current_start = line.as_bytes()[start] as char;

        if current_start != '.' {
            // Push the digit from the start
            queue.push_back(current_start.to_digit(10).unwrap() as i128);
        } else {
            // Find the nearest digit from the end
            while end >= start && line.as_bytes()[end] as char == '.' {
                end -= 1;
            }
            if end < start {
                break;
            }
            let current_end = line.as_bytes()[end] as char;
            queue.push_back(current_end.to_digit(10).unwrap() as i128);
            end -= 1;
        }

        start += 1;
    }

    queue
}
