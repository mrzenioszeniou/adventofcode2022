mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let arg = std::env::args().nth(1).map(|arg| arg.to_lowercase());

    match arg.as_deref() {
        Some("1") => day1::Day1::solve_n_print(),
        Some("2") => day2::Day2::solve_n_print(),
        Some("3") => day3::Day3::solve_n_print(),
        Some("4") => day4::Day4::solve_n_print(),
        Some("all") => {
            day1::Day1::solve_n_print();
            day2::Day2::solve_n_print();
            day3::Day3::solve_n_print();
            day4::Day4::solve_n_print();
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
                println!("Failed to solve {}: {err}", Self::NAME);
                std::process::exit(1);
            }
        }
    }
}
