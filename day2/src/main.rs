use std::fs;

enum Action {
    Rock,
    Paper,
    Scissors,
    Unknown,
}

enum Outcome {
    Win,
    Draw,
    Lose,
    Unknown,
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn map_opponent_move(input: &str) -> Action {
    if input.eq("A") {
        return Action::Rock;
    } else if input.eq("B") {
        return Action::Paper;
    } else if input.eq("C") {
        return Action::Scissors;
    }

    Action::Unknown
}

fn map_my_move(input: &str, action: &Action) -> Action {
    if input.eq("X") {
        match action {
            Action::Rock => return Action::Scissors,
            Action::Paper => return Action::Rock,
            Action::Scissors => return Action::Paper,
            Action::Unknown => return Action::Unknown,
        }
    } else if input.eq("Y") {
        match action {
            Action::Rock => return Action::Rock,
            Action::Paper => return Action::Paper,
            Action::Scissors => return Action::Scissors,
            Action::Unknown => return Action::Unknown,
        }
    } else if input.eq("Z") {
        match action {
            Action::Rock => return Action::Paper,
            Action::Paper => return Action::Scissors,
            Action::Scissors => return Action::Rock,
            Action::Unknown => return Action::Unknown,
        }
    }

    Action::Unknown
}

fn determine_result(my_action: &Action, opponent_action: &Action) -> Outcome {
    match my_action {
        Action::Rock => match opponent_action {
            Action::Rock => Outcome::Draw,
            Action::Paper => Outcome::Lose,
            Action::Scissors => Outcome::Win,
            Action::Unknown => Outcome::Unknown,
        },
        Action::Paper => match opponent_action {
            Action::Rock => Outcome::Win,
            Action::Paper => Outcome::Draw,
            Action::Scissors => Outcome::Lose,
            Action::Unknown => Outcome::Unknown,
        },
        Action::Scissors => match opponent_action {
            Action::Rock => Outcome::Lose,
            Action::Paper => Outcome::Win,
            Action::Scissors => Outcome::Draw,
            Action::Unknown => Outcome::Unknown,
        },
        Action::Unknown => Outcome::Unknown,
    }
}

fn main() {
    let file_input = read_input_file();

    let mut score = 0;

    for turn in file_input.lines() {
        let moves: Vec<&str> = turn.split_whitespace().collect();

        let opponent_move = moves.first().expect("No value at this position");
        let my_move = moves.get(1).expect("No value at this position");

        let opponent_action = map_opponent_move(opponent_move);
        let my_action = map_my_move(my_move, &opponent_action);

        match my_action {
            Action::Rock => score += 1,
            Action::Paper => score += 2,
            Action::Scissors => score += 3,
            Action::Unknown => score += 0,
        }

        let result = determine_result(&my_action, &opponent_action);

        match result {
            Outcome::Win => score += 6,
            Outcome::Draw => score += 3,
            Outcome::Lose => score += 0,
            Outcome::Unknown => score += 0,
        }
    }

    println!("Score: {score}")
}
