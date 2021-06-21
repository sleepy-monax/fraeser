mod cutter;
mod gpio;
mod job;
mod turtle;
mod units;

use std::error::Error;
use std::io::Read;

use crate::gpio::*;
use crate::job::*;
use crate::turtle::*;

fn read_file_to_string(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    return s;
}

fn main() -> Result<(), Box<dyn Error>> {
    //let mut turtle_cutter = TurtleCutter::new();

    let mut gpio_cutter = GPIOCutter::new(GPIOPinLayout {
        x_ena_pin: 21,
        x_dir_pin: 20,
        x_pul_pin: 16,
        x_stp_pin: 12,

        y_ena_pin: 13,
        y_dir_pin: 19,
        y_pul_pin: 26,
        y_stp_pin: 5,
    })?;

    let path = read_file_to_string("samples/cat.path");
    let job = Job::new(path.as_str());
    job.run(&mut gpio_cutter);

    return Ok(());
}
