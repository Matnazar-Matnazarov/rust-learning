#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: impl Into<String>, age: u8) -> Self {
        Self {
            name: name.into(),
            age,
        }
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn birthday(&mut self) {
        self.age = self.age.saturating_add(1);
    }
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

struct Rgb(u8, u8, u8);

pub fn run() {
    println!("Lesson 05 (day05): structs and impl");
    println!("------------------------------");

    point_demo();
    user_demo();
    rectangle_demo();
    tuple_struct_demo();
}

fn point_demo() {
    println!("\n[1] Point struct");

    let p = Point::new(3.0, 4.0);
    println!("p = {:?}", p);
    println!("distance_to_origin = {}", p.distance_to_origin());

    let Point { x, y } = p;
    println!("destructure => x = {x}, y = {y}");
}

fn user_demo() {
    println!("\n[2] User struct");

    let mut u = User::new("Ali", 17);
    println!("u = {:?}", u);
    println!("name = {}", u.name);
    println!("is_adult = {}", u.is_adult());

    u.birthday();
    println!("after birthday => u = {:?}", u);
    println!("is_adult = {}", u.is_adult());

    let u2 = User {
        age: 21,
        ..u.clone()
    };
    println!("field update syntax => u2 = {:?}", u2);
}

fn rectangle_demo() {
    println!("\n[3] Rectangle struct");

    let r1 = Rectangle::new(30, 50);
    let r2 = Rectangle::new(10, 40);
    println!("r1 = {:?}, area = {}", r1, r1.area());
    println!("r2 = {:?}, area = {}", r2, r2.area());
    println!("r1 can_hold r2? {}", r1.can_hold(&r2));
}

fn tuple_struct_demo() {
    println!("\n[4] Tuple struct");

    let color = Rgb(255, 128, 0);
    println!("Rgb = ({}, {}, {})", color.0, color.1, color.2);
}
