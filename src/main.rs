mod day1;
mod day2;

fn main() {
    println!("day1: {:?}", day1::get_elf_and_energy("src/inputs/day1", 3));
    println!("day2: {:?}", day2::calculate_win_score("src/inputs/day2"));
}
