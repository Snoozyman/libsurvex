
#[cfg(feature="mnemo")]
pub mod mnemo;
pub mod types;

use std::io::{Error, Write};
use types::*;


#[allow(dead_code)]
macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident),*
    }) => {
        pub enum $name {
            $($variant),*
        }
        #[allow(dead_code)]
        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        enum $name {
            $($variant = $val),*
        }
        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}
pub enum SurvexTokens {
    DCOM,
    STAR
}

enum_str!{ 
    enum SurvexKeyword {
        Include,
        Export,
        Fix,
        End,
        Calibrate,
        Data,
        Undefined
}
}

pub fn new () {
    let mut project = SurvexProject::new();
    let file = SurvexFile::default();
    project.add_file(file);
    let file2 = SurvexFile::default();
    project.add_file(file2);
    let files = project.get_files();
    dbg!(files);
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct SurvexWriter {
    path: String,
    project: SurvexProject,
}
#[allow(dead_code)]
impl SurvexWriter {
    fn new () -> Self {
        Self {
            path: "".into(),
            project: SurvexProject::default(),
        }
    }
    fn set_path (&mut self, path: String) {
        self.path = path;
    }
    pub fn write_to_path(&self, path: String) -> Result<(), Error> {
        let mut s = std::fs::File::create(path)?;
        let mut headers = String::new();
        headers += ";from to tape compass fromdepth todepth\n";
        headers += "*begin Test\n";
        headers += "*copyright Mikko Tuomikoski\n";
        headers += "*data diving from to tape compass fromdepth todepth ignoreall\n";


        let footer = "*end Test";
        writeln!(s, "{}", headers)?;
        for file in &self.project.files {
            for point in &file.point_data {
                // println!("{}", point.as_string());
                writeln!(s,"{}", point.as_string())?;
            }
        }
        writeln!(s, "{}", footer)?;
        Ok(())
    }
}
impl From<SurvexProject> for SurvexWriter {
    fn from(value: SurvexProject) -> Self {
        Self {
            path: "".into(),
            project: value
        }
    }
}