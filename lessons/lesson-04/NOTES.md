# Lesson 04 — Control Flow: `if`, loops, `match`, `Option`/`Result` patterns (Notes)

## 1) Maqsad

Ushbu darsdan keyin siz:

- `if/else` bilan shartli ishlashni bilasiz
- `loop`, `while`, `for` sikllarini farqlaysiz
- `match` orqali kuchli branching (pattern matching) qilasiz
- `Option` va `Result` bilan xavfsiz ishlashni (error handling patterns) boshlaysiz

## 2) Kod qayerda va qanday run qilinadi?

- Dars kodi: `src/lessons/day04.rs`
- Run:

```bash
cargo run -- day04
```

## 3) `if/else`

`if` expression hisoblanadi (ya’ni qiymat qaytarishi mumkin):

```rust
let n = 10;
let kind = if n % 2 == 0 { "even" } else { "odd" };
```

Shart **`bool`** bo‘lishi shart (C/C++ dagidek 0/1 ishlamaydi).

## 4) Loops

### `loop`

- cheksiz loop
- `break` bilan to‘xtaydi
- `break <value>` qilib qiymat qaytarishi mumkin

### `while`

- shart `true` bo‘lsa ishlaydi

### `for`

- iteratsiya (iterator) ustidan yurish
- ko‘pincha eng xavfsiz va qulay variant

## 5) `match`

`match` Rust’ning kuchli imkoniyati:

- barcha holatlar qoplanishi kerak (exhaustive)
- `Option`/`Result` bilan ishlashda juda foydali

## 6) `Option<T>` bilan ishlash

`Option` qiymat bor/yo‘qligini bildiradi:

- `Some(value)`
- `None`

Ko‘p ishlatiladigan patternlar:

- `match opt { Some(v) => ..., None => ... }`
- `if let Some(v) = opt { ... }`
- `unwrap_or(default)`
- `map`, `and_then` (chain)

## 7) `Result<T, E>` bilan ishlash

`Result` muvaffaqiyat/xatoni bildiradi:

- `Ok(value)`
- `Err(error)`

Ko‘p ishlatiladigan patternlar:

- `match` bilan tahlil qilish
- `?` operatori (xatoni yuqoriga uzatish)
- `map`, `map_err`

## 8) Mashqlar

- `for` bilan 1..=20 oralig‘ida faqat juft sonlarni chiqaring.
- `loop` yordamida counter qiling va `break` bilan qiymat qaytaring.
- `Option` uchun `first()` dan foydalanib bo‘sh `Vec` holatini ham tekshiring.
- `Result` uchun `parse::<i32>()` qilib, xato bo‘lsa chiroyli xabar chiqaring.
