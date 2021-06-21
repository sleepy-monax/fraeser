use crate::{cutter::Cutter, units::*};

pub enum Action {
    Begin,
    End,

    MoveTo { pos: Position<Millimeters> },
    LineTo { pos: Position<Millimeters> },
}

pub struct Job {
    pub actions: Vec<Action>,
}

impl Job {
    pub fn new(path: &str) -> Job {
        let mut job = Job {
            actions: Vec::new(),
        };

        let polys = svg2polylines::parse_path(path, 0.01).unwrap();

        job.actions.push(Action::Begin);

        for poly in polys {
            let start = poly[0];

            job.actions.push(Action::MoveTo {
                pos: Position {
                    x: Millimeters(start.x),
                    y: Millimeters(start.y),
                },
            });

            for line in &poly[1..] {
                job.actions.push(Action::LineTo {
                    pos: Position {
                        x: Millimeters(line.x),
                        y: Millimeters(line.y),
                    },
                });
            }
        }

        job.actions.push(Action::End);

        return job;
    }

    pub fn run(&self, cutter: &mut dyn Cutter) {
        for act in &self.actions {
            match act {
                Action::Begin => {
                    println!("[BEGIN]");

                    cutter.begin();
                }
                Action::MoveTo { pos } => {
                    println!("[MOVE_TO] {:?}", pos);

                    cutter.move_to(*pos)
                }
                Action::LineTo { pos } => {
                    println!("[LINE_TO] {:?}", pos);

                    cutter.line_to(*pos)
                }
                Action::End => {
                    println!("[END]");

                    cutter.end();
                }
            }
        }
    }
}
