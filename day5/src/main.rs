use std::collections::VecDeque;
use std::fs;

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Unable to read input file");
}

fn main() {
    let file_input = read_input_file();

    let split_input: Vec<&str> = file_input.split("\n\n").collect();

    let crates = split_input.get(0).unwrap();
    let instructions = split_input.get(1).unwrap();

    let lines = crates.lines().collect::<Vec<&str>>();
    let num_stacks = (lines.get(0).unwrap().len() / 4) + 1;
    let mut stacks: [VecDeque<char>; 9] = Default::default();

    let mut line_index = 0;
    while line_index < lines.len() - 1 {
        let mut index = 0;
        while index < num_stacks {
            let crate_id = lines[line_index].chars().nth(index * 4 + 1).unwrap();
            if !crate_id.eq(&' ') {
                stacks[index].push_front(crate_id);
            }

            index += 1;
        }
        line_index += 1;
    }

    // for instruction in instructions.lines().collect::<Vec<&str>>() {
    //     let split_instruction = instruction.split_whitespace().collect::<Vec<&str>>();

    //     let num_to_move: usize = split_instruction.get(1).unwrap().parse().unwrap();
    //     let from_stack: usize = split_instruction.get(3).unwrap().parse().unwrap();
    //     let to_stack: usize = split_instruction.get(5).unwrap().parse().unwrap();

    //     for _ in 0..num_to_move {
    //         let crate_to_move = stacks[from_stack - 1].pop_back().unwrap();
    //         stacks[to_stack - 1].push_back(crate_to_move);
    //     }
    // }

    for instruction in instructions.lines().collect::<Vec<&str>>() {
        let split_instruction = instruction.split_whitespace().collect::<Vec<&str>>();

        let num_to_move: usize = split_instruction.get(1).unwrap().parse().unwrap();
        let from_stack: usize = split_instruction.get(3).unwrap().parse().unwrap();
        let to_stack: usize = split_instruction.get(5).unwrap().parse().unwrap();

        let mut moving_crates: VecDeque<char> = VecDeque::new();
        for _ in 0..num_to_move {
            let crate_to_move = stacks[from_stack - 1].pop_back().unwrap();
            moving_crates.push_front(crate_to_move);
        }

        for moving_crate_num in 0..=(num_to_move - 1) {
            stacks[to_stack - 1].push_back(moving_crates[moving_crate_num]);
        }
    }

    for stack in &stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}
