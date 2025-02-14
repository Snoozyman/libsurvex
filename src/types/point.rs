use std::fmt::{self, Display};
use crate::types::pointname::PointName;

#[derive(Default, Debug)]
pub struct SurvexPoint {
    pub from:   PointName,
    pub to:     PointName,
    pub heading: f32,
    pub back_compass: f32,
    pub tape:   Tape,
    pub depth:  Depth,
    
}

#[derive(Default, Debug)]
pub struct Depth(pub f32, pub f32);

#[derive(Default, Debug)]
pub struct Tape(pub f32);

impl SurvexPoint {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn set_from_to(&mut self, f: &str, t: &str ) {
        let from = PointName::from(f);
        let to = PointName::from(t);
        self.from = from;
        self.to = to;
    }
    pub fn set_depth(&mut self, f: f32, t: f32) {
        self.depth = Depth(f,t);
    }
    pub fn set_tape(&mut self, t: f32) {
        self.tape = Tape(t);
    }
}
impl Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",self.0)
    }
}
impl Display for Depth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
impl Display for SurvexPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = "\t";
        let SurvexPoint { from, to, heading, back_compass ,tape, depth } = self;
        let output = format!("{} {} {} {} {}", from, to, tape, heading, depth);
        writeln!(f,"{}", output.replace(" ", sep))?;
        Ok(())
    }
}