use aoc::d1::d1::Day1;
use aoc::d2::d2::Day2;
use aoc::d3::d3::Day3;
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
    let days: Vec<Box<dyn DayActivity>> = vec![
        Box::new(Day1::new("../inputs/day1")),
        Box::new(Day2::new("../inputs/day2")),
        Box::new(Day3::new("../inputs/day3")),
    ];
    for (i, day) in days.iter().enumerate() {
        call_activity(&**day, &(i + 1));
    }
}
