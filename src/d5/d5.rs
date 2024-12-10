use std::{collections::HashMap, fmt::Error};

use crate::{traits::DayActivity, utils::utils::read_input_no_separator};

pub struct Day5<'a> {
    file_path: &'a str,
}
impl<'a> Day5<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Day5 { file_path }
    }
}

fn is_superior(
    &a: &u32,
    &b: &u32,
    priority_hash: &HashMap<u32, u32>,
    invrese_hash: &HashMap<u32, u32>,
) -> bool {
    let superior = priority_hash.get(&a);
    match superior {
        None => {
            //check if inf to before sup
            let inv = invrese_hash.get(&a);
            match inv {
                None => false, //none existant
                Some(&v) => is_superior(&v, &b, priority_hash, invrese_hash),
            }
        }
        Some(v) => {
            if *v != b {
                return is_superior(v, &b, priority_hash, invrese_hash);
            } else {
                true
            }
        }
    }
}

impl<'a> DayActivity for Day5<'a> {
    fn step_1(&self) -> Result<i32, Error> {
        let content = read_input_no_separator(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });

        //fisr we parse the input
        let mut final_result = 0;

        let parsed_file = parse_file(&content);
        let priority_hash = parsed_file.1;
        let list_start = parsed_file.0;
        let inverse_order = parsed_file.2;
        //now we just need to parse the print list one by one and sort them with the hash

        for (i, line) in content.iter().skip(list_start).enumerate() {
            let mut current_list: Vec<u32> = Vec::new();
            let mut current_number = 0;
            let mut current_len = 0;
            let mut is_list_ok = true;
            for c in line.chars() {
                if c == ',' {
                    current_list.push(current_number);
                    //check next if exists
                    current_len += 1;

                    if current_len > 1 {
                        let previous = current_list[current_len - 2];
                        if !is_superior(&current_number, &previous, &priority_hash, &inverse_order)
                        {
                            is_list_ok = false;
                            break;
                        }
                    }
                    current_number = 0;

                    continue;
                }
                current_number *= 10;
                current_number += c.to_digit(10).unwrap();
            }
            //push last
            if is_list_ok {
                current_list.push(current_number);
            }
            println!("{:?} : {:?}", current_list, is_list_ok);
            //now we sort the list
            // current_list.sort_by(|first, next| {
            //     println!("{:?}", first);
            //     if priority_hash.get(first).unwrap().eq(next) {
            //         return std::cmp::Ordering::Less;
            //     } else {
            //         return std::cmp::Ordering::Greater;
            //     }
            // });
        }
        Ok(final_result)
    }
    fn step_2(&self) -> Result<i32, Error> {
        Ok(0)
    }
}

fn parse_file(content: &Vec<String>) -> (usize, HashMap<u32, u32>, HashMap<u32, u32>) {
    let mut print_order: HashMap<u32, u32> = HashMap::new();
    let mut inverse_priority_order: HashMap<u32, u32> = HashMap::new();
    let mut print_order_index = 0;
    for (i, line) in content.iter().enumerate() {
        if line.is_empty() {
            print_order_index = i + 1; // we skip the empty line
            break;
            //no need to go further
        }
        //get all char before |
        let mut first = 0;
        let mut last = 0;
        for (j, c) in line.char_indices() {
            if c == '|' {
                //we can automaticaly determine the end
                last = line[(j + 1)..].parse().unwrap();
                break;
            }
            first *= 10;
            first += c.to_digit(10).unwrap();
        }
        print_order.insert(last, first);
        inverse_priority_order.insert(first, last);
    }
    (print_order_index, print_order, inverse_priority_order)
}
