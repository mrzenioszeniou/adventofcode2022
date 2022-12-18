mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod dir;
mod pf;

fn main() {
    let arg = std::env::args().nth(1).map(|arg| arg.to_lowercase());

    match arg.as_deref() {
        Some("1") => day1::Day1::solve_n_print(),
        Some("2") => day2::Day2::solve_n_print(),
        Some("3") => day3::Day3::solve_n_print(),
        Some("4") => day4::Day4::solve_n_print(),
        Some("5") => day5::Day5::solve_n_print(),
        Some("6") => day6::Day6::solve_n_print(),
        Some("7") => day7::Day7::solve_n_print(),
        Some("8") => day8::Day8::solve_n_print(),
        Some("9") => day9::Day9::solve_n_print(),
        Some("10") => day10::Day10::solve_n_print(),
        Some("11") => day11::Day11::solve_n_print(),
        Some("12") => day12::Day12::solve_n_print(),
        Some("13") => day13::Day13::solve_n_print(),
        Some("14") => day14::Day14::solve_n_print(),
        Some("15") => day15::Day15::solve_n_print(),
        Some("16") => day16::Day16::solve_n_print(),
        Some("17") => day17::Day17::solve_n_print(),
        Some("18") => day18::Day18::solve_n_print(),
        Some("all") => {
            day1::Day1::solve_n_print();
            day2::Day2::solve_n_print();
            day3::Day3::solve_n_print();
            day4::Day4::solve_n_print();
            day5::Day5::solve_n_print();
            day6::Day6::solve_n_print();
            day7::Day7::solve_n_print();
            day8::Day8::solve_n_print();
            day9::Day9::solve_n_print();
            day10::Day10::solve_n_print();
            day11::Day11::solve_n_print();
            day12::Day12::solve_n_print();
            day13::Day13::solve_n_print();
            day14::Day14::solve_n_print();
            day15::Day15::solve_n_print();
            day16::Day16::solve_n_print();
            day17::Day17::solve_n_print();
            day18::Day18::solve_n_print();
        }
        _ => {
            usage();
            std::process::exit(1);
        }
    }
}

fn usage() {
    println!("USAGE:\n    adventofcode2022 DAY | ALL");
}

pub trait Day {
    const NAME: &'static str;

    fn solve() -> anyhow::Result<(String, String)>;

    fn solve_n_print() {
        match Self::solve() {
            Ok((part1, part2)) => {
                println!("{}\n├ Part 1: {part1}\n└ Part 2: {part2}", Self::NAME)
            }
            Err(err) => {
                println!("Failed to solve '{}':\n{err}", Self::NAME);
                std::process::exit(1);
            }
        }
    }
}
