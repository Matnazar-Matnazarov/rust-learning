use std::fmt::Debug;

pub fn run() {
    println!("Lesson 07 (day07): traits and generics");
    println!("--------------------------------------");

    trait_basics();
    generics_demo();
    trait_bounds_demo();
    impl_trait_demo();
    default_methods_demo();
    container_generic();
}

// Trait definitions
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
    fn describe(&self) {
        println!("This is a drawable object with area: {:.2}", self.area());
    }
}

trait Summary {
    fn summarize(&self) -> String;
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// Structs
#[derive(Debug, Clone)]
struct Circle {
    radius: f64,
}

#[derive(Debug, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Article {
    title: String,
    author: String,
    content: String,
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
}

// Implementations
impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Generic Container
#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn set(&mut self, value: T) {
        self.value = value;
    }
}

fn trait_basics() {
    println!("\n[1] Trait basics");

    let circle = Circle::new(5.0);
    let rect = Rectangle::new(10.0, 20.0);

    circle.draw();
    circle.describe();
    
    rect.draw();
    rect.describe();

    println!("Circle area: {:.2}", circle.area());
    println!("Rectangle area: {:.2}", rect.area());
}

fn generics_demo() {
    println!("\n[2] Generics demo");

    let int_max = max(10, 20);
    let float_max = max(3.14, 2.71);
    let str_max = max("apple", "banana");

    println!("max(10, 20) = {}", int_max);
    println!("max(3.14, 2.71) = {}", float_max);
    println!("max('apple', 'banana') = {}", str_max);

    let pair1 = Pair::new(1, 2);
    let pair2 = Pair::new("hello", "world");

    println!("Pair 1: first={}, second={}", pair1.first, pair1.second);
    println!("Pair 2: first={}, second={}", pair2.first, pair2.second);
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Self { first, second }
    }
}

fn trait_bounds_demo() {
    println!("\n[3] Trait bounds demo");

    let article = Article {
        title: String::from("Rust Programming"),
        author: String::from("Jane Doe"),
        content: String::from("Rust is a systems programming language..."),
    };

    let tweet = Tweet {
        username: String::from("@rustacean"),
        content: String::from("Learning Rust is fun!"),
    };

    notify(&article);
    notify(&tweet);

    println!("Article default: {}", article.default_summary());
    println!("Article content preview: {}...", &article.content[..20.min(article.content.len())]);
}

fn notify<T: Summary>(item: &T) {
    println!("Breaking! {}", item.summarize());
}

fn impl_trait_demo() {
    println!("\n[4] impl Trait demo");

    let drawable = create_drawable(true);
    drawable.draw();
    drawable.describe();

    let items = vec![
        create_drawable(false),
        create_drawable(true),
    ];

    for item in &items {
        item.draw();
    }
}

fn create_drawable(is_circle: bool) -> Box<dyn Drawable> {
    if is_circle {
        Box::new(Circle::new(3.0))
    } else {
        Box::new(Rectangle::new(5.0, 10.0))
    }
}

fn default_methods_demo() {
    println!("\n[5] Default trait methods");

    let article = Article {
        title: String::from("Advanced Rust"),
        author: String::from("John Smith"),
        content: String::from("Traits are powerful..."),
    };

    println!("Summary: {}", article.summarize());
    println!("Default summary: {}", article.default_summary());
    println!("Full article content: {}", article.content);
}

fn container_generic() {
    println!("\n[6] Generic Container<T>");

    let mut int_container = Container::new(42);
    println!("Container value: {}", int_container.get());
    
    int_container.set(100);
    println!("After set: {}", int_container.get());

    let string_container = Container::new(String::from("Hello"));
    println!("String container: {:?}", string_container);

    let circle_container = Container::new(Circle::new(5.0));
    println!("Circle container: {:?}", circle_container);
    println!("Circle area: {:.2}", circle_container.get().area());
}
