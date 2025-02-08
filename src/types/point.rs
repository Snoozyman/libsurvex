use std::{fmt::{self, Display}, str::FromStr};
use super::PointName;

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
    pub fn as_string(&self) -> String {
        let SurvexPoint { from, to, heading, back_compass ,tape, depth } = self;
        let Depth (fromd, tod) = depth;
        let compass = (heading + back_compass)/2.0;
        format!("{} {} {} {} {} {}", from, to, tape, heading, fromd, tod)
    }
}
impl Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",self.0)
    }
}
impl Display for Depth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}", self.0, self.1)
    }
}