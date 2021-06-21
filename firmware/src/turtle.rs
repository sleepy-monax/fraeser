use turtle::{Point, Turtle};

use crate::{cutter::*, units::*};

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

    fn move_to(&mut self, pos: Position<Millimeters>) -> MoveResult {
        self.turtle.pen_up();
        self.turtle.go_to(Point {
            x: pos.x.raw_value() as f64,
            y: -pos.y.raw_value() as f64,
        });

        return MoveResult::Done;
    }

    fn line_to(&mut self, pos: Position<Millimeters>) -> MoveResult {
        self.turtle.pen_down();
        self.turtle.go_to(Point {
            x: pos.x.raw_value() as f64,
            y: -pos.y.raw_value() as f64,
        });

        return MoveResult::Done;
    }
}
