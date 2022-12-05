use std::collections::VecDeque;

use crate::common::Solution;

pub struct Day05 {}

fn string_to_instructions(input: &str) -> Vec<usize> {
    let s = input.replace("move", "-").replace("from", "-").replace("to", "-").replace(" ", "");
    s.split("-").collect::<Vec<&str>>().iter().filter(|c| !c.is_empty()).map(|c| c.parse::<usize>().unwrap()).collect()
}

impl Solution for Day05 {
    fn name(&self) -> String {
        "Supply Stacks".to_owned()
    }

    fn part_a(&self, input: String) -> String {
        let mut stack = [
            VecDeque::from(["N", "D", "M", "Q", "B", "P", "Z"]),
            VecDeque::from(["C", "L", "Z", "Q", "M", "D", "H", "V"]),
            VecDeque::from(["Q", "H", "R", "D", "V", "F", "Z", "G"]),
            VecDeque::from(["H", "G", "D", "F", "N"]),
            VecDeque::from(["N", "F", "Q"]),
            VecDeque::from(["D", "Q", "V", "Z", "F", "B", "T"]),
            VecDeque::from(["Q", "M", "T", "Z", "D", "V", "S", "H"]),
            VecDeque::from(["M", "G", "F", "P", "N", "Q"]),
            VecDeque::from(["B", "W", "R", "M"])];
        
        for line in input.lines() {
            let instructions = string_to_instructions(line);

            let amount = instructions[0];
            let from = instructions[1];
            let to = instructions[2];

            for _ in 0..amount {
                let pop = stack[from - 1].pop_back().expect("Something");
                stack[to - 1].push_back(pop);
            }
        }
        stack.map(|s| *s.back().unwrap()).into_iter().collect()
    }

    fn part_b(&self, input: String) -> String {
        let mut stack = [
            VecDeque::from(["N", "D", "M", "Q", "B", "P", "Z"]),
            VecDeque::from(["C", "L", "Z", "Q", "M", "D", "H", "V"]),
            VecDeque::from(["Q", "H", "R", "D", "V", "F", "Z", "G"]),
            VecDeque::from(["H", "G", "D", "F", "N"]),
            VecDeque::from(["N", "F", "Q"]),
            VecDeque::from(["D", "Q", "V", "Z", "F", "B", "T"]),
            VecDeque::from(["Q", "M", "T", "Z", "D", "V", "S", "H"]),
            VecDeque::from(["M", "G", "F", "P", "N", "Q"]),
            VecDeque::from(["B", "W", "R", "M"])];
        
        for line in input.lines() {
            let instructions = string_to_instructions(line);

            let amount = instructions[0];
            let from = instructions[1];
            let to = instructions[2];

            let mut temp: VecDeque<&str> = VecDeque::new();
            for _ in 0..amount {
                let pop = stack[from - 1].pop_back().expect("Something");
                temp.push_front(pop);

            }
            for item in temp {
                stack[to - 1].push_back(item);
            }
        }
        stack.map(|s| *s.back().unwrap()).into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{common, solutions};

    #[test]
    fn part_a() {
        let day = 5;
        let result =
            solutions::ALL[day - 1].part_a(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part_b() {
        let day = 5;
        let result =
            solutions::ALL[day - 1].part_b(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "4");
    }
}
