use crate::common::{self, Solution};

pub struct Day01 {}

impl Solution for Day01 {
    fn name(&self) -> String {
        "TODO".to_owned()
    }

    fn part_a(&self) -> i32 {
        let numbers = common::load("01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        0
    }

    fn part_b(&self) -> i32 {
        let numbers = common::load("01")
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        0
    }
}
