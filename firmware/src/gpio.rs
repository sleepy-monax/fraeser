use crate::{cutter::*, units::*};
use rppal::gpio::*;
use std::thread::sleep;
use std::time::Duration;

pub struct GPIOPinLayout {
    pub x_ena_pin: u8,
    pub x_dir_pin: u8,
    pub x_pul_pin: u8,

    pub y_ena_pin: u8,
    pub y_dir_pin: u8,
    pub y_pul_pin: u8,
}

pub struct GPIOAxis {
    pub pos: Steps,
    pub millimeters_per_steps: Millimeters,
    pub dir: Dir,

    pub ena_pin: OutputPin,
    pub dir_pin: OutputPin,
    pub pul_pin: OutputPin,
}

impl GPIOAxis {
    pub fn enabled(&mut self, state: bool) {
        if state {
            self.ena_pin.set_low();
        } else {
            self.ena_pin.set_high();
        }
    }

    pub fn dir(&mut self, dir: Dir) {
        if dir != self.dir {
            match dir {
                Dir::Minus => self.dir_pin.set_high(),
                Dir::Plus => self.dir_pin.set_low(),
            }

            self.dir = dir;
        }
    }

    pub fn step(&mut self, dir: Dir) {
        self.dir(dir);
        self.pul_pin.set_low();
        sleep(Duration::from_secs_f64(0.0005));
        self.pul_pin.set_high();
        sleep(Duration::from_secs_f64(0.0005));

        self.pos += match dir {
            Dir::Minus => Steps(-1),
            Dir::Plus => Steps(1),
        };
    }
}

pub struct GPIOCutter {
    pub x_axis: GPIOAxis,
    pub y_axis: GPIOAxis,
}

impl GPIOCutter {
    pub fn new(
        GPIOPinLayout {
            x_ena_pin,
            x_dir_pin,
            x_pul_pin,
            y_ena_pin,
            y_dir_pin,
            y_pul_pin,
        }: GPIOPinLayout,
    ) -> rppal::gpio::Result<GPIOCutter> {
        let gpio = Gpio::new()?;

        return Ok(GPIOCutter {
            x_axis: GPIOAxis {
                pos: Steps(0),
                millimeters_per_steps: Millimeters(1.0), // 1 step = 1 milimiter
                dir: Dir::Minus,
                ena_pin: gpio.get(x_ena_pin)?.into_output(),
                dir_pin: gpio.get(x_dir_pin)?.into_output(),
                pul_pin: gpio.get(x_pul_pin)?.into_output(),
            },
            y_axis: GPIOAxis {
                pos: Steps(0),
                millimeters_per_steps: Millimeters(1.0), // 1 step = 1 milimiter
                dir: Dir::Minus,
                ena_pin: gpio.get(y_ena_pin)?.into_output(),
                dir_pin: gpio.get(y_dir_pin)?.into_output(),
                pul_pin: gpio.get(y_pul_pin)?.into_output(),
            },
        });
    }
}

impl Cutter for GPIOCutter {
    fn begin(&mut self) {
        self.x_axis.enabled(true);
        self.y_axis.enabled(true);
    }

    fn end(&mut self) {
        self.x_axis.enabled(false);
        self.y_axis.enabled(false);
    }

    fn move_to(&mut self, pos: Position<Millimeters>) {
        let final_x = pos.x.to_steps(self.x_axis.millimeters_per_steps);
        let final_y = pos.y.to_steps(self.x_axis.millimeters_per_steps);

        while self.x_axis.pos != final_x || self.y_axis.pos != final_y {
            if self.x_axis.pos != final_x {
                let x_dir = if (final_x - self.x_axis.pos) > Steps(0) {
                    Dir::Plus
                } else {
                    Dir::Minus
                };

                self.x_axis.step(x_dir);
            }

            if self.y_axis.pos != final_y {
                let y_dir = if (final_y - self.y_axis.pos) > Steps(0) {
                    Dir::Plus
                } else {
                    Dir::Minus
                };

                self.y_axis.step(y_dir);
            }
        }
    }

    fn line_to(&mut self, pos: Position<Millimeters>) {
        // TODO enabled cutter
        self.move_to(pos);
        // TODO disable cutter
    }
}
