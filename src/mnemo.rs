#[cfg(feature="mnemo")]
extern crate libmnemo as mnemo;

use libmnemo::types::{NemoFile, NemoPoint};
use crate::{types::ABC, PointName, SurvexFile, SurvexPoint, SurvexProject};
use std::rc::Rc;

pub trait FromNemo<T, U> {
    fn from_nemo(input: &T) -> U;
}

#[cfg(feature="mnemo")]
impl FromNemo<mnemo::NemoReader, SurvexProject> for SurvexProject {
    fn from_nemo(nemo: &mnemo::NemoReader) -> SurvexProject {
        let mut vec = vec![];
        for (i, file) in nemo.files.iter().enumerate() {
            let c = ABC.chars().nth(i).unwrap();
            
            let f = translate_file(file, c);
            vec.push(Rc::new(f));
        }
        SurvexProject {
            files: vec, ..Default::default()
        }
    }
}
fn translate_file(nmf: &NemoFile, c: char) -> SurvexFile {
    let mut vec = vec![];
    for (i,point) in nmf.points.iter().enumerate() {
        vec.push(tranlate_points(point, c, i+1));
    }
    SurvexFile {
         point_data: vec, ..Default::default()
    }
}
fn tranlate_points (point: &NemoPoint, c: char, idx: usize) -> Rc<SurvexPoint> {
    let mut from = PointName (c,idx);
    let to = from.inc();
    Rc::new(SurvexPoint {
        from,
        to,
        heading: point.heading._in,
        back_compass: point.heading._out,
        tape: crate::Tape(point.length),
        depth: crate::Depth(point.depth._in,point.depth._out)
    })
}