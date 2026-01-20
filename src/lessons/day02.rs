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

    for (i, value) in a.iter().enumerate() {
        println!("a[{i}] = {value}");
    }
}

fn vectors() {
    println!("\n[2] Vector: Vec<T>");

    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    println!("v = {:?}", v);

    if let Some(last) = v.pop() {
        println!("popped = {last}, now v = {:?}", v);
    }

    v.extend([10, 20, 30]);
    println!("extended v = {:?}", v);

    let evens: Vec<i32> = v.iter().copied().filter(|x| x % 2 == 0).collect();
    println!("evens = {:?}", evens);
}

fn matrix() {
    println!("\n[3] Matrix (2D): Vec<Vec<T>>");

    let m: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    println!("m = {:?}", m);
    println!("m[1][2] = {}", m[1][2]);

    let mut sum = 0;
    for row in &m {
        for &value in row {
            sum += value;
        }
    }
    println!("sum(matrix) = {sum}");
}

fn strings() {
    println!("\n[4] Strings: &str vs String");

    let s: &str = "hello world";
    println!("&str = {s}");

    let mut owned = String::from("Rust");
    owned.push(' ');
    owned.push_str("lang");
    println!("String = {owned}");

    let composed = format!("{s} | {owned} | {}", 2026);
    println!("format! => {composed}");

    let words: Vec<&str> = s.split_whitespace().collect();
    println!("split_whitespace => {:?}", words);
}

fn hash_map() {
    println!("\n[5] HashMap<K, V> (dict)");

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 2);

    let key = "apple";
    println!("get({key}) = {:?}", map.get(key));

    *map.entry("banana").or_insert(0) += 1;
    *map.entry("orange").or_insert(0) += 1;
    println!("map = {:?}", map);

    let text = "rust is fast and rust is safe";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w).or_insert(0) += 1;
    }
    println!("word_count = {:?}", counts);
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
}
