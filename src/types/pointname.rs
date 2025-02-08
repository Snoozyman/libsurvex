use std::fmt::{Display, Write};
use super::{AsStr, ABC};
use std::str::FromStr;


#[derive(Default,Clone, Eq, PartialEq, Hash, Debug)]
pub struct PointName (pub char, pub usize);

impl From<&str> for PointName {
    fn from(value: &str) -> Self {
        let temp = value.split_at(1);
        let c = temp.0.parse::<char>().expect("Can't parse to char");
        let id = temp.1.parse::<usize>().expect("Can't parse to usize");
        Self (c, id)
    }
}
impl From<String> for PointName {
    fn from(value: String) -> Self {
        PointName::from(value.as_str())
    }
}
impl Display for PointName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}
impl PointName {
    /// Creates a new clone and increments it by one
    pub fn inc (&mut self) -> Self {
        Self ('A', self.1 + 1)
    }
}
impl AsStr for PointName {
    fn as_str(&self) -> String {
        format!("{}", self)
    }
}