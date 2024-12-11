use std::collections::{HashMap, HashSet, VecDeque};

use crate::{traits::DayActivity, utils::utils::read_input_no_separator};

pub struct Day6<'a> {
    file_path: &'a str,
}
impl<'a> Day6<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Day6 { file_path }
    }
}

impl<'a> DayActivity for Day6<'a> {
    fn step_1(&self) -> Result<i32, std::fmt::Error> {
        let mut content = read_input_no_separator(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });
        let mut passage_counter = 0;
        let directions = [
            (-1, 0), //to top
            (0, 1),  // to left
            (1, 0),  // to bottom
            (0, -1), //to right
        ];
        let pos = get_guard_pos(&mut content);
        let mut x: i32 = pos.0 as i32;
        let mut y: i32 = pos.1 as i32;
        let mut current_oriantation = 0;
        loop {
            let x_u = x as usize;
            let y_u = y as usize;
            if x < 0 || x_u >= content.len() || y < 0 || y_u >= content[x_u].len() {
                //exit found
                break;
            }

            let current_char = content[x_u].as_bytes()[y_u] as char;
            if current_char == '#' {
                //go back and change direction
                x = x + (-1 * directions[current_oriantation].0);
                y = y + (-1 * directions[current_oriantation].1);
                current_oriantation = (current_oriantation + 1) % 4;
                x += directions[current_oriantation].0;
                y += directions[current_oriantation].1;
                continue;
            }
            //check if char has been visited
            if current_char == '.' {
                //mark it
                content[x_u].replace_range(y_u..y_u + 1, "X");
                passage_counter += 1;
            }
            //already visted
            x += directions[current_oriantation].0;
            y += directions[current_oriantation].1;
        }
        Ok(passage_counter + 1)
    }
    fn step_2(&self) -> Result<i32, std::fmt::Error> {
        Ok(-1) // I tried various different thing: BFS, brutforce etc but couln't find anything without an infinit loop
    }
}
fn get_guard_pos(content: &Vec<String>) -> (i32, i32) {
    for (i, line) in content.iter().enumerate() {
        for (j, c) in line.char_indices() {
            if c == '^' {
                return (i as i32, j as i32);
            }
        }
    }
    (0, 0)
}
