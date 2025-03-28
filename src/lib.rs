/*!

A library for interacting with Survex files

# Overview


*/ 
use std::path::PathBuf;
use std::io::Write;

#[doc(inline)]
pub use crate::{
    types::{
        file::SurvexFile,
        metadata::MetaData,
        point::SurvexPoint,
        pointname::PointName,
        project::SurvexOptions,
        project::SurvexProject,
        survexmeta::SurvexMeta,
        survexmeta::Sep,
    },
    error::{SurvexError, ErrorKind},
    utils::TrimAndLower,
};

#[cfg(feature="mnemo")]
pub mod mnemo;
pub mod types;
mod utils;
mod error;
/// Writes project to file
/// 
/// * `project` - [SurvexProject]
/// * `path` - [str]
/// 
/// # Example
/// 
/// ```
/// use libsurvex::{SurvexProject, write_project};
/// let project = SurvexProject::new();
/// //write_project(&project, "project.svx");
/// 
/// ```
pub fn write_project(project: &SurvexProject, path: &str) {
    let path = PathBuf::from(&path);
    let mut file = std::fs::File::create(path).unwrap();
    let _ = writeln!(file, "{}", project);
}
/// Prints project to stdout
/// 
/// * `project` - [SurvexProject]
/// 
/// # Example
/// 
/// ```
/// use libsurvex::{SurvexProject, print_project};
/// let project = SurvexProject::new();
/// print_project(&project);
/// ```
pub fn print_project(project: &SurvexProject) {
    println!("{}", project);
}
/// Returns project as string
///
/// * `project` - [SurvexProject]
/// 
/// # Example
/// 
/// ```
/// use libsurvex::{SurvexProject, get_project_as_string};
/// let project = SurvexProject::new();
/// let project_as_string = get_project_as_string(&project);
/// ```
pub fn get_project_as_string(project: &SurvexProject) -> String {
    format!("{}", project)
}