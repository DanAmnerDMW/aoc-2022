use std::fs;

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Unable to read input file");
}

fn pt_1_full_overlaps(zone1_start: i32, zone1_end: i32, zone2_start: i32, zone2_end: i32) -> i32 {
    // Zone 2 within Zone 1
    if zone2_start >= zone1_start && zone2_end <= zone1_end {
        return 1;
    // Zone 1 within Zone 2
    } else if zone1_start >= zone2_start && zone1_end <= zone2_end {
        return 1;
    }

    return 0;
}

fn pt_2_partial_overlaps(
    zone1_start: i32,
    zone1_end: i32,
    zone2_start: i32,
    zone2_end: i32,
) -> i32 {
    // Zone 2 within Zone 1
    if zone2_start >= zone1_start && zone2_start <= zone1_end {
        return 1;
    // Zone 1 within Zone 2
    } else if zone1_start >= zone2_start && zone1_start <= zone2_end {
        return 1;
    }

    return 0;
}

fn main() {
    let file_input = read_input_file();
    let mut full_overlapping_pairs = 0;
    let mut partial_overlapping_pairs: i32 = 0;

    for cleaning_pair in file_input.lines() {
        let split_pair = cleaning_pair.split_once(",");

        let zone_1 = split_pair.unwrap().0;
        let zone_2 = split_pair.unwrap().1;

        let zone1_split = zone_1.split_once("-");
        let zone1_start = zone1_split.unwrap().0.parse::<i32>().unwrap();
        let zone1_end = zone1_split.unwrap().1.parse::<i32>().unwrap();

        let zone2_split = zone_2.split_once("-");
        let zone2_start = zone2_split.unwrap().0.parse::<i32>().unwrap();
        let zone2_end = zone2_split.unwrap().1.parse::<i32>().unwrap();

        full_overlapping_pairs +=
            pt_1_full_overlaps(zone1_start, zone1_end, zone2_start, zone2_end);

        partial_overlapping_pairs +=
            pt_2_partial_overlaps(zone1_start, zone1_end, zone2_start, zone2_end);
    }

    println!("Full Overlapping pairs: {full_overlapping_pairs}");

    println!("Partial Overlapping pairs: {partial_overlapping_pairs}");
}
