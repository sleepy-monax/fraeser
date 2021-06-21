use turtle::{Point, Turtle};

use crate::{cutter::Cutter, units::*};

pub struct TurtleCutter {
    turtle: Turtle,
}

impl TurtleCutter {
    pub fn new() -> TurtleCutter {
        return TurtleCutter {
            turtle: Turtle::new(),
        };
    }
}

impl Cutter for TurtleCutter {
    fn begin(&mut self) {}

    fn end(&mut self) {}

    fn move_to(&mut self, pos: Position<Millimeters>) {
        self.turtle.pen_up();
        self.turtle.go_to(Point {
            x: pos.x.raw_value() as f64,
            y: -pos.y.raw_value() as f64,
        });
    }

    fn line_to(&mut self, pos: Position<Millimeters>) {
        self.turtle.pen_down();
        self.turtle.go_to(Point {
            x: pos.x.raw_value() as f64,
            y: -pos.y.raw_value() as f64,
        });
    }
}
