mod file;
mod point;
mod pointname;
mod project;

pub use self::file::*;
pub use self::point::*;
pub use self::pointname::*;
pub use self::project::*;
pub const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub trait AsStr {
    fn as_str(&self) -> String;
}