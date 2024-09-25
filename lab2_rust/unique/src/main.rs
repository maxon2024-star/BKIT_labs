use std::collections::HashSet;
use std::env;
use std::iter::FromIterator;

pub struct Unique<I> {
    iter: I,
    seen: HashSet<String>,
    ignore_case: bool,
}

impl<I> Unique<I>
where
    I: Iterator,
{
    pub fn new(iter: I, ignore_case: bool) -> Self {
        Unique {
            iter,
            seen: HashSet::new(),
            ignore_case,
        }
    }
}

impl<I> Iterator for Unique<I>
where
    I: Iterator<Item = String>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            let key = if self.ignore_case {
                item.to_lowercase()
            } else {
                item.clone()
            };

            if !self.seen.contains(&key) {
                self.seen.insert(key.clone());
                return Some(item);
            }
        }
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect(); // Пропускаем первый аргумент (имя программы)
    if args.is_empty() {
        eprintln!("Пожалуйста, введите хотя бы один элемент.");
        return;
    }

    let ignore_case = true; // Установите значение в зависимости от ваших потребностей
    let unique = Unique::new(args.into_iter(), ignore_case);

    for item in unique {
        println!("{}", item);
    }
}