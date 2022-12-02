use crate::common::Solution;
use crate::solutions::day_02::Choices::*;
use crate::solutions::day_02::Result::*;

pub struct Day02 {}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Choices {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Result {
    Lose,
    Draw,
    Win,
}

fn did_player_win(p1: crate::solutions::day_02::Choices, p2: Choices) -> bool {
    (p1 == Rock && p2 == Scissors) || (p1 == Scissors && p2 == Paper) || (p1 == Paper && p2 == Rock)
}

fn to_choice(choice: &str) -> crate::solutions::day_02::Choices {
    if choice == "A" || choice == "X" {
        Rock
    } else if choice == "B" || choice == "Y" {
        Paper
    } else {
        Scissors
    }
}

fn to_result(choice: &str) -> Result {
    if choice == "X" {
        Lose
    } else if choice == "Y" {
        Draw
    } else {
        Win
    }
}

impl Solution for Day02 {
    fn name(&self) -> String {
        "Rock Paper Scissors".to_owned()
    }

    fn part_a(&self, input: String) -> String {
        let mut score = 0;
        for line in input.lines() {
            let split = line.split(' ').collect::<Vec<&str>>();
            let opponent = to_choice(split[0]);
            let player = to_choice(split[1]);

            score += (player as i32) + 1;

            if player == opponent {
                score += 3;
            } else if did_player_win(player, opponent) {
                score += 6;
            } else {
                score += 0;
            }
        }
        score.to_string()
    }

    fn part_b(&self, input: String) -> String {
        let mut score = 0;
        for line in input.lines() {
            let split = line.split(' ').collect::<Vec<&str>>();
            let opponent = to_choice(split[0]);
            let result = to_result(split[1]);
            let player;
            if result == Draw {
                player = opponent;
            } else if result == Win {
                if opponent == Rock {
                    player = Paper;
                } else if opponent == Paper {
                    player = Scissors;
                } else {
                    player = Rock;
                }
            } else if opponent == Rock {
                player = Scissors
            } else if opponent == Paper {
                player = Rock;
            } else {
                player = Paper
            }

            score += (player as i32) + 1;

            if player == opponent {
                score += 3;
            } else if did_player_win(player, opponent) {
                score += 6;
            } else {
                score += 0;
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
        let day = 2;
        let result =
            solutions::ALL[day - 1].part_a(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "15");
    }

    #[test]
    fn part_b() {
        let day = 2;
        let result =
            solutions::ALL[day - 1].part_b(common::load_example(format!("{:02}", day).as_str()));
        assert_eq!(result, "12");
    }
}
