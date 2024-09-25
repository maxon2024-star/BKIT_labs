extern crate rand;

use rand::Rng; // Не забудьте добавить rand в зависимости вашего Cargo.toml

struct GenRandom {
    count: usize,
    min: i32,
    max: i32,
    generated: usize,
}

impl GenRandom {
    fn new(count: usize, min: i32, max: i32) -> Self {
        GenRandom {
            count,
            min,
            max,
            generated: 0,
        }
    }
}

impl Iterator for GenRandom {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.generated < self.count {
            let mut rng = rng::thread_rng().gen_range(self.min..self.max);// Включает границы
            self.generated += 1;
            Some(rng)
        } else {
            None
        }
    }
}

fn gen_random(count: usize, min: i32, max: i32) -> GenRandom {
    GenRandom::new(count, min, max)
}

fn main() {
    let random_numbers: Vec<_> = gen_random(5, 1, 3).collect();
    for number in random_numbers {
        println!("{}", number);
    }
}