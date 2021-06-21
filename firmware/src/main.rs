pub mod app;

use std::error::Error;
use std::io::Read;
use std::process;
use std::thread::sleep;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use turtle::{Point, Turtle};

type Step = i32;
type Millimeter = i32;

#[derive(PartialEq, Copy, Clone)]
enum Dir {
    Minus,
    Plus,
}

struct Axis {
    pos: Step,
    step_size: Millimeter,
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
        sleep(Duration::from_secs_f64(0.0005));
        self.pul_pin.set_high();
        sleep(Duration::from_secs_f64(0.0005));

        self.pos += match dir {
            Dir::Minus => -1,
            Dir::Plus => 1,
        };
    }
}

trait Cutter {
    fn enabled(&mut self, state: bool);
    fn move_to(&mut self, x: Step, y: Step);
    fn line_to(&mut self, x: Step, y: Step);
}

struct TurtleCutter {
    turtle: Turtle,
}

impl Cutter for TurtleCutter {
    fn enabled(&mut self, state: bool) {}

    fn move_to(&mut self, x: Step, y: Step) {
        self.turtle.pen_up();
        self.turtle.go_to(Point {
            x: x as f64,
            y: -y as f64,
        });
    }

    fn line_to(&mut self, x: Step, y: Step) {
        self.turtle.pen_down();
        self.turtle.go_to(Point {
            x: x as f64,
            y: -y as f64,
        });
    }
}

struct PlasmaCutter {
    x_axis: Axis,
    y_axis: Axis,
}

impl Cutter for PlasmaCutter {
    fn enabled(&mut self, state: bool) {
        self.x_axis.enabled(state);
        self.y_axis.enabled(state);
    }

    fn move_to(&mut self, x: Step, y: Step) {
        println!("[move_to] x:{} y:{}", x, y);

        while self.x_axis.pos != x || self.y_axis.pos != y {
            if self.x_axis.pos != x {
                let x_dir = if (x - self.x_axis.pos) > 0 {
                    Dir::Plus
                } else {
                    Dir::Minus
                };

                self.x_axis.step(x_dir);
            }

            if self.y_axis.pos != y {
                let y_dir = if (y - self.y_axis.pos) > 0 {
                    Dir::Plus
                } else {
                    Dir::Minus
                };

                self.y_axis.step(y_dir);
            }
        }
    }

    fn line_to(&mut self, x: Step, y: Step) {
        println!("[line to] x:{} y:{}", x, y);

        // TODO enabled cutter
        self.move_to(x, y);
        // TODO disable cutter
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut turtle_cutter = TurtleCutter {
        turtle: Turtle::new(),
    };

    /*
    let gpio = Gpio::new()?;

    let mut plasma_cutter = PlasmaCutter {
        x_axis: Axis {
            pos: 0,
            step_size: 1, // 1 step = 1 milimiter
            dir: Dir::Minus,
            ena_pin: gpio.get(21)?.into_output(),
            dir_pin: gpio.get(20)?.into_output(),
            pul_pin: gpio.get(16)?.into_output(),
        },
        y_axis: Axis {
            pos: 0,
            step_size: 1, // 1 step = 1 milimiter
            dir: Dir::Minus,
            ena_pin: gpio.get(13)?.into_output(),
            dir_pin: gpio.get(19)?.into_output(),
            pul_pin: gpio.get(26)?.into_output(),
        },
    };
    */

    turtle_cutter.enabled(true);
    //plasma_cutter.enabled(true);

    let mut file = std::fs::File::open("cat.svg").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let polys = svg2polylines::parse(&mut s, 0.001)?;

    for poly in polys {
        let start = poly[0];

        turtle_cutter.move_to((start.x * 10.0) as i32, (start.y * 10.0) as i32);
        //plasma_cutter.move_to((start.x * 300.0) as i32, (start.y * 300.0) as i32);

        for line in &poly[1..] {
            turtle_cutter.line_to((line.x * 10.0) as i32, (line.y * 10.0) as i32);
            //plasma_cutter.line_to((line.x * 300.0) as i32, (line.y * 300.0) as i32);
        }
    }

    turtle_cutter.move_to(0, 0);
    // plasma_cutter.move_to(0, 0);

    turtle_cutter.enabled(false);
    // plasma_cutter.enabled(false);

    sleep(Duration::from_secs(10));
    process::exit(0);
}
