use std::fmt::{self, Display};
#[doc(inline)]
use crate::types::pointname::PointName;
use crate::error::SurvexError;

/// SurvexPoint representation
/// 
/// # Example
/// 
/// ```
/// use libsurvex::{SurvexPoint, PointName};
/// let mut point = SurvexPoint::new();
/// 
/// ```

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

/// SurvexPoint implementation
impl SurvexPoint {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    /// Set from and to points
    ///
    /// * `f` - from point as [&str]
    /// * `t` - to point as [&str]
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
        if self.0.is_sign_negative() && self.1.is_sign_negative(){
            write!(f, "{} {}", self.0, self.1)
        } else {
            write!(f, "-{} -{}", self.0, self.1)
        }
    }
}
impl Display for SurvexPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = "\t";
        let SurvexPoint { from, to, heading, back_compass ,tape, depth } = self;
        let compass = (heading + back_compass) / 2.0;
        let output = format!("{} {} {} {} {}", from, to, tape, heading, depth);
        writeln!(f,"{}", output.replace(" ", sep))?;
        Ok(())
    }
}
/// [From<&str>] implementation for [SurvexPoint]
/// 
/// # Example
/// 
/// ```
/// use libsurvex::SurvexPoint;
/// let test = "A1\tA2\t1.0\t2.0\t3.0\t4.0";
/// let point = SurvexPoint::from(test);
/// println!("{}", point);
/// ```
impl From<&str> for SurvexPoint {
    fn from(s: &str) -> Self {
        let vec_point = s.split("\t").collect::<Vec<&str>>();
        Self {
            from: PointName::from(vec_point[0]),
            to: PointName::from(vec_point[1]),
            tape: Tape::from(vec_point[2]),
            heading: vec_point[3].parse::<f32>().unwrap(),
            back_compass: vec_point[4].parse::<f32>().unwrap(),
            depth: Depth::from((vec_point[5],vec_point[6]))

        }
    }
}
impl From<&str> for Tape {
    fn from(s: &str) -> Self {
        Self(s.trim().parse::<f32>().unwrap())
    }
}
impl From<(&str,&str)> for Depth {
    fn from(s: (&str,&str)) -> Self {
        Self(s.0.trim().parse::<f32>().unwrap(), s.1.trim().parse::<f32>().unwrap())
    }
}