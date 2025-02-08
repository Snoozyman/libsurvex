use super::SurvexPoint;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct SurvexFile {
    pub name: String,
    pub id: char,
    pub point_data: Vec<Rc<SurvexPoint>>,
}

impl SurvexFile {
    pub fn add_point(&mut self, point: SurvexPoint ) {
        self.point_data.push(Rc::new(point));
    }
    pub fn set_id(&mut self, c: char) {
        self.id = c;
    }
}
