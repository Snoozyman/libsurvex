#[cfg(feature="mnemo")]
extern crate libmnemo as mnemo;
#[cfg(feature="mnemo")]
use libmnemo::types::{NemoFile, NemoPoint, NemoTypeShot};

use crate::types::project::SurvexProject;
use crate::types::file::SurvexFile;
use crate::types::ABC;
use crate::types::point::{SurvexPoint, Tape, Depth};
use crate::types::pointname::PointName;

use std::{rc::Rc, vec};


#[cfg(feature="mnemo")]
impl From<&mnemo::NemoReader> for SurvexProject {
    fn from(nemo: &mnemo::NemoReader) -> SurvexProject {
        let mut vec: Vec<Rc<SurvexFile>> = vec![];
        for (i, file) in nemo.files.iter().enumerate() {
            let c = ABC.chars().nth(i).unwrap();
            vec.push(Rc::new(translate_file(file, c)));
        }
        SurvexProject {
            files: vec, ..Default::default()
        }
    }
}
fn translate_file(nmf: &NemoFile, c:char ) -> SurvexFile {
    let mut vec = vec![];
    let mut first_point = PointName::default();
    for (i,point) in nmf.points.iter().enumerate() {
        if i == 0 { first_point.0 = c; first_point.1 = i + 1; }
        if point.typeshot != NemoTypeShot::EOC {
            vec.push(translate_point(point, c, i+1));
        }
    }
    SurvexFile {
        name: nmf.filename.clone(), id: c, point_data: vec, first_point
        
    }
}
fn translate_point(point: &NemoPoint, c: char, idx: usize) -> SurvexPoint {
    let mut from = PointName (c,idx);
    let to = from.inc();
    SurvexPoint {
        from,
        to,
        heading: point.heading._in,
        back_compass: point.heading._out,
        tape: Tape(point.length),
        depth: Depth(point.depth._in,point.depth._out)
    }
}
