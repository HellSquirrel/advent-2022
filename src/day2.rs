use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Gestures {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn score(&self) -> usize {
        match self {
            Result::Win => 6,
            Result::Lose => 0,
            Result::Draw => 3,
        }
    }
}

impl Gestures {
    fn beats(&self, other: &Gestures) -> Result {
        if *self == *other {
            return Result::Draw;
        }

        match (self, other) {
            (Gestures::Scissors, Gestures::Paper) => Result::Lose,
            (Gestures::Paper, Gestures::Rock) => Result::Lose,
            (Gestures::Rock, Gestures::Scissors) => Result::Lose,
            _ => Result::Win,
        }
    }

    fn get_gesture<'a>(
        expected_result: &'a Result,
        first_payer_gesture: &'a Gestures,
    ) -> Option<&'a Gestures> {
        [Gestures::Rock, Gestures::Paper, Gestures::Scissors]
            .iter()
            .find(|gesture| first_payer_gesture.beats(&gesture) == *expected_result)
    }

    fn score(&self) -> usize {
        match self {
            Gestures::Rock => 1,
            Gestures::Paper => 2,
            Gestures::Scissors => 3,
        }
    }
}

pub fn calculate_win_score(path: &str) -> usize {
    let decoder_gestures = HashMap::from([
        ("A", Gestures::Rock),
        ("B", Gestures::Paper),
        ("C", Gestures::Scissors),
    ]);

    let decoder_expected_result =
        HashMap::from([("X", Result::Lose), ("Y", Result::Draw), ("Z", Result::Win)]);

    let file = read_to_string(path).expect("Unable to read file");
    file.split("\n").into_iter().fold(0, |acc, s| {
        let split = s.split(" ").collect::<Vec<&str>>();
        if split.len() != 2 {
            return acc;
        };
        let first_gesture = decoder_gestures.get(split[0]).unwrap();
        let result = decoder_expected_result.get(split[1]).unwrap();
        let second_gesture = Gestures::get_gesture(result, first_gesture).unwrap();
        acc + first_gesture.beats(second_gesture).score() + second_gesture.score()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day2() {
        let file = "src/specs/day2";
        let result = calculate_win_score(file);
        assert_eq!(result, 12);
    }
}
