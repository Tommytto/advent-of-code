fn main() {
    let result = calculate_assumed_score("./input");
    println!("1 {result}");
    let result = calculate_strategy_score("./input");
    println!("1 {result}");
}

enum FightOutcome {
    Win,
    Lose,
    Draw,
}
fn calculate_strategy_score(file_path: &str) -> u64 {
    let content = read_file(file_path);
    content
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let opponent = match chars.nth(0).unwrap() {
                'A' => Ok(Gesture::Rock),
                'B' => Ok(Gesture::Paper),
                'C' => Ok(Gesture::Scissors),
                _ => Err("bad")
            }.unwrap();
            let needed_fight_outcome = match chars.nth(1).unwrap() {
                'X' => Ok(FightOutcome::Lose),
                'Y' => Ok(FightOutcome::Draw),
                'Z' => Ok(FightOutcome::Win),
                _ => Err("bad")
            }.unwrap();
            let me = match needed_fight_outcome {
                FightOutcome::Win => {
                    match opponent {
                        Gesture::Rock => Gesture::Paper,
                        Gesture::Scissors => Gesture::Rock,
                        Gesture::Paper => Gesture::Scissors,
                    }
                },
                FightOutcome::Draw => {
                    match opponent {
                        Gesture::Rock => Gesture::Rock,
                        Gesture::Scissors => Gesture::Scissors,
                        Gesture::Paper => Gesture::Paper,
                    }
                },
                FightOutcome::Lose => {
                    match opponent {
                        Gesture::Rock => Gesture::Scissors,
                        Gesture::Scissors => Gesture::Paper,
                        Gesture::Paper => Gesture::Rock,
                    }
                },
            };
            calculate_fight_score(FightRound(opponent, me))
        })
        .sum()

}

fn calculate_assumed_score(file_path: &str) -> u64 {
    let content = read_file(file_path);

    content
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let opponent = match chars.nth(0).unwrap() {
                'A' => Ok(Gesture::Rock),
                'B' => Ok(Gesture::Paper),
                'C' => Ok(Gesture::Scissors),
                _ => Err("bad")
            }.unwrap();
            let me = match chars.nth(1).unwrap() {
                'X' => Ok(Gesture::Rock),
                'Y' => Ok(Gesture::Paper),
                'Z' => Ok(Gesture::Scissors),
                _ => Err("bad")
            }.unwrap();
            calculate_fight_score(FightRound(opponent, me))
        })
        .sum::<u64>()
}

enum Gesture {
    Scissors,
    Rock,
    Paper
}

struct FightRound(Gesture, Gesture);

fn calculate_fight_score(record: FightRound) -> u64 {
    let round_outcome_score = match record {
        FightRound(Gesture::Scissors, Gesture::Rock) => 6,
        FightRound(Gesture::Scissors, Gesture::Paper) => 0,
        FightRound(Gesture::Scissors, Gesture::Scissors) => 3,
        FightRound(Gesture::Rock, Gesture::Paper) => 6,
        FightRound(Gesture::Rock, Gesture::Scissors) => 0,
        FightRound(Gesture::Rock, Gesture::Rock) => 3,
        FightRound(Gesture::Paper, Gesture::Scissors) => 6,
        FightRound(Gesture::Paper, Gesture::Rock) => 0,
        FightRound(Gesture::Paper, Gesture::Paper) => 3,
    };

    let selected_shape_score = match record.1 {
        Gesture::Rock => 1,
        Gesture::Paper => 2,
        Gesture::Scissors => 3,
    };

    round_outcome_score + selected_shape_score
}

fn read_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_assume_strategy() {
        assert_eq!(calculate_assumed_score("test-input"), 15)
    }

    #[test]
    fn calculate_fight_score_positive() {
        assert_eq!(calculate_fight_score(FightRound(Gesture::Paper, Gesture::Rock)), 1);
        assert_eq!(calculate_fight_score(FightRound(Gesture::Paper, Gesture::Paper)), 5);
    }
}
