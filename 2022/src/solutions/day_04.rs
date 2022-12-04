use crate::common::Solution;

pub struct Day04 {}

impl Solution for Day04 {
    fn name(&self) -> String {
        "Camp Cleanup".to_owned()
    }

    fn part_a(&self, input: String) -> String {
        let mut score = 0;
        for line in input.lines() {
            let pairs = line.split(',').collect::<Vec<&str>>();
            let first = pairs[0]
                .split('-')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let second = pairs[1]
                .split('-')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let first_range: Vec<i32> = (first[0]..=first[1]).collect();
            let second_range: Vec<i32> = (second[0]..=second[1]).collect();

            if first_range.iter().all(|f| second_range.contains(f))
                || second_range.iter().all(|f| first_range.contains(f))
            {
                score += 1;
            }
        }
        score.to_string()
    }

    fn part_b(&self, input: String) -> String {
        let mut score = 0;
        for line in input.lines() {
            let pairs = line.split(',').collect::<Vec<&str>>();
            let first = pairs[0]
                .split('-')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let second = pairs[1]
                .split('-')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let first_range: Vec<i32> = (first[0]..=first[1]).collect();
            let second_range: Vec<i32> = (second[0]..=second[1]).collect();

            if first_range.iter().any(|f| second_range.contains(f))
                || second_range.iter().any(|f| first_range.contains(f))
            {
                score += 1;
            }
        }
        score.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{common, solutions};

    #[test]
    fn part_a() {
        let day = 4;
        let result =
            solutions::ALL[day - 1].part_a(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "2");
    }

    #[test]
    fn part_b() {
        let day = 4;
        let result =
            solutions::ALL[day - 1].part_b(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "4");
    }
}
