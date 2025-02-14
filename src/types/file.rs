use std::fmt::Display;

use crate::types::pointname::PointName;
use crate::types::point::SurvexPoint;
use crate::types::metadata::MetaData;
use crate::types::survexmeta::DEBUG_LINE;

#[derive(Debug, Default)]
pub struct SurvexFile {
    pub name: String,
    pub id: char,
    pub first_point: PointName,
    pub point_data: Vec<SurvexPoint>,
}

impl SurvexFile {
    pub fn new () -> Self {
        Self::default()
    }
    pub fn add_point(&mut self, point: SurvexPoint ) {
        self.point_data.push(point);
    }
    pub fn set_id(self, c: char) -> Self {
        Self {
            id: c, ..self
        }
    }
    pub fn first_point(self) -> PointName {
        self.point_data[0].from.clone()
    }
}
impl Display for SurvexFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filename = MetaData::new("Filename:",Some(&self.name)).comment();
        writeln!(f, "\n{}", filename)?;
        writeln!(f, "{}", DEBUG_LINE)?;
        for p in &self.point_data {
            write!(f, "{}", p)?;
        }
        Ok(())
    }
}