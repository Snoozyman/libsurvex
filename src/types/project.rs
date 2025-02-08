use std::rc::Rc;
use super::SurvexFile;

type VecRcSF = Vec<Rc<SurvexFile>>;

#[derive(Default, Debug)]
pub struct SurvexProject {
    pub name: String,
    pub ext: String,
    pub author: String,
    pub files: Vec<Rc<SurvexFile>>,
}

impl SurvexProject {
    pub fn new() -> Self {
        Self { ext: ".svx".into(), ..Default::default() }
    }
    pub fn add_file(&mut self, f: SurvexFile)
    {
       let b: Rc<SurvexFile> = Rc::new(f);
       self.files.push(b);
    }
    pub fn get_files(&self) -> &VecRcSF {
        self.files.as_ref()
    }
    pub fn set_name (&mut self, name: String) {
        self.name = name;
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
}
