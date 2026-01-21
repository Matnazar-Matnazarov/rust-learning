pub fn run() {
    println!("Lesson 03 (day03): ownership and borrowing");
    println!("----------------------------------------");

    copy_vs_move();
    borrowing();
    slices();
}

fn copy_vs_move() {
    println!("\n[1] Copy vs Move");

    let a: i32 = 10;
    let b = a;
    println!("Copy: a = {a}, b = {b}");

    let s1 = String::from("hello");
    let s2 = s1;
    println!("Move: s2 = {s2}");

    let s3 = s2.clone();
    println!("Clone: s2 = {s2}, s3 = {s3}");
}

fn borrowing() {
    println!("\n[2] Borrowing (&T and &mut T)");

    let s = String::from("rust");
    let len = len_str(&s);
    println!("len_str(&s) = {len}");

    let len2 = len_str(s.as_str());
    println!("len_str(s.as_str()) = {len2}");

    let mut name = String::from("Matnazar");
    append_suffix(&mut name, "ov");
    println!("after append_suffix => {name}");

    let n = 5;
    let n2 = add_one(&n);
    println!("add_one(&{n}) => {n2}");
}

fn slices() {
    println!("\n[3] Slices (&str and &[T])");

    let text = String::from("hello rust world");
    let first = first_word(text.as_str());
    println!("first_word = {first}");

    let v = vec![1, 2, 3, 4, 5];
    let part: &[i32] = &v[1..4];
    println!("slice = {:?}, sum = {}", part, sum_slice(part));
}

fn len_str(s: &str) -> usize {
    s.len()
}

fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

fn add_one(n: &i32) -> i32 {
    *n + 1
}

fn first_word(s: &str) -> &str {
    match s.split_whitespace().next() {
        Some(w) => w,
        None => "",
    }
}

fn sum_slice(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}
