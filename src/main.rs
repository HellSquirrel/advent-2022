mod day1;
mod day10;
mod day11;
mod day11_2;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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

    println!(
        "day5 part1: {:?}",
        day5::parse_input("src/inputs/day5", true)
    );
    println!(
        "day5 part2: {:?}",
        day5::parse_input("src/inputs/day5", false)
    );

    println!("day6 part1: {:?}", day6::get_marker("src/inputs/day6", 4));
    println!("day6 part2: {:?}", day6::get_marker("src/inputs/day6", 14));
    // println!(
    //     "day7 part1: {:?}",
    //     day7::get_smallest_sum("src/inputs/day7")
    // );

    // println!("day7 part1: {:?}", day7::get_biggest_sum("src/inputs/day7"));
    println!(
        "day8 part1: {:?}",
        day8::count_edge_trees("src/inputs/day8")
    );

    println!(
        "day8 part2: {:?}",
        day8::get_scenic_score("src/inputs/day8")
    );

    println!(
        "day10 part1: {:?}",
        day10::parse_input("src/inputs/day10").0
    );
    println!("day10 part2:");

    let parsed = day10::parse_input("src/inputs/day10").1;
    let mut result = parsed.split("").collect::<Vec<&str>>();
    result.pop();

    print!(
        "{}",
        result
            .chunks(40)
            .map(|x| x.join(""))
            .map(|x| format!("{}{}", x, '\n'))
            .collect::<String>()
    );

    println!("\n");

    println!("day9 part1: {:?}", day9::parse_input("src/inputs/day9", 1));
    println!("day9 part2: {:?}", day9::parse_input("src/inputs/day9", 9));
    println!(
        "day11 part1: {:?}",
        day11::monkeys_to_string("src/inputs/day11", 20)
    );

    println!(
        "day11 part2: {:?}",
        day11_2::monkeys_to_string("src/inputs/day11", 10000)
    );

    // println!("day12 part1: {:?}", day12::get_path("src/inputs/day12"));
    // println!(
    //     "day12 part2: {:?}",
    //     day12::get_path_part2("src/inputs/day12")
    // );

    println!(
        "day13 part1: {:?}",
        day13::parse_input("src/inputs/day13").0
    );

    println!(
        "day13 part2: {:?}",
        day13::parse_input("src/inputs/day13").1
    );

    println!(
        "day14 part1: {:?}",
        day14::parse_input("src/inputs/day14").0
    );

    println!(
        "day14 part2: {:?}",
        day14::parse_input("src/inputs/day14").1 + 1
    );

    // println!("day15 part1: {:?}", day15::part_1("src/inputs/day15"));
    // println!(
    //     "day15 part2: {:?}",
    //     day15::part_2(
    //         "src/inputs/day15",
    //         day15::Point::new(0, 0),
    //         day15::Point::new(4000000, 4000000)
    //     )
    // );

    // println!("day 16 part1: {}", day16::part_1("src/inputs/day16"));

    println!("day 18 part1: {}", day18::part_1("src/inputs/day18"));
    println!("day 18 part2: {}", day18::part_2("src/inputs/day18"));
}
