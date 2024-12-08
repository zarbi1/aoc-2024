use std::{collections::HashMap, fmt::Error};

use crate::{traits::DayActivity, utils::utils::read_input};

pub struct Day1<'a> {
    file_path: &'a str,
}
impl<'a> Day1<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Day1 { file_path }
    }
}

impl<'a> DayActivity for Day1<'a> {
    fn step_1(&self) -> Result<i32, Error> {
        let lists =
            compute_two_lists(self.file_path).unwrap_or_else(|error| panic!("Error: {:?}", error));
        //sort the two lists to make up the pairs
        let mut list1 = lists.0;
        let mut list2 = lists.1;
        list1.sort();
        list2.sort();
        //compute the res
        let mut res = 0;
        for (i, _) in list1.iter().enumerate() {
            res += i32::abs(list1[i] - list2[i])
        }
        Ok(res)
    }

    fn step_2(&self) -> Result<i32, Error> {
        //first we get the content of th input
        let mut content = read_input(self.file_path).unwrap_or_else(|error| {
            panic!("Could not extract text: {:?}", error);
        });
        //building right sim score

        let mut sim_score: HashMap<i32, i32> = HashMap::new();
        for line in content.iter_mut() {
            let val2: i32 = line[1].parse().expect("Not a valid number");

            //check if exists
            if sim_score.contains_key(&val2) {
                sim_score.insert(val2, sim_score.get(&val2).unwrap() + 1);
            } else {
                sim_score.insert(val2, 1);
            }
        }
        //now lets iterate again to compute the final score
        let mut result = 0;
        for line in content.iter_mut() {
            let val1: i32 = line[0].parse().expect("Not a valid number");
            //check if it exists in th right list
            if sim_score.contains_key(&val1) {
                result += val1 * sim_score.get(&val1).unwrap();
            }
            //else we do nothing
        }
        Ok(result)
    }
}

fn compute_two_lists(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), Error> {
    //first we get the content of th input
    let mut content = read_input(file_path).unwrap_or_else(|error| {
        panic!("Could not extract text: {:?}", error);
    });
    //building the two lists first
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in content.iter_mut() {
        let val1: i32 = line[0].parse().expect("Not a valid number");
        let val2: i32 = line[1].parse().expect("Not a valid number");
        list1.push(val1);
        list2.push(val2);
    }
    Ok((list1, list2))
}
