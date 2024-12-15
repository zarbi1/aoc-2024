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
        let line = build_line(&content[0]);
        println!("{:?}", &line);
        let mut queue = build_queue(&line);
        let mut i = 0;
        //now we dequeue
        let mut result = 0;

        println!("{:?}", queue);
        while let Some(current) = queue.pop_front() {
            result += current * i;
            i += 1;
        }
        Ok(result)
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
        let mut current = input.as_bytes()[i] as char;
        for _ in 0..current.to_digit(10).unwrap() {
            f_string.push_str(&current_id.to_string());
        }
        current_id += 1;
        i += 1;
        println!("{:?}", i);
        if i >= input.len() {
            //last one check
            break;
        }
        current = input.as_bytes()[i] as char;
        //now the free space
        for _ in 0..current.to_digit(10).unwrap() {
            f_string.push('.');
        }
        i += 1;
    }

    f_string
}

fn build_queue(line: &String) -> VecDeque<i32> {
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut start = 0;
    let mut end = line.len() - 1;
    while start <= end {
        let current_el = line.as_bytes()[start] as char;
        if current_el != '.' {
            //is digit
            queue.push_back(current_el.to_digit(10).unwrap() as i32);
        } else {
            //we are on a free space
            while line.as_bytes()[end] as char == '.' && end >= start {
                end -= 1;
            }
            //we are on a digit, enque it
            queue.push_back((line.as_bytes()[end] as char).to_digit(10).unwrap() as i32);
            end -= 1;
        }
        //in all cases we need to move forward
        start += 1;
    }
    queue
}
