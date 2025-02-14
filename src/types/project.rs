use std::{fmt::Display, rc::Rc};

use crate::types::file::SurvexFile;
use crate::types::metadata::MetaData;
use crate::SurvexMeta;
use crate::TrimAndLower;

#[derive(Default, Debug)]
pub struct SurvexProject {
    pub header: SurvexMeta,
    pub name: String,
    pub ext: String,
    pub author: String,
    pub files: Vec<Rc<SurvexFile>>,
}

impl SurvexProject {
    pub fn new() -> Self {
        Self { ext: ".svx".into(), ..Default::default() }
    }
    fn add_file(&mut self, f: SurvexFile)
    {
       let b: Rc<SurvexFile> = Rc::new(f);
       self.files.push(b);
    }
    fn get_files(&self) -> &Vec<Rc<SurvexFile>> {
        self.files.as_ref()
    }
    pub fn set_author(&mut self, author: String) {
        self.author = author;
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