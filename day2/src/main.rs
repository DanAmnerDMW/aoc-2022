use std::fs;

enum Action {
    ROCK,
    PAPER,
    SCISSORS,
    UNKNOWN,
}

enum Outcome {
    WIN,
    DRAW,
    LOSE,
    UNKNOWN,
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Unable to read input file");
}

fn map_opponent_move(input: &str) -> Action {
    if input.eq("A") {
        return Action::ROCK;
    } else if input.eq("B") {
        return Action::PAPER;
    } else if input.eq("C") {
        return Action::SCISSORS;
    }

    return Action::UNKNOWN;
}

fn map_my_move(input: &str, action: &Action) -> Action {
    if input.eq("X") {
        match action {
            Action::ROCK => return Action::SCISSORS,
            Action::PAPER => return Action::ROCK,
            Action::SCISSORS => return Action::PAPER,
            Action::UNKNOWN => return Action::UNKNOWN,
        }
    } else if input.eq("Y") {
        match action {
            Action::ROCK => return Action::ROCK,
            Action::PAPER => return Action::PAPER,
            Action::SCISSORS => return Action::SCISSORS,
            Action::UNKNOWN => return Action::UNKNOWN,
        }
    } else if input.eq("Z") {
        match action {
            Action::ROCK => return Action::PAPER,
            Action::PAPER => return Action::SCISSORS,
            Action::SCISSORS => return Action::ROCK,
            Action::UNKNOWN => return Action::UNKNOWN,
        }
    }

    return Action::UNKNOWN;
}

fn determine_result(my_action: &Action, opponent_action: &Action) -> Outcome {
    match my_action {
        Action::ROCK => match opponent_action {
            Action::ROCK => return Outcome::DRAW,
            Action::PAPER => return Outcome::LOSE,
            Action::SCISSORS => return Outcome::WIN,
            Action::UNKNOWN => return Outcome::UNKNOWN,
        },
        Action::PAPER => match opponent_action {
            Action::ROCK => return Outcome::WIN,
            Action::PAPER => return Outcome::DRAW,
            Action::SCISSORS => return Outcome::LOSE,
            Action::UNKNOWN => return Outcome::UNKNOWN,
        },
        Action::SCISSORS => match opponent_action {
            Action::ROCK => return Outcome::LOSE,
            Action::PAPER => return Outcome::WIN,
            Action::SCISSORS => return Outcome::DRAW,
            Action::UNKNOWN => return Outcome::UNKNOWN,
        },
        Action::UNKNOWN => return Outcome::UNKNOWN,
    }
}

fn main() {
    let file_input = read_input_file();

    let mut score = 0;

    for turn in file_input.lines() {
        let moves: Vec<&str> = turn.split_whitespace().collect();

        let opponent_move = moves.get(0).expect("No value at this position");
        let my_move = moves.get(1).expect("No value at this position");

        let opponent_action = map_opponent_move(&opponent_move);
        let my_action = map_my_move(&my_move, &opponent_action);

        match my_action {
            Action::ROCK => score += 1,
            Action::PAPER => score += 2,
            Action::SCISSORS => score += 3,
            Action::UNKNOWN => score += 0,
        }

        let result = determine_result(&my_action, &opponent_action);

        match result {
            Outcome::WIN => score += 6,
            Outcome::DRAW => score += 3,
            Outcome::LOSE => score += 0,
            Outcome::UNKNOWN => score += 0,
        }
    }

    println!("Score: {score}")
}
