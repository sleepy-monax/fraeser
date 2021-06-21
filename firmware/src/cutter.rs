use crate::units::*;

pub trait Cutter {
    fn begin(&mut self);
    fn end(&mut self);

    fn move_to(&mut self, pos: Position<Millimeters>);
    fn line_to(&mut self, pos: Position<Millimeters>);
}
