pub fn run() {
    println!("Lesson 04 (day04): control flow and Option/Result patterns");
    println!("--------------------------------------------------------");

    if_else();
    loops();
    matching();
    option_patterns();
    result_patterns();
}

fn if_else() {
    println!("\n[1] if / else");

    let n = 10;
    let kind = if n % 2 == 0 { "even" } else { "odd" };
    println!("n = {n}, kind = {kind}");

    let temperature = 27;
    if temperature >= 30 {
        println!("hot");
    } else if temperature >= 20 {
        println!("warm");
    } else {
        println!("cold");
    }
}

fn loops() {
    println!("\n[2] loop / while / for");

    let mut counter = 0;
    let loop_value = loop {
        counter += 1;
        if counter == 3 {
            break counter * 10;
        }
    };
    println!("loop_value = {loop_value}");

    let mut x = 3;
    while x > 0 {
        println!("while x = {x}");
        x -= 1;
    }

    let mut sum = 0;
    for i in 1..=5 {
        sum += i;
    }
    println!("sum(1..=5) = {sum}");
}

fn matching() {
    println!("\n[3] match");

    let day = 3;
    let name = match day {
        1 => "Mon",
        2 => "Tue",
        3 => "Wed",
        4 => "Thu",
        5 => "Fri",
        6 | 7 => "Weekend",
        _ => "Invalid",
    };
    println!("day = {day}, name = {name}");

    let score = 87;
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    };
    println!("score = {score}, grade = {grade}");
}

fn option_patterns() {
    println!("\n[4] Option patterns");

    let v: Vec<i32> = vec![];
    let first = v.first().copied();
    println!("first(empty vec) = {first:?}");

    let value = first.unwrap_or(0);
    println!("unwrap_or(0) => {value}");

    let text = "rust";
    let upper_first = text
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "".to_string());
    println!("upper_first = '{upper_first}'");

    let number: Option<i32> = Some(10);
    if let Some(n) = number {
        println!("if let Some(n) => {n}");
    }
}

fn result_patterns() {
    println!("\n[5] Result patterns");

    println!("parse_i32('42')   => {:?}", parse_i32("42"));
    println!("parse_i32('nope') => {:?}", parse_i32("nope"));

    match divide(10, 2) {
        Ok(v) => println!("10 / 2 = {v}"),
        Err(e) => println!("divide error: {e}"),
    }

    match divide(10, 0) {
        Ok(v) => println!("10 / 0 = {v}"),
        Err(e) => println!("divide error: {e}"),
    }

    match sum_two_numbers("7", "8") {
        Ok(v) => println!("sum_two_numbers = {v}"),
        Err(e) => println!("sum_two_numbers error: {e}"),
    }
}

fn parse_i32(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| e.to_string())
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("division by zero".to_string());
    }
    Ok(a / b)
}

fn sum_two_numbers(a: &str, b: &str) -> Result<i32, String> {
    let x = parse_i32(a)?;
    let y = parse_i32(b)?;
    Ok(x + y)
}
