# Lesson 02 — Collections: Array, Matrix, `Vec`, `String`, `HashMap`, `HashSet` (Notes)

## 1) Maqsad

Ushbu darsdan keyin siz:

- Array (`[T; N]`) va `Vec<T>` farqini tushunasiz
- 2D “matritsa”ni Rust’da odatda `Vec<Vec<T>>` bilan qanday qilishni bilasiz
- `String` bilan amaliy ishlashni (push/append/format/split) ko‘rasiz
- `HashMap<K, V>` (dict) va `HashSet<T>` (set) bilan boshlang‘ich amallarni bajara olasiz

## 2) Kod qayerda va qanday run qilinadi?

- Dars kodi: `src/lessons/day02.rs`
- Run:

```bash
cargo run -- day02
```

## 3) Array vs Vec

### Array: `[T; N]`

- Uzunligi **compile time** da aniq
- Stack’da saqlanadi (ko‘pincha)
- O‘lcham o‘zgarmaydi

```rust
let a: [i32; 3] = [1, 2, 3];
```

### Vector: `Vec<T>`

- Uzunligi runtime’da o‘zgaradi
- Heap’da saqlanadi
- `push`, `pop`, `insert`, `remove` ishlaydi

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
v.push(4);
```

## 4) Matrix (2D)

Rust’da 2D struktura uchun eng sodda yo‘l:

- `Vec<Vec<T>>` — qatorlar uzunligi turlicha bo‘lishi mumkin
- `Vec<[T; N]>` — har bir qator uzunligi bir xil bo‘lsa

Masalan:

```rust
let matrix: Vec<Vec<i32>> = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
];
```

## 5) String: `&str` vs `String`

- `&str` — slice, ko‘pincha literal va o‘zgarmas
- `String` — heap’da, o‘sadi, mutable bo‘lishi mumkin

Ko‘p uchraydigan amallar:

- `push`, `push_str`
- `format!` bilan string yig‘ish
- `split_whitespace` bilan bo‘lish

## 6) Dict: `HashMap<K, V>`

Rust’da “dict” ga mos tur: `std::collections::HashMap`.

Asosiy amallar:

- `insert(k, v)`
- `get(&k)`
- `entry(k).or_insert(v)` — mavjud bo‘lmasa qo‘shish

## 7) Set: `HashSet<T>`

Rust’da “set” ga mos tur: `std::collections::HashSet`.

- `insert`
- `contains`
- `remove`

## 8) Mashqlar

- `Vec<i32>` yarating, 10 ta element qo‘shing, juftlarini filter qilib chiqaring.
- `HashMap<&str, u32>` qilib “so‘z -> count” qiling (oddiy word count).
- `matrix` yig‘indisini hisoblang.
