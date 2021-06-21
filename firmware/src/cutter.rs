use crate::units::*;

#[derive(PartialEq, Copy, Clone)]
pub enum MoveResult {
    Done,
    Stopped,
}

pub trait Cutter {
    fn begin(&mut self);
    fn end(&mut self);

    fn move_to(&mut self, pos: Position<Millimeters>) -> MoveResult;
    fn line_to(&mut self, pos: Position<Millimeters>) -> MoveResult;
}
