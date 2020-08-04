extern crate rodio;
extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use rodio::Sink;
use rand::Rng;


// reads an mp3 file and plays it
fn notify() {
    let device = rodio::default_output_device().unwrap();

    let file = File::open("dn.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    let sink = Sink::new(&device);
    sink.set_volume(1.0);

    sink.append(source);
    sink.sleep_until_end();
}

// just sleeps for that many seconds
fn wait(seconds: u64) {
    sleep(Duration::from_secs(seconds));
}

fn main() {
    let mut rng = rand::thread_rng();

    // chime 3 times
    notify();
    notify();
    notify();

    // then start the loop
    loop {
        // random time between 17 and 53 minutes
        let wait_time = rng.gen_range(1020, 3180);
        wait(wait_time);
        // Chime again and start over
        notify();
    }
}
