use std::env;

use aoc::d1::d1::Day1;
use aoc::d2::d2::Day2;
use aoc::d3::d3::Day3;
use aoc::d4::d4::Day4;
use aoc::d5::d5::Day5;
use aoc::traits::DayActivity;

fn call_activity(day: &dyn DayActivity, day_number: &usize) {
    println!("----------- Day: {:?} -----------", day_number);
    println!(
        "Step 1 result: {:?} | Step2 result: {:?}",
        day.step_1().unwrap(),
        day.step_2().unwrap()
    );
}
fn main() {
    //creating the day objects
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join("inputs");
    let day1_path = file_path.join("day1").to_string_lossy().into_owned();
    let day2_path = file_path.join("day2").to_string_lossy().into_owned();
    let day3_path = file_path.join("day3").to_string_lossy().into_owned();
    let day4_path = file_path.join("day4").to_string_lossy().into_owned();
    let day5_path = file_path.join("day5").to_string_lossy().into_owned();
    let days: Vec<Box<dyn DayActivity>> = vec![
        Box::new(Day1::new(&day1_path)),
        Box::new(Day2::new(&day2_path)),
        Box::new(Day3::new(&day3_path)),
        Box::new(Day4::new(&day4_path)),
        Box::new(Day5::new(&day5_path)),
    ];
    for (i, day) in days.iter().enumerate() {
        call_activity(&**day, &(i + 1));
    }
}
