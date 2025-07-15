use std::{hint::black_box, path::PathBuf};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("We must have one arguments");
        return;
    }

    let path = PathBuf::from(&args[1]);

    let nums: Vec<_> = std::fs::read_to_string(&path)
        .unwrap()
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    black_box(&nums);

    let time = std::time::Instant::now();

    let count: Vec<_> = nums.iter().filter(|&&num| num > 50000).collect();

    black_box(&count);

    let elapsed = time.elapsed();

    println!("Count is {}", count.len());
    println!("Running took {} milli", elapsed.as_micros())
}
