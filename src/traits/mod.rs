use std::fmt::Error;

pub trait DayActivity {
    fn step_1(&self) -> Result<i32, Error>;
    fn step_2(&self) -> Result<i32, Error>;
}
