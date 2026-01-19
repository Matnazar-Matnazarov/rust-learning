pub fn run() {
    println!("Lesson 01 (day01): variables, types, and basics");
    println!("--------------------------------------------");

    const MAX_POINTS: u32 = 100;
    println!("const MAX_POINTS = {MAX_POINTS}");

    let x = 10;
    println!("let x = {x} (immutable by default)");

    let mut y = 5;
    println!("let mut y = {y} (before)");
    y = y + 1;
    println!("y = {y} (after)");

    let z = 3;
    let z = z + 1;
    let z = z * 2;
    println!("shadowing z => {z}");

    println!("\nScalar types");
    let a: i32 = -42;
    let b: u32 = 42;
    let c: f64 = 3.14;
    let ok: bool = true;
    let ch: char = 'R';
    println!("i32  a  = {a}");
    println!("u32  b  = {b}");
    println!("f64  c  = {c}");
    println!("bool ok = {ok}");
    println!("char ch = {ch}");

    println!("\nString types");
    let s: &str = "hello";
    let mut owned: String = String::from("Rust");
    owned.push('!');
    println!("&str    = {s}");
    println!("String  = {owned}");
    println!("len(&str)   = {}", s.len());
    println!("len(String) = {}", owned.len());

    println!("\nCompound types");
    let pair: (i32, bool) = (10, true);
    println!("tuple pair = ({}, {})", pair.0, pair.1);
    let arr: [i32; 3] = [1, 2, 3];
    println!("array arr[0] = {}, len = {}", arr[0], arr.len());

    println!("\nType annotations and casting");
    let inferred = 12;
    let annotated: i64 = 12;
    println!("inferred (i32 by default) = {inferred}");
    println!("annotated (i64) = {annotated}");
    let sum64: i64 = inferred as i64 + b as i64;
    println!("casted sum (i64) = {sum64}");

    println!("\nFunctions");
    let sum = add(2, 3);
    println!("add(2, 3) = {sum}");

    println!("\nOption and Result (intro)");
    let maybe = first_char("Rust");
    println!("first_char(\"Rust\") = {maybe:?}");

    let parsed_ok = parse_u32("42");
    let parsed_err = parse_u32("nope");
    println!("parse_u32(\"42\")   = {parsed_ok:?}");
    println!("parse_u32(\"nope\") = {parsed_err:?}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse::<u32>().map_err(|e| e.to_string())
}
