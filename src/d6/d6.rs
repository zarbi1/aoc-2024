use crate::traits::DayActivity;

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
        Ok(1)
    }
    fn step_2(&self) -> Result<i32, std::fmt::Error> {
        Ok(1)
    }
}
