use std::io::Write;

use rand::Rng;
use serde::Serialize;

#[derive(Serialize)]
struct Data {
    x: u8,
    y: i32,
    z: bool,
}

fn main() {
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();

    let size = std::env::args()
        .nth(1)
        .map(|x| x.parse().expect("First argument must be a number"))
        .unwrap_or(30000000);

    let mut rng = rand::thread_rng();

    for _ in 0..size {
        let d = Data {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        };

        serde_json::to_writer(&mut stdout_lock, &d).unwrap();
        stdout_lock.write_all(b"\n").unwrap();
    }

    stdout_lock.flush().unwrap();
}
