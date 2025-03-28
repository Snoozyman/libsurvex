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

#[derive(Default, Debug, PartialEq)]
pub struct Depth(pub f32, pub f32);

#[derive(Default, Debug, PartialEq)]
pub struct Tape(pub f32);

/// SurvexPoint implementation
impl SurvexPoint {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    /// Set from and to points
    ///
    /// * `from` - from point as [&str]
    /// * `to` - to point as [&str]
    /// 
    /// # Example
    /// ```
    ///     use libsurvex::{SurvexPoint, PointName};
    ///     let mut point = SurvexPoint::new();
    ///     point.set_from_to("A1", "A2");
    ///     assert_eq!(point.from, PointName::from("A1"));
    ///     assert_eq!(point.to, PointName::from("A2"));
    /// 
    pub fn set_from_to(&mut self, from: &str, to: &str ) {
        let from = PointName::from(from);
        let to = PointName::from(to);
        self.from = from;
        self.to = to;
    }
    /// Set depth
    /// 
    /// * `from` - from depth as [f32]
    /// * `to` - to depth as [f32]
    /// 
    /// # Example
    /// ```
    ///     use libsurvex::{SurvexPoint, PointName};
    ///     use libsurvex::types::point::Depth;
    ///     let mut point = SurvexPoint::new();
    ///     point.set_depth(1.0, 2.0);
    ///     assert_eq!(point.depth, Depth(1.0,2.0));
    /// ```
    pub fn set_depth(&mut self, from: f32, to: f32) {
        self.depth = Depth(from,to);
    }
    /// Set tape
    /// 
    /// * `tape` - tape as [f32]
    /// 
    /// # Example
    /// ```
    ///     use libsurvex::{SurvexPoint, PointName};
    ///     use libsurvex::types::point::Tape;
    ///     let mut point = SurvexPoint::new();
    ///     point.set_tape(1.0);
    ///     assert_eq!(point.tape, Tape(1.0));
    /// ```
    pub fn set_tape(&mut self, tape: f32) {
        self.tape = Tape(tape);
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
impl From<&str> for SurvexPoint {
    /// [From<&str>] implementation for [SurvexPoint]
    /// 
    /// # Example
    /// 
    /// ```
    ///     use libsurvex::{SurvexPoint, PointName};
    ///     let test = "A1\tA2\t1.0\t2.0\t3.0\t4.0\t5.0";
    ///     let point = SurvexPoint::from(test);
    ///     let pointname = PointName::from("A1");
    ///     assert_eq!(point.from, pointname);
    /// ```
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