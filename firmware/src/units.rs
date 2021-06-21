use derive_more::Add;
use derive_more::AddAssign;
use derive_more::Sub;
use derive_more::SubAssign;

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Add, AddAssign, Sub, SubAssign)]
pub struct Steps(pub i64);
impl Steps {
    pub fn raw_value(self) -> i64 {
        return self.0;
    }

    pub fn to_millimeters(self, millimeters_per_steps: Millimeters) -> Millimeters {
        return Millimeters(self.0 as f64 * millimeters_per_steps.0);
    }
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Add, AddAssign, Sub, SubAssign)]
pub struct Millimeters(pub f64);
impl Millimeters {
    pub fn raw_value(self) -> f64 {
        return self.0;
    }

    pub fn to_steps(self, millimeters_per_steps: Millimeters) -> Steps {
        return Steps((self.0 / millimeters_per_steps.0) as i64);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position<U> {
    pub x: U,
    pub y: U,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Dir {
    Minus,
    Plus,
}
