use std::fs;

struct Top3 {
    first: i32,
    second: i32,
    third: i32,
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Unable to read input file");

    let elf_calories: Vec<&str> = file_contents.split("\n\n").collect();

    let mut top3 = Top3 {
        first: 0,
        second: 0,
        third: 0,
    };

    for elf in elf_calories {
        let mut total_calories: i32 = 0;
        for calories in elf.split_whitespace().collect::<Vec<&str>>() {
            total_calories += calories.parse::<i32>().expect("Unable to cast to int");
        }
        if total_calories > top3.first {
            top3.third = top3.second;
            top3.second = top3.first;
            top3.first = total_calories;
        } else if total_calories > top3.second {
            top3.third = top3.second;
            top3.second = total_calories;
        } else if total_calories > top3.third {
            top3.third = total_calories;
        }
    }

    let top3_total = top3.first + top3.second + top3.third;
    println!("{top3_total}");
}
