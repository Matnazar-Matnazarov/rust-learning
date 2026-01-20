use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("Lesson 02 (day02): collections and strings");
    println!("---------------------------------------");

    arrays();
    vectors();
    matrix();
    strings();
    hash_map();
    hash_set();
}

fn arrays() {
    println!("\n[1] Array: [T; N]");

    let a: [i32; 3] = [1, 2, 3];
    println!("a = {:?}", a);
    println!("a[0] = {}, len = {}", a[0], a.len());

    println!("get(10) = {:?}", a.get(10));

    for (i, value) in a.iter().enumerate() {
        println!("a[{i}] = {value}");
    }

    let doubled: [i32; 3] = a.map(|x| x * 2);
    println!("mapped (x2) = {:?}", doubled);
}

fn vectors() {
    println!("\n[2] Vector: Vec<T>");

    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("capacity(before) = {}", v.capacity());
    v.push(4);
    println!("v = {:?}", v);

    if let Some(last) = v.pop() {
        println!("popped = {last}, now v = {:?}", v);
    }

    v.extend([10, 20, 30]);
    println!("extended v = {:?}", v);

    v.insert(0, 99);
    println!("after insert(0, 99) = {:?}", v);
    let removed = v.remove(0);
    println!("remove(0) => {removed}, v = {:?}", v);

    if let Some(first) = v.first() {
        println!("first() = {first}");
    }

    if let Some(value) = v.get(3) {
        println!("get(3) = {value}");
    }

    if let Some(slot) = v.get_mut(0) {
        *slot *= -1;
    }
    println!("after get_mut(0) *=-1 => {:?}", v);

    let mut sortable = vec![3, 1, 2, 2, 5, 4];
    sortable.sort();
    sortable.dedup();
    println!("sort + dedup => {:?}", sortable);

    let mut retained = vec![1, 2, 3, 4, 5, 6];
    retained.retain(|x| x % 2 == 0);
    println!("retain(evens) => {:?}", retained);

    let evens: Vec<i32> = v.iter().copied().filter(|x| x % 2 == 0).collect();
    println!("evens = {:?}", evens);
}

fn matrix() {
    println!("\n[3] Matrix (2D): Vec<Vec<T>>");

    let m: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    println!("m = {:?}", m);
    println!("m[1][2] = {}", m[1][2]);

    println!("rows = {}, cols(row0) = {}", m.len(), m[0].len());

    let mut sum = 0;
    for row in &m {
        for &value in row {
            sum += value;
        }
    }
    println!("sum(matrix) = {sum}");

    let flattened: Vec<i32> = m.iter().flat_map(|row| row.iter().copied()).collect();
    println!("flattened = {:?}", flattened);

    let mut mutable = m.clone();
    for row in &mut mutable {
        for value in row.iter_mut() {
            *value *= 10;
        }
    }
    println!("scaled (x10) = {:?}", mutable);
}

fn strings() {
    println!("\n[4] Strings: &str vs String");

    let s: &str = "hello world";
    println!("&str = {s}");

    let mut owned = String::from("Rust");
    owned.push(' ');
    owned.push_str("lang");
    println!("String = {owned}");

    let trimmed = "  spaced  ".trim();
    println!("trim() => '{trimmed}'");

    let composed = format!("{s} | {owned} | {}", 2026);
    println!("format! => {composed}");

    let words: Vec<&str> = s.split_whitespace().collect();
    println!("split_whitespace => {:?}", words);

    let joined = words.join("-");
    println!("join('-') => {joined}");

    println!("starts_with('he')? {}", s.starts_with("he"));
    println!("contains('wor')? {}", s.contains("wor"));

    let bytes = s.as_bytes();
    println!("as_bytes len = {}", bytes.len());
}

fn hash_map() {
    println!("\n[5] HashMap<K, V> (dict)");

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 2);

    let key = "apple";
    println!("get({key}) = {:?}", map.get(key));

    map.entry("banana").and_modify(|v| *v += 1).or_insert(1);
    map.entry("orange").and_modify(|v| *v += 1).or_insert(1);
    println!("map = {:?}", map);

    println!("keys()  => {:?}", map.keys().collect::<Vec<_>>());
    println!("values() => {:?}", map.values().collect::<Vec<_>>());

    println!("remove('banana') => {:?}", map.remove("banana"));
    println!("after remove => {:?}", map);

    let text = "rust is fast and rust is safe";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w).or_insert(0) += 1;
    }
    println!("word_count = {:?}", counts);

    let mut pairs: Vec<(&str, u32)> = counts.into_iter().collect();
    pairs.sort_by_key(|(_, c)| *c);
    println!("sorted by count => {:?}", pairs);
}

fn hash_set() {
    println!("\n[6] HashSet<T> (set)");

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2);
    println!("set = {:?}", set);

    println!("contains 2? {}", set.contains(&2));
    set.remove(&2);
    println!("after remove 2 => {:?}", set);

    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
    let union: HashSet<i32> = a.union(&b).copied().collect();
    let inter: HashSet<i32> = a.intersection(&b).copied().collect();
    let diff: HashSet<i32> = a.difference(&b).copied().collect();
    println!("union = {:?}", union);
    println!("intersection = {:?}", inter);
    println!("difference = {:?}", diff);
}
