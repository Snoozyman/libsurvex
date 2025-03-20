use std::{ fmt::Display};
use crate::utils::TrimAndLower;

#[derive(Debug, Default, Copy, Clone)]
pub enum Survex {
    COMMENT,
    #[default]
    HEADER,
}
#[derive(Debug)]
pub struct MetaData<Survex>{
    pub _begin: Survex,
    pub data: String,
    arg: Option<String>,
}


impl MetaData<Survex> {
    pub fn new<'s, U>(data: U, arg: Option<U>) -> Self
    where  's: 'static,
            U: ToString,
    {
        let opt_arg: Option<String> = arg.map(|a| a.to_string());
        Self {
            _begin: Survex::default(), data: data.to_string(), arg: opt_arg
        }   
    }
    pub fn comment(self) -> Self {
        Self {
            _begin: Survex::COMMENT, ..self
        }
    }
    pub fn change<'s, U>(self, value: U) -> Self
    where  's: 'static,
            U: ToString + ToOwned,
        {
        Self {
            arg: Some(value.to_string()), ..self
        }
    }
}
impl Display for Survex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.as_char())?;
        Ok(())
    }
}
impl Survex {
    pub fn as_char(self) -> char {
        match self {
            Self::COMMENT => ';',
            Self::HEADER => '*',
        }
    }
}
impl Display for MetaData<Survex> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.arg.is_some() {
            let opt_arg = self.arg.as_ref().unwrap().to_owned();
            writeln!(f, "{}{} {}", self._begin, self.data, opt_arg)?;
        }else {
            writeln!(f, "{}{}", self._begin, self.data)?;
        }
        Ok(())
    }
}
impl TrimAndLower for MetaData<Survex> {
    fn lower(&self) -> String {
        self.data.to_string().to_lowercase()
    }
    fn replace_whitespace(&self, to: char) -> String {
        self.data.to_string().replace(' ', to.to_string().as_str())
    }
}