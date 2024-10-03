use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io::{self, Write};

// Реализация аналогичная cm_timer_1
fn cm_timer_1<F>(block: F)
where
    F: FnOnce(),
{
    let start = Instant::now();
    block();
    let duration = start.elapsed();
    println!("time1: {:.2?}", duration.as_secs_f64());
}

// Реализация аналогичная cm_timer_2
struct CmTimer2 {
    start: Instant,
}

impl CmTimer2 {
    fn new() -> CmTimer2 {
        CmTimer2 {
            start: Instant::now(),
        }
    }

    fn stop(self) {
        let duration = self.start.elapsed();
        println!("time2: {:.2?}", duration.as_secs_f64());
    }
}

// Функция для получения времени ожидания от пользователя
fn get_sleep_duration_from_input() -> Duration {
    print!("Введите время в секундах: ");
    io::stdout().flush().unwrap(); // Очищаем буфер вывода для корректного вывода

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: f64 = input.trim().parse().expect("Введите число!");

    Duration::from_secs_f64(input)
}

fn main() {
    // Получаем время ожидания от пользователя
    let sleep_duration = get_sleep_duration_from_input();

    // Пример использования cm_timer_1
    cm_timer_1(|| {
        sleep(sleep_duration);
    });

    // Пример использования cm_timer_2
    let timer = CmTimer2::new();
    sleep(sleep_duration);
    timer.stop();
}