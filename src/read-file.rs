use std::{io::BufRead, time::Instant};

use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    x: i32,
    y: i32,
    z: bool,
}

fn main() {
    let start = Instant::now();
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();

    let mut xy_count = 0;
    let mut z_count = 0;
    for line in stdin_lock.lines() {
        let line = line.unwrap();
        let data: Data = serde_json::from_str(&line).unwrap();

        if data.x > 0 {
            if data.y % data.x == 0 {
                xy_count += 1;
            }
        }
        if data.z {
            z_count += 1
        }
    }

    println!("xy: {}", xy_count);
    println!("z: {}", z_count);
    println!("Elapsed: {:?}", start.elapsed());
}
