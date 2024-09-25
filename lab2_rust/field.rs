use std::collections::HashMap;

struct Field<'a> {
    goods: &'a [HashMap<&'a str, Option<String>>],
    keys: Vec<&'a str>,
    index: usize,
}

impl<'a> Field<'a> {
    fn new(goods: &'a [HashMap<&'a str, Option<String>>], keys: Vec<&'a str>) -> Self {
        Field { goods, keys, index: 0 }
    }
}

impl<'a> Iterator for Field<'a> {
    type Item = HashMap<&'a str, String>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.goods.len() {
            let item = &self.goods[self.index];
            let mut result = HashMap::new();
            let mut has_values = false;

            for key in &self.keys {
                if let Some(value) = item.get(*key).and_then(|v| v.as_ref()) {
                    result.insert(*key, value.clone());
                    has_values = true;
                }
            }

            self.index += 1;

            if has_values {
                return Some(result);
            }
        }
        None
    }
}

fn field<'a>(goods: &'a [HashMap<&'a str, Option<String>>], keys: Vec<&'a str>) -> Field<'a> {
    Field::new(goods, keys)
}

fn main() {
    let goods = vec![
        HashMap::from([
            ("title", Some("Ковер".to_string())),
            ("price", Some("2000".to_string())),
            ("color", Some("green".to_string())),
        ]),
        HashMap::from([
            ("title", Some("Диван для отдыха".to_string())),
            ("color", Some("black".to_string())),
        ]),
    ];

    // Пример использования: получение значений одного поля
    let titles: Vec<_> = field(&goods, vec!["title"]).collect();
    for title in titles {
        println!("{}", title.get("title").unwrap());
    }

    // Пример использования: получение значений нескольких полей
    let items: Vec<_> = field(&goods, vec!["title", "price"]).collect();
    for item in items {
        println!("{:?}", item);
    }
}