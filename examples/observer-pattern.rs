/// https://z0ltan.wordpress.com/2017/06/23/the-observer-pattern-in-rust/
extern crate rand;

trait Observable<T> {
    fn register(&mut self, observer: Box<Observer<Item = T>>);
}

trait Observer {
    type Item;

    fn notify(&self, val: &Self::Item);
}

struct EvenCounter {
    counter: u32,
    observers: Vec<Box<Observer<Item = u32>>>,
}

impl EvenCounter {
    fn new() -> Self {
        EvenCounter {
            counter: 0u32,
            observers: Vec::new(),
        }
    }

    fn run(&mut self) {
        loop {
            use std::thread;
            use std::time::Duration;

            thread::sleep(Duration::from_millis(self.get_rand_duration()));

            self.counter += 1;

            if self.counter % 2 == 0 {
                for observer in self.observers.iter() {
                    observer.notify(&self.counter);
                }
            }
        }
    }

    fn get_rand_duration(&self) -> u64 {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        rng.gen_range(0, 1000)
    }
}

impl Observable<u32> for EvenCounter {
    fn register(&mut self, observer: Box<Observer<Item = u32>>) {
        self.observers.push(observer);
    }
}

struct EvenObserver {
    name: String,
}

impl EvenObserver {
    fn new(name: String) -> Self {
        EvenObserver { name }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Observer for EvenObserver {
    type Item = u32;

    fn notify(&self, val: &Self::Item) {
        println!("'{}' got: {}", self.name(), val);
    }
}

fn main() {
    let mut foo = EvenCounter::new();
    let (bar, baz, quux) = (
        Box::new(EvenObserver::new("bar".to_string())),
        Box::new(EvenObserver::new("baz".to_string())),
        Box::new(EvenObserver::new("quux".to_string())),
    );

    foo.register(bar);
    foo.register(baz);
    foo.register(quux);

    foo.run();
}
