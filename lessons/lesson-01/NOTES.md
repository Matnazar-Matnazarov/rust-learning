# Lesson 01 — Rustga kirish: `cargo`, o‘zgaruvchilar va turlar (Notes)

## 1) Maqsad

Ushbu darsdan keyin siz:

- Rust toolchain (`rustup`, `rustc`, `cargo`) nima qilishini bilasiz
- Loyihani ishga tushirishni bilasiz
- `let`, `mut`, shadowing tushunchalarini amalda ko‘rasiz
- Asosiy scalar va compound turlarni (types) tushunasiz
- `&str` va `String` farqini bilasiz

## 2) Repo’da dars kodi qayerda?

- Ishlaydigan dars kodi: `src/lessons/day01.rs`
- Darsni ishga tushirish: 

```bash
cargo run -- day01
```

Agar `Unknown command` chiqsa, `src/main.rs`dagi `print_help()` dan ko‘rsatmalarni tekshiring.

## 3) `cargo` asoslari

- `cargo new <name>` — yangi loyiha yaratadi
- `cargo init` — mavjud papkani Rust loyihaga aylantiradi
- `cargo run` — compile + run
- `cargo build` — faqat compile
- `cargo check` — tez tekshiradi (ko‘pincha compile bosqichigacha bormaydi)

## 4) O‘zgaruvchilar: `let`, `mut` va immutability

Rust’da default holatda o‘zgaruvchi **o‘zgarmas (immutable)**:

```rust
let x = 10; // x ni keyin o‘zgartirib bo‘lmaydi
```

Agar o‘zgartirmoqchi bo‘lsangiz `mut` ishlatasiz:

```rust
let mut y = 5;
y = y + 1;
```

Bu yondashuv sizni tasodifiy bug’lardan saqlaydi: kodda qaysi qiymatlar o‘zgarishini aniq ko‘rasiz.

## 5) Shadowing (ustidan yozish)

Shadowing — bir xil nom bilan yangi binding yaratish:

```rust
let z = 3;
let z = z + 1;
let z = z * 2;
```

Shadowing `mut`dan farq qiladi:

- `mut` — o‘sha binding’ning qiymatini o‘zgartirasiz
- shadowing — yangi binding yaratasiz (ko‘pincha type ham o‘zgarishi mumkin)

## 6) Types: scalar turlar

Rust’da ko‘p type’lar aniq:

- `i32` — signed integer (manfiy bo‘lishi mumkin)
- `u32` — unsigned integer (manfiy bo‘lmaydi)
- `f64` — floating-point
- `bool` — `true/false`
- `char` — bitta Unicode belgi

Qachon qaysi?

- Agar manfiy qiymat bo‘lishi mumkin bo‘lsa: `i32`
- Faqat musbat bo‘lsa: `u32`
- Decimal kerak bo‘lsa: `f64`

## 7) Compound turlar: tuple va array

- Tuple: turli turlarni bitta qiymatda ushlaydi

```rust
let pair: (i32, bool) = (10, true);
let first = pair.0;
```

- Array: hamma element bir xil turda, uzunligi compile time’da ma’lum

```rust
let a: [i32; 3] = [1, 2, 3];
let first = a[0];
```

## 8) `&str` va `String`

- `&str` — string slice (odatda read-only, ko‘pincha literal: `"hello"`)
- `String` — heap’da saqlanadigan, o‘sadigan (growable) string

Misol:

```rust
let s: &str = "hello";
let owned: String = String::from("Rust");
```

## 9) Type conversion (casting)

Rust type’larni “o‘zi” aralashtirib yubormaydi. Kerak bo‘lsa explicit konvertatsiya qilasiz:

```rust
let a: i32 = 10;
let b: u32 = 20;
let sum = a as i64 + b as i64;
```

## 10) Funksiyalar

Rust’da funksiya return type aniq beriladi:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Oxirgi expression `;`siz bo‘lsa return bo‘ladi.

## 11) 12 ta darslik roadmap (taklif)

- Lesson 01: `cargo`, `let/mut`, shadowing, types (scalar/compound), `String` vs `&str`
- Lesson 02: Control flow — `if`, `loop`, `while`, `for`, `match`
- Lesson 03: Ownership — move/copy, borrowing, references
- Lesson 04: Slices, `String` chuqurroq, collections kirish
- Lesson 05: Struct’lar, `impl`, methods
- Lesson 06: Enum, `Option`, `Result`, error handling
- Lesson 07: Modules, crates, `pub`, project structure
- Lesson 08: Traits, generics (asoslar)
- Lesson 09: Iterators, closures
- Lesson 10: Collections (`Vec`, `HashMap`) va lifetimes kirish
- Lesson 11: Testing (`#[test]`), `cargo test`, basic benchmarking fikri
- Lesson 12: Mini project (CLI) — argument parsing, fayl o‘qish/yozish, error handling

## 12) Mashqlar

- `src/lessons/day01.rs` ni ochib, yangi misol qo‘shing:
  - `const` e’lon qiling
  - tuple’dan qiymat oling
  - array uzunligini `len()` bilan chiqarib ko‘ring
- `cargo run -- day01` qilib natijani ko‘ring
