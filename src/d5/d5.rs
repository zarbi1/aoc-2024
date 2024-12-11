use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Error,
};

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
    &b_index: &usize,
    priority_hash: &HashSet<(u32, u32)>,
    current_vec: &Vec<u32>,
) -> bool {
    //first we check if a is higher in prio than b
    let prio = priority_hash.get(&(a, current_vec[b_index]));
    match prio {
        None => {
            //we first check reverse, if its the case then the list is in the wrong order
            let last = priority_hash.get(&(current_vec[b_index], a));
            match last {
                None => {
                    //now we check if we are higher than b's prio
                    if b_index == 0 {
                        //no more to check
                        return false;
                    }
                    //call on predecessor
                    return is_superior(&a, &(b_index - 1), priority_hash, current_vec);
                }
                Some(_) => false,
            }
        }
        Some(_) => true, //b is the predecessor of a, all good
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
        let list_start = parsed_file.0;
        let priority_hash = parsed_file.1;
        let lists = get_valid_and_invalid_lists(&content, &priority_hash, list_start);
        for val in lists.0 {
            final_result += val[val.len() / 2];
        }
        Ok(final_result as i32)
    }
    fn step_2(&self) -> Result<i32, Error> {
        let content = read_input_no_separator(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });
        let parsed_file = parse_file(&content);
        let list_start = parsed_file.0;
        let priority_hash = parsed_file.1;

        //fisr we parse the input
        let mut final_result = 0;

        let mut lists = get_valid_and_invalid_lists(&content, &priority_hash, list_start);
        //now we just need to sort the invalid lists
        for list in lists.1.iter_mut() {
            list.sort_by(|a, b| {
                // Check if there's a priority relationship between a and b
                if priority_hash.contains(&(*a, *b)) {
                    Ordering::Greater // a should come after b
                } else if priority_hash.contains(&(*b, *a)) {
                    Ordering::Less // b should come before a
                } else {
                    Ordering::Equal // no defined priority, maintain original order
                }
            });
            //get the value
            final_result += list[list.len() / 2];
        }
        Ok(final_result as i32)
    }
}

fn parse_file(content: &Vec<String>) -> (usize, HashSet<(u32, u32)>) {
    let mut print_order: HashSet<(u32, u32)> = HashSet::new();
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
        print_order.insert((last, first));
    }
    (print_order_index, print_order)
}

fn get_valid_and_invalid_lists(
    content: &Vec<String>,
    priority_hash: &HashSet<(u32, u32)>,
    list_start: usize,
) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    //now we just need to parse the print list one by one and sort them with the hash
    let mut validList = Vec::new();
    let mut invList = Vec::new();
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
                    if is_list_ok {
                        if !is_superior(
                            &current_number,
                            &(current_len - 2),
                            &priority_hash,
                            &current_list,
                        ) {
                            is_list_ok = false;
                        }
                    }
                }
                current_number = 0;

                continue;
            }
            current_number *= 10;
            current_number += c.to_digit(10).unwrap();
        }
        //push last

        current_list.push(current_number);
        //check last
        if is_list_ok {
            current_len += 1;
            if current_len > 1 {
                if !is_superior(
                    &current_number,
                    &(current_len - 2),
                    &priority_hash,
                    &current_list,
                ) {
                    is_list_ok = false;
                }
            }
            if is_list_ok {
                //in case of update
                validList.push(current_list);
                continue;
            }
        }
        invList.push(current_list);
    }
    (validList, invList)
}

//now we sort the list
// current_list.sort_by(|first, next| {
//     println!("{:?}", first);
//     if priority_hash.get(first).unwrap().eq(next) {
//         return std::cmp::Ordering::Less;
//     } else {
//         return std::cmp::Ordering::Greater;
//     }
// });
