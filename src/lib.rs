use std::path::PathBuf;
use std::io::Write;

use types::metadata::MetaData;

pub use crate::types::project::SurvexProject;
pub use crate::types::survexmeta::SurvexMeta;
use crate::types::metadata::Survex;
#[cfg(feature="mnemo")]
pub mod mnemo;
mod types;

pub trait TrimAndLower {
    fn replace_whitespace(&self, to: char) -> String;
    fn lower(&self) -> String;
}
impl TrimAndLower for String {
    fn lower(&self) -> String {
        self.to_string().to_lowercase()
    }
    fn replace_whitespace(&self, to: char) -> String {
        self.to_string().replace(' ', to.to_string().as_str())
    }
}


pub fn init_dir(path: &PathBuf) {
    if !path.exists(){
        std::fs::create_dir_all(path).unwrap();
    }else {
        println!("Path already exists");
    }
}
pub fn write_project(project: &SurvexProject, path: &PathBuf) {
    let path = PathBuf::from(&path);
    let mut file = std::fs::File::create(path).unwrap();
    let _ = writeln!(file, "{}", project);
}
pub fn print_project(project: &SurvexProject) {
    println!("{}", project);
}
pub fn get_project_as_string(project: &SurvexProject) -> String {
    format!("{}", project)
}