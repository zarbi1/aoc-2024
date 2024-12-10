use std::collections::HashSet;

use crate::{traits::DayActivity, utils::utils::read_input_no_separator};

pub struct Day4<'a> {
    file_path: &'a str,
}
impl<'a> Day4<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Day4 { file_path }
    }
}

impl<'a> DayActivity for Day4<'a> {
    fn step_1(&self) -> Result<i32, std::fmt::Error> {
        let content = read_input_no_separator(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });
        let rows = content.len();
        let cols = content[0].len();
        let target = "XMAS";
        let mut results = HashSet::new();

        // Directions: horizontal, vertical, diagonal, and their reverse
        let directions = [
            (0, 1),   // right
            (1, 0),   // down
            (1, 1),   // diagonal down-right
            (1, -1),  // diagonal down-left
            (0, -1),  // left
            (-1, 0),  // up
            (-1, -1), // diagonal up-left
            (-1, 1),  // diagonal up-right
        ];

        for start_row in 0..rows {
            for start_col in 0..cols {
                for &(dx, dy) in &directions {
                    let mut current_word = String::new();
                    let mut r = start_row as i32;
                    let mut c = start_col as i32;
                    let mut is_match = true;

                    //building word
                    for _ in 0..4 {
                        // Check if current position
                        if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
                            current_word.push(content[r as usize].as_bytes()[c as usize] as char);
                        } else {
                            is_match = false;
                            break;
                        }

                        // Move to next position
                        r += dx;
                        c += dy;
                    }

                    // Check forward and backward
                    if is_match && current_word == target {
                        results.insert((start_row, start_col, dx, dy));
                    }
                }
            }
        }

        Ok(results.len() as i32)
    }
    fn step_2(&self) -> Result<i32, std::fmt::Error> {
        Ok(-1) // I tried a lot of things but my brain said stop
    }
}
