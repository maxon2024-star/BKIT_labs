use serde::{Deserialize, Serialize};
use serde_json::Value;
use rand::Rng;
use std::time::Instant;

// Структура для описания данных
#[derive(Serialize, Deserialize, Debug)]
struct Vacancy {
    name: String,
    salary: Option<String>,
    location: Option<String>,
}

// Декоратор для вывода результата
fn print_result<T>(result: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", result);
}

// Контекстный менеджер для измерения времени
struct CmTimer<'a> {
    label: &'a str,
    start: Instant,
}

impl<'a> CmTimer<'a> {
    fn new(label: &'a str) -> Self {
        Self {
            label,
            start: Instant::now(),
        }
    }
}

impl<'a> Drop for CmTimer<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        println!("{} took: {:?}", self.label, duration);
    }
}

// f1: Сортировка и вывод уникальных профессий
fn f1(vacancies: &Vec<Vacancy>) -> Vec<String> {
    let mut professions: Vec<String> = vacancies
        .iter()
        .map(|v| v.name.to_lowercase())
        .collect::<Vec<_>>();
    professions.sort();
    professions.dedup();
    professions
}

// f2: Фильтрация профессий, которые начинаются со слова "программист"
fn f2(professions: Vec<String>) -> Vec<String> {
    professions
        .into_iter()
        .filter(|p| p.to_lowercase().starts_with("программист"))
        .collect()
}

// f3: Добавление "с опытом Python"
fn f3(professions: Vec<String>) -> Vec<String> {
    professions
        .into_iter()
        .map(|p| format!("{} с опытом Python", p))
        .collect()
}

// f4: Генерация зарплат
fn f4(professions: Vec<String>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let salaries = (0..professions.len())
        .map(|_| rng.gen_range(100_000..200_000))
        .collect::<Vec<_>>();

    professions
        .into_iter()
        .zip(salaries)
        .map(|(p, s)| format!("{}, зарплата {} руб.", p, s))
        .collect()
}

// Основная функция
fn main() {
    // Пример данных
    let data = r#"
    [
        {"name": "Программист C#", "salary": null, "location": null},
        {"name": "Менеджер проектов", "salary": null, "location": null},
        {"name": "Программист Java", "salary": null, "location": null},
        {"name": "Программист C#", "salary": null, "location": null}
    ]
    "#;

    let vacancies: Vec<Vacancy> = serde_json::from_str(data).expect("Invalid JSON");

    let _timer = CmTimer::new("Processing time");

    let result_f1 = f1(&vacancies);
    print_result(&result_f1);

    let result_f2 = f2(result_f1);
    print_result(&result_f2);

    let result_f3 = f3(result_f2);
    print_result(&result_f3);

    let result_f4 = f4(result_f3);
    print_result(&result_f4);
}