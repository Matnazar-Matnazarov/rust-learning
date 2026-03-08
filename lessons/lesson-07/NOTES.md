# Lesson 07 — Traits va Generics (Notes)

## 1) Maqsad

Ushbu darsdan keyin siz:

- `trait` yordamida interfeyslar yaratishni bilasiz
- Generics bilan turli turlar uchun umumiy kod yozishni o‘rganasiz
- Trait bounds (`T: Trait`) bilan cheklangan generic turlar yaratishni bilasiz
- `impl Trait` sintaksisi bilan ishlashni o‘rganasiz
- Default trait methodlaridan foydalanishni bilasiz

## 2) Kod qayerda va qanday run qilinadi?

- Dars kodi: `src/lessons/day07.rs`
- Run:

```bash
cargo run -- day07
```

## 3) Trait asoslari

Trait - bu metodlar to‘plamini ta’riflaydigan interfeys:

```rust
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}
```

Trait'ni struct yoki enum'ga implement qilish:

```rust
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
```

## 4) Generics bilan ishlash

Generics - bir xil logikani turli turlar uchun ishlatish:

```rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

## 5) Trait bounds

Generic tur uchun cheklovlar qo‘yish:

```rust
fn process<T: Display + Debug>(item: T) {
    println!("{} {:?}", item, item);
}
```

## 6) `impl Trait` sintaksisi

Funksiya return type sifatida trait ishlatish:

```rust
fn create_drawable() -> impl Drawable {
    Circle::new(5.0)
}
```

## 7) Default methodlar

Trait'da default implementatsiya:

```rust
trait Greet {
    fn greet(&self) {
        println!("Hello!");
    }
}
```

## 8) Mashqlar

- `Summary` trait yarating va `Article`, `Tweet` struct'lari uchun implement qiling
- `Container<T>` generic struct yarating
- `PartialEq` va `Debug` trait'larini derive qilib ko‘ring
- Trait bounds bilan generic funksiya yozing
