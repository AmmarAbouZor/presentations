use std::cell::Cell;
use std::path::Path;
use std::thread;
use std::time::Duration;

struct Proc {}

impl Proc {
    // Simulate work during creation (e.g., reading a config file)
    fn create(config_path: &Path) -> Self {
        std::hint::black_box(config_path);
        thread::sleep(Duration::from_millis(2)); // Simulate I/O
        println!("Creating Proc");
        Proc {}
    }

    // Simulate a CPU-intensive task
    fn process(&self, data: &[u8]) -> Vec<String> {
        std::hint::black_box(data);
        thread::sleep(Duration::from_millis(1)); // Simulate heavy computation
        vec!["item1".to_string(), "item2".to_string()]
    }
}

// Simulate a data source
fn get_data() -> Option<Vec<u8>> {
    thread_local! {
        static COUNTER: Cell<u32> = Cell::new(0);
    }

    let count = COUNTER.get();
    if count < 50 {
        COUNTER.set(count + 1);
        Some(vec![0; 1024])
    } else {
        None
    }
}

fn run(config_path: &Path) {
    let process = Proc::create(config_path);

    while let Some(data) = get_data() {
        let items = process.process(&data);
        std::hint::black_box(items);
    }
}

fn main() {
    let config_path = Path::new("config.config");
    run(config_path);
}
