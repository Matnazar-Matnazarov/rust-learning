# Lesson 06 — Enums, Pattern Matching va `?` operatori (Notes)

## 1) Maqsad

Ushbu darsdan keyin siz:

- `enum` yordamida o‘z turlaringizni yaratishni bilasiz
- `match` bilan exhaustive pattern matching qilishni bilasiz
- `Option` va `Result` bilan `?` operatorini ishlatishni o‘rganasiz
- `if let` va `while let` bilan qisqa pattern matching qilishni bilasiz

## 2) Kod qayerda va qanday run qilinadi?

- Dars kodi: `src/lessons/day06.rs`
- Run:

```bash
cargo run -- day06
```

## 3) Enum asoslari

Enum - bu bir nechta variantga ega bo‘lishi mumkin bo‘lgan tur:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Har bir variant o‘ziga xos ma'lumot saqlashi mumkin.

## 4) Pattern Matching bilan ishlash

### Match bilan exhaustive tekshirish

```rust
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Write: {}", text),
    Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
}
```

### If let bilan qisqa tekshirish

```rust
if let Message::Write(text) = msg {
    println!("Writing: {}", text);
}
```

### While let bilan iteratsiya

```rust
while let Some(value) = stack.pop() {
    println!("Popped: {}", value);
}
```

## 5) `?` operatori

`?` operatori `Result` yoki `Option` ni tekshirib, `Ok` bo‘lsa qiymatni oladi, `Err` bo‘lsa funksiyadan erta qaytadi:

```rust
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

## 6) Mashqlar

- `Shape` enum yarating: `Circle(f64)`, `Rectangle(f64, f64)`, `Triangle(f64, f64, f64)` va maydon hisoblash funksiyasi yozing
- `Option` bilan ishlashda `?` operatorini ishlating
- `if let` va `match` o‘rtasidagi farqni tushuntiring
- `Result` chain qilish uchun `?` operatoridan foydalaning
