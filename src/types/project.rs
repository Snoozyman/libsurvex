use std::{fmt::Display, rc::Rc};

use crate::types::file::SurvexFile;
use crate::types::metadata::MetaData;
use crate::types::survexmeta::SurvexMeta;

use super::metadata::Survex;

#[derive(Default, Debug)]
pub struct SurvexProject {
    pub header: SurvexMeta,
    pub name: String,
    pub ext: String,
    pub author: String,
    pub files: Vec<Rc<SurvexFile>>,
}

pub type SurvexOptions = Vec<MetaData<Survex>>;

impl SurvexProject {
    pub fn new() -> Self {
        Self { ext: ".svx".into(), ..Default::default() }
    }
    pub fn options(self, options: SurvexOptions) -> SurvexProject {
        let mut header = self.header;
        header.options = options;
        Self {
            header, ..self
        }
    }
    pub fn print(&self) {
        println!("Name: {}", self.name);
        println!("Author: {}", self.author);
        println!("Ext: {}", self.ext);
        for f in &self.files {
            println!("File:\t{:?}",f);
        }
    }
    pub fn name<U>(self, name: U) -> Self 
    where U: ToString,
    {
        Self {
            name: name.to_string(),
            header: self.header.name(name.to_string().as_str()),
             ..self
        }
    }
    pub fn author<U>(self, name: U) -> Self
    where U: ToString
    {
        Self {
            author: name.to_string(),
            header: self.header.author(name.to_string().as_str()),
            ..self
        }
    }
    pub fn fix(self, point: &str,lat: f64, lon: f64, elevation: f64){
        let meta = MetaData::new("fix", Some(
            format!("{} {} {} {}", point, lat, lon, elevation).as_str()
        ));
        self.add_options(meta);
    }
    pub fn entrance(self, name: &str){
        let meta = MetaData::new("entrance", Some(name));
        self.add_options(meta);
    }
    fn add_options(mut self, meta: MetaData<Survex>){
        self.header.options.push(meta);
    }
}
impl Display for SurvexProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let project_name = 
            MetaData::new("Project name:", Some(&self.name))
            .comment();
        write!(f, "{}", self.header)?;
        write!(f, "\n{}", project_name)?;
        for p in &self.files {
            write!(f, "{}", p)?;
        }
        write!(f, "{}", self.header.end)?;
        Ok(())
    }
}