mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("day1: {:?}", day1::get_elf_and_energy("src/inputs/day1", 3));
    println!("day2: {:?}", day2::calculate_win_score("src/inputs/day2"));
    println!(
        "day3 part1: {:?}",
        day3::calculate_priorities_part1("src/inputs/day3")
    );
    println!(
        "day3 part2: {:?}",
        day3::calculate_priorities_part2("src/inputs/day3")
    );

    println!(
        "day4 part1: {:?}",
        day4::get_intersect_ranges("src/inputs/day4", day4::fully_overlaps)
    );

    println!(
        "day4 part2: {:?}",
        day4::get_intersect_ranges("src/inputs/day4", day4::partially_overlaps)
    );
}
