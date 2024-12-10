use std::fmt::Error;

use crate::traits::DayActivity;

pub struct Day5<'a> {
    file_path: &'a str,
}
impl<'a> Day5<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Day5 { file_path }
    }
}

impl<'a> DayActivity for Day5<'a> {
    fn step_1(&self) -> Result<i32, Error> {}
    fn step_2(&self) -> Result<i32, Error> {}
}
