use std::fmt::Error;

use crate::utils::utils::read_input;
enum Direction {
    NotSet,
    Up,
    Down,
}
pub fn get_nb_safe(file_path: &str) -> Result<i32, Error> {
    //first we get the values

    let content = read_input(file_path).unwrap_or_else(|error| {
        panic!("Could not extract text: {:?}", error);
    });

    //now we compute the number of safe reports
    let mut safe_reports = 0;
    for report in content {
        if is_safe(&report) {
            safe_reports += 1;
            continue;
        }
    }
    Ok(safe_reports)
}

pub fn get_nb_safe_2(file_path: &str) -> Result<i32, Error> {
    let content = read_input(file_path).unwrap_or_else(|error| {
        panic!("Could not extract text: {:?}", error);
    });

    let mut safe_reports = 0;
    for report in &content {
        if is_safe(report) {
            safe_reports += 1;
            continue;
        }

        // Try removing each level
        for remove_idx in 0..report.len() {
            if is_safe_without_level(report, remove_idx) {
                safe_reports += 1;
                break;
            }
        }
    }
    Ok(safe_reports)
}

fn is_safe(report: &Vec<String>) -> bool {
    let mut direction = Direction::NotSet;

    for window in report.windows(2) {
        let val1: i32 = window[0].parse().unwrap();
        let val2: i32 = window[1].parse().unwrap();
        let diff = (val1 - val2).abs();

        if diff > 3 || diff == 0 {
            return false;
        }

        match direction {
            Direction::NotSet => {
                direction = if val1 < val2 {
                    Direction::Up
                } else {
                    Direction::Down
                };
            }
            Direction::Up => {
                if val1 > val2 {
                    return false;
                }
            }
            Direction::Down => {
                if val1 < val2 {
                    return false;
                }
            }
        }
    }

    true
}

fn is_safe_without_level(report: &Vec<String>, remove_idx: usize) -> bool {
    //we clone the reports
    let mut modified_report = report.clone();
    modified_report.remove(remove_idx);

    let mut direction = Direction::NotSet;

    for window in modified_report.windows(2) {
        let val1: i32 = window[0].parse().unwrap();
        let val2: i32 = window[1].parse().unwrap();
        let diff = (val1 - val2).abs();

        if diff > 3 || diff == 0 {
            return false;
        }

        match direction {
            Direction::NotSet => {
                direction = if val1 < val2 {
                    Direction::Up
                } else {
                    Direction::Down
                };
            }
            Direction::Up => {
                if val1 > val2 {
                    return false;
                }
            }
            Direction::Down => {
                if val1 < val2 {
                    return false;
                }
            }
        }
    }

    true
}
