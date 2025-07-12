fn main() {
    println!("Hello, world!");
}

fn run_sequential() {
    while let Some(data) = load() {
        let results = process(&data);
        write_results(results);
    }
}

fn load() -> Option<Vec<u8>> {
    todo!()
}

fn process(data: &[u8]) -> String {
    todo!()
}

fn write_results(results: String) {}

use std::sync::mpsc;
use std::thread;

fn run_with_channels() {
    let (load_tx, load_rx) = mpsc::channel::<Vec<u8>>();
    let (process_tx, process_rx) = mpsc::channel::<String>();

    let process_hanlde = thread::spawn(move || {
        while let Ok(data) = load_rx.recv() {
            let res = process(&data);
            process_tx.send(res).unwrap();
        }
    });

    let write_hanlde = thread::spawn(move || {
        while let Ok(results) = process_rx.recv() {
            write_results(results);
        }
    });

    while let Some(data) = load() {
        load_tx.send(data).unwrap();
    }

    process_hanlde.join().unwrap();
    write_hanlde.join().unwrap();
}
