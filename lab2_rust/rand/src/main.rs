use rand::Rng; // Импортируем библиотеку rand для генерации случайных чисел
use std::env;

fn gen_random(count: usize, min: i32, max: i32) -> impl Iterator<Item = i32> {
    let mut rng = rand::thread_rng(); // Создаем генератор случайных чисел
    (0..count).map(move |_| rng.gen_range(min..=max)) // Генерируем случайные числа в заданном диапазоне
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect(); // Пропускаем первый аргумент (имя программы)

    if args.len() != 3 {
        eprintln!("Использование: {} <количество> <минимум> <максимум>", args[0]);
        return;
    }

    let count: usize = args[0].parse().expect("Некорректное количество");
    let min: i32 = args[1].parse().expect("Некорректное минимальное значение");
    let max: i32 = args[2].parse().expect("Некорректное максимальное значение");

    if min > max {
        eprintln!("Минимальное значение должно быть меньше или равно максимальному.");
        return;
    }

    let random_numbers: Vec<i32> = gen_random(count, min, max).collect();
    for number in random_numbers {
        println!("{}", number);
    }
}