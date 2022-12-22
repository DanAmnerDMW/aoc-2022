use std::fs;

fn read_input_file() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn part1() -> u32 {
    let file_input = read_input_file();
    let mut score = 0;

    for rucksack in file_input.lines() {
        let compartment_split_point = rucksack.len() / 2;
        let first_compartment = &rucksack[0..compartment_split_point];
        let second_compartment = &rucksack[compartment_split_point..rucksack.len()];

        for item in first_compartment.chars() {
            if second_compartment.contains(item) {
                score += if item.is_ascii_uppercase() {
                    item as u32 - 38
                } else {
                    item as u32 - 96
                };
                break;
            }
        }
    }

    score
}

fn part2() -> u32 {
    let file_input = read_input_file();
    let mut rucksack_group: [&str; 3] = [""; 3];
    let mut score = 0;

    for (rucksack_num, rucksack) in file_input.lines().enumerate() {
        rucksack_group[rucksack_num % 3] = rucksack;

        // If mod3 == 2 then we have a full group
        if rucksack_num % 3 == 2 {
            for item in rucksack_group[0].chars() {
                if rucksack_group[1].contains(item) && rucksack_group[2].contains(item) {
                    score += if item.is_ascii_uppercase() {
                        item as u32 - 38
                    } else {
                        item as u32 - 96
                    };
                    break;
                }
            }
        }
    }

    score
}

fn main() {
    let part1_result = part1();

    println!("Part 1: {part1_result}");

    let part2_result = part2();

    println!("Part 2: {part2_result}");
}
