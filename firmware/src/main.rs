use std::error::Error;
use std::io::Read;
use std::process;
use std::thread::sleep;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

type Step = i32;
type Millimeter = i32;

#[derive(PartialEq, Copy, Clone)]
enum Dir {
    Minus,
    Plus,
}

struct Axis {
    pos: Step,
    step: Millimeter,
    dir: Dir,

    ena_pin: OutputPin,
    dir_pin: OutputPin,
    pul_pin: OutputPin,
}

impl Axis {
    fn enabled(&mut self, state: bool) {
        if state {
            self.ena_pin.set_low();
        } else {
            self.ena_pin.set_high();
        }
    }

    fn dir(&mut self, dir: Dir) {
        if dir != self.dir {
            match dir {
                Dir::Minus => self.dir_pin.set_high(),
                Dir::Plus => self.dir_pin.set_low(),
            }

            self.dir = dir;
        }
    }

    fn step(&mut self, dir: Dir) {
        self.dir(dir);
        self.pul_pin.set_low();
        sleep(Duration::from_secs_f64(0.001));
        self.pul_pin.set_high();
        sleep(Duration::from_secs_f64(0.001));

        self.pos += match dir {
            Dir::Minus => -1,
            Dir::Plus => 1,
        };
    }
}

struct Cutter {
    x_axis: Axis,
}

impl Cutter {
    fn enabled(&mut self, state: bool) {
        self.x_axis.enabled(state);
    }

    fn move_to(&mut self, x: Step) {
        println!("[move_to] x:{}", x);

        while self.x_axis.pos != x {
            let x_dir = if (x - self.x_axis.pos) > 0 {
                Dir::Plus
            } else {
                Dir::Minus
            };

            self.x_axis.step(x_dir);
        }
    }

    fn line_to(&mut self, x: Step) {
        println!("[line to] x:{}", x);

        // TODO enabled cutter
        self.move_to(x);
        // TODO disable cutter
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;

    let mut cutter = Cutter {
        x_axis: Axis {
            pos: 1,
            step: 1, // 1 step = 1 milimiter
            dir: Dir::Minus,
            ena_pin: gpio.get(21)?.into_output(),
            dir_pin: gpio.get(20)?.into_output(),
            pul_pin: gpio.get(16)?.into_output(),
        },
    };

    cutter.enabled(true);

    let mut file = std::fs::File::open("cat.svg").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let polys = svg2polylines::parse(&mut s, 0.1)?;

    for poly in polys {
        let start = poly[0];

        cutter.move_to(start.x as i32);

        for line in &poly[1..] {
            cutter.line_to((line.x * 100.0) as i32);
        }
    }

    cutter.enabled(false);

    process::exit(0);
}
