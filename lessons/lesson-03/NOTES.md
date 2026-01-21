# Lesson 03 — Ownership & Borrowing (Notes)

## 1) Maqsad

Ushbu darsdan keyin siz:

- Rust’dagi **ownership** nima ekanini tushunasiz
- `move` va `copy` farqini bilasiz
- `&T` (immutable borrow) va `&mut T` (mutable borrow) qoidalarini tushunasiz
- `String` va `&str` bilan ownership qanday ishlashini amalda ko‘rasiz
- `slice` tushunchasini (masalan `&str` va `&[T]`) ishlata olasiz

## 2) Kod qayerda va qanday run qilinadi?

- Dars kodi: `src/lessons/day03.rs`
- Run:

```bash
cargo run -- day03
```

## 3) Ownership: 3 ta asosiy qoida

- Har bir qiymatning **egasi (owner)** bitta bo‘ladi
- Owner scope’dan chiqqanda qiymat **drop** bo‘ladi
- Ownership boshqa o‘zgaruvchiga o‘tsa — avvalgi o‘zgaruvchi endi ishlatilmaydi (**move**)

## 4) Move vs Copy

### Move

`String`, `Vec`, `HashMap` kabi heap’ga ega turlar `let b = a;` qilinganda odatda **move** bo‘ladi:

```rust
let a = String::from("hi");
let b = a; // move
// a bu yerdan keyin ishlamaydi
```

### Copy

`i32`, `u32`, `bool`, `char` kabi oddiy scalar turlar ko‘pincha `Copy`:

```rust
let a: i32 = 10;
let b = a; // copy
println!("{a} {b}");
```

## 5) Borrowing (references)

Ownership’ni ko‘chirmasdan vaqtincha foydalanish:

- `&T` — immutable borrow
- `&mut T` — mutable borrow

### Borrow qoidalari

- Bir vaqtning o‘zida:
  - ko‘p `&T` bo‘lishi mumkin, **yoki**
  - bitta `&mut T` bo‘lishi mumkin
- `&mut` bor bo‘lsa, shu paytda `&` bo‘lmasligi kerak

## 6) `String` bilan ishlash: `&str` vs `String`

- `String` — egalik qiladigan, heap’da o‘suvchi qiymat
- `&str` — string slice (ko‘pincha `String` yoki literal’dan kesib olingan qism)

Amaliy:

- Funksiyaga `&str` berish ko‘p hollarda qulay (literal ham, `String` ham mos keladi)

## 7) Slices

- `&str` — string slice
- `&[T]` — array/vector slice

Slice: ownership olmaydi, faqat view beradi.

## 8) Mashqlar

- `String` yarating va uni funksiya orqali uzunligini hisoblang:
  - 1-variant: ownership bilan (`String` qabul qilib `usize` qaytarish)
  - 2-variant: borrow bilan (`&str` yoki `&String` qabul qilib `usize` qaytarish)
- `Vec<i32>` yarating, `&[i32]` slice qilib funksiya yozing va yig‘indisini qaytaring.
- `&mut String` orqali string’ga suffix qo‘shadigan funksiya yozing.
