
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
    speed: u8,
    patience: u8,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, speed: u8, patience: u8, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            speed: speed,
            patience: patience,
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let mut _left;
        let mut _right;
        loop {
            _left = match table.forks[self.left].try_lock() {
                Ok(_left) => {
                    println!("{} берёт в левую {}", self.name, self.left);
                    _right = match table.forks[self.right].try_lock() {
                        Ok(_right) => {
                            println!("{} берёт в правую {}", self.name, self.right);
                            println!("{} начала есть", self.name);
                            thread::sleep(Duration::from_millis(1000 * (self.speed as u64)));
                            println!("{} закончила есть", self.name);
                            println!("{} кладёт {} и {}", self.name, self.left, self.right);
                            break;
                        },
                        Err(..) => {
                            println!("{} не смогла взять в правую {}", self.name, self.right);
                            thread::sleep(Duration::from_millis(1000 * (self.patience as u64)));
                            println!("{} кладёт {}", self.name, self.left);
                            drop(_left);
                            continue;
                        }
                    };
                },
                Err(..) => {
                    println!("{} не смогла взять в левую {}", self.name, self.left);
                    thread::sleep(Duration::from_millis(1000 * (self.patience as u64)));
                }
            };
        };
    }
}


struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    println!("Накрываем на стол");
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    println!("Усаживаем философов");
    let philosophers = vec![
        Philosopher::new("Джудит Батлер",    0, 5, 0, 1),
        Philosopher::new("Рая Дунаевская",   1, 2, 1, 2),
        Philosopher::new("Зарубина Наталья", 2, 1, 2, 3),
        Philosopher::new("Эмма Гольдман",    0, 3, 3, 4),
        Philosopher::new("Анна Шмидт",       3, 2, 4, 0),
    ];

    println!("Начинаем обед");
    let handles: Vec<_> = philosophers.into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    println!("Обед окончен");
}