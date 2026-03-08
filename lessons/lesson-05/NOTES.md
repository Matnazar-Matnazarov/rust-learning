# Lesson 05 — Structs & `impl`: data modeling, methods, constructors (Notes)

## 1) Maqsad

Ushbu darsdan keyin siz:

- `struct` orqali data model qilishni bilasiz
- `impl` blok orqali method yozishni bilasiz
- Associated function (`new`, `from_*`) bilan “constructor-style” API qurishni bilasiz
- `derive(Debug, Clone, Copy)` kabi trait’lar nima uchun kerakligini tushunasiz
- Field update syntax va destructuring (`let Point { x, y } = p;`) bilan tanishasiz

## 2) Kod qayerda va qanday run qilinadi?

- Dars kodi: `src/lessons/day05.rs`
- Run:

```bash
cargo run -- day05
```

## 3) Struct turlari

### Named-field struct

```rust
struct User {
    name: String,
    age: u8,
}
```

### Tuple struct

```rust
struct Rgb(u8, u8, u8);
```

### Unit-like struct

```rust
struct Marker;
```

## 4) `impl`: methods vs associated functions

### Method

- Birinchi parametri `self` (`self`, `&self`, `&mut self`)

```rust
impl User {
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}
```

### Associated function

- `self` olmaydi
- Ko‘pincha `new(...)` ko‘rinishida bo‘ladi

```rust
impl User {
    fn new(name: impl Into<String>, age: u8) -> Self {
        Self { name: name.into(), age }
    }
}
```

## 5) `derive(Debug, Clone, Copy)`

- `Debug` — `println!("{:?}", value)` uchun
- `Clone` — `value.clone()` bilan nusxa olish
- `Copy` — move o‘rniga copy (faqat “cheap” turlar uchun)

## 6) Field update syntax

```rust
let u1 = User::new("Ali", 20);
let u2 = User { age: 21, ..u1 };
```

Eslatma: `..u1` ownership/clone masalalarini keltirib chiqarishi mumkin (ayniqsa `String`).

## 7) Mashqlar

- `Rectangle` struct yarating: `width`, `height`
  - `area()` method
  - `can_hold(&self, other: &Rectangle) -> bool` method
- `Point` struct yarating
  - `distance_to_origin()` method
- `User` uchun `birthday(&mut self)` method yozing
