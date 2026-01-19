fn main() {
    let mut args = std::env::args();
    let _program = args.next();

    match args.next().as_deref() {
        Some("day01") | Some("day-01") => lessons::day01::run(),
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
}
