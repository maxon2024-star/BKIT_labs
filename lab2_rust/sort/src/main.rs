use std::io::{self, Write};

fn main() {
    // Чтение входных данных
    let mut input = String::new();
    print!("Введите числа, разделённые пробелами: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");

    // Парсинг входной строки в вектор чисел
    let mut data: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Ошибка парсинга"))
        .collect();

    // Сортировка с использованием lambda-функции
    data.sort_by(|a, b| b.abs().cmp(&a.abs()));

    // Вывод отсортированных данных
    println!("{:?}", data);
    // пример
    //-100 -30 30 100 -100 123 1 0 -1 -4
}

/*
без лямб

use std::io::{self, Write};
use std::cmp::Ordering;

fn compare_by_abs(a: &i32, b: &i32) -> Ordering {
    b.abs().cmp(&a.abs())
}

fn main() {
    // Чтение входных данных
    let mut input = String::new();
    print!("Введите числа, разделённые пробелами: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");

    // Парсинг входной строки в вектор чисел
    let mut data: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Ошибка парсинга"))
        .collect();

    // Сортировка с использованием функции
    data.sort_by(compare_by_abs);

    // Вывод отсортированных данных
    println!("{:?}", data);
}
*/