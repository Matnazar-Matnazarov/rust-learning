use std::collections::HashMap;

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

#[derive(Debug)]
enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

pub fn run() {
    println!("Lesson 06 (day06): enums, pattern matching, and ? operator");
    println!("-------------------------------------------------------");

    enum_basics();
    pattern_matching();
    shape_area_demo();
    option_result_with_question();
    if_let_while_let_demo();
}

fn enum_basics() {
    println!("\n[1] Enum basics");

    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write = Message::Write(String::from("Hello Rust"));
    let color = Message::ChangeColor(255, 128, 0);

    println!("quit = {:?}", quit);
    println!("move_msg = {:?}", move_msg);
    println!("write = {:?}", write);
    println!("color = {:?}", color);

    process_message(&quit);
    process_message(&move_msg);
    process_message(&write);
    process_message(&color);
}

fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("Processing: Quit command"),
        Message::Move { x, y } => println!("Processing: Move to ({}, {})", x, y),
        Message::Write(text) => println!("Processing: Write '{}'", text),
        Message::ChangeColor(r, g, b) => println!("Processing: Change color to RGB({}, {}, {})", r, g, b),
    }
}

fn pattern_matching() {
    println!("\n[2] Pattern matching with Operation enum");

    let ops = vec![
        Operation::Add(5, 3),
        Operation::Subtract(10, 4),
        Operation::Multiply(6, 7),
        Operation::Divide(20, 4),
        Operation::Divide(10, 0),
    ];

    for op in ops {
        let result = calculate(&op);
        println!("{:?} = {:?}", op, result);
    }
}

fn calculate(op: &Operation) -> Option<i32> {
    match op {
        Operation::Add(a, b) => Some(a + b),
        Operation::Subtract(a, b) => Some(a - b),
        Operation::Multiply(a, b) => Some(a * b),
        Operation::Divide(a, b) => {
            if *b == 0 {
                None
            } else {
                Some(a / b)
            }
        }
    }
}

fn shape_area_demo() {
    println!("\n[3] Shape enum with area calculation");

    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(10.0, 20.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in shapes {
        let area = calculate_area(&shape);
        println!("{:?} area = {:.2}", shape, area);
    }
}

fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn option_result_with_question() {
    println!("\n[4] ? operator with Option and Result");

    let numbers = vec!["42", "100", "invalid", "50"];
    
    for num_str in numbers {
        match parse_and_double(num_str) {
            Some(result) => println!("parse_and_double('{}') = {}", num_str, result),
            None => println!("parse_and_double('{}') failed", num_str),
        }
    }

    let config_result = load_config();
    match config_result {
        Ok(config) => println!("Config loaded: {:?}", config),
        Err(e) => println!("Config error: {}", e),
    }
}

fn parse_and_double(s: &str) -> Option<i32> {
    let num = s.parse::<i32>().ok()?;
    Some(num * 2)
}

fn load_config() -> Result<HashMap<String, String>, String> {
    let mut config = HashMap::new();
    
    let key = get_config_key()?;
    let value = get_config_value(&key)?;
    
    config.insert(key, value);
    Ok(config)
}

fn get_config_key() -> Result<String, String> {
    Ok(String::from("database_url"))
}

fn get_config_value(_key: &str) -> Result<String, String> {
    Ok(String::from("postgres://localhost/mydb"))
}

fn if_let_while_let_demo() {
    println!("\n[5] if let and while let");

    let msg = Message::Write(String::from("Hello"));
    
    if let Message::Write(text) = &msg {
        println!("if let captured: {}", text);
    } else {
        println!("if let: not a Write message");
    }

    let mut stack: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Popping from stack:");
    while let Some(value) = stack.pop() {
        print!("{} ", value);
    }
    println!();
}
