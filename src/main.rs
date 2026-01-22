fn main() {
    let mut args = std::env::args();
    let _program: Option<String> = args.next();

    match args.next().as_deref() {
        Some("day01") | Some("day-01") => lessons::day01::run(),
        Some("day02") | Some("day-02") => lessons::day02::run(),
        Some("day03") | Some("day-03") => lessons::day03::run(),
        Some("day04") | Some("day-04") => lessons::day04::run(),
        Some("help") | Some("--help") | Some("-h") => print_help(),
        Some(other) => {
            eprintln!("Unknown command: {other}");
            print_help();
        }
        None => print_help(),
    }
}

mod lessons;

fn print_help() {
    println!("Usage:");
    println!("  cargo run -- day01");
    println!("  cargo run -- day02");
    println!("  cargo run -- day03");
    println!("  cargo run -- day04");
}
