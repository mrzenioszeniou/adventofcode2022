fn main() {
    let arg = std::env::args().nth(1).map(|arg| arg.to_lowercase());

    match arg.as_deref() {
        Some("all") => {}
        _ => {
            usage();
            std::process::exit(1);
        }
    }
}

fn usage() {
    println!("USAGE:\n    adventofcode2022 DAY | ALL");
}
