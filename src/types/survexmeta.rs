use std::fmt::Display;
use crate::{types::metadata::{MetaData, Survex}, SurvexOptions};
use std::default::Default;
use crate::utils::TrimAndLower;

lazy_static::lazy_static!{
    #[doc(hidden)]
    pub static ref DEBUG_LINE: MetaData<Survex> = MetaData::new("from to tape compass fromdepth todepth", None).comment();
}

#[derive(Debug, Default)]
pub enum Sep {
    #[default]
    Tab,
    Comma,
    WSpace,
}
impl DEBUG_LINE {
    fn to_meta(&self) -> MetaData<Survex> {
        lazy_static::initialize(self);
        MetaData::new(self.data.clone(), None).comment()       
    }
}
impl Display for DEBUG_LINE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self._begin {
            Survex::COMMENT=> write!(f, ";")?,
            Survex::HEADER => write!(f, "*")?,
        }
        write!(f, "{}", self.data.replace_whitespace('\t'))?;
        Ok(())
    }
}
impl Display for Sep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = match &self {
            Sep::Tab => '\t',
            Sep::Comma => ',',
            Sep::WSpace => ' ',
        };
        write!(f, "{}", sep)?;
        Ok(())
    }
}
/// SurvexMeta
#[derive(Debug)]
pub struct SurvexMeta {
    _separator: Sep,
    _debug_line: MetaData<Survex>,
    pub begin: MetaData<Survex>,
    pub copyright: MetaData<Survex>,
    pub data: MetaData<Survex>,
    pub end: MetaData<Survex>,
    pub options: SurvexOptions,
}
/// Implementation of SurvexMeta
impl SurvexMeta {
    pub fn new() -> Self { Self::default() }
    pub fn name(self, name: &str) -> Self{
        let begin = self.begin.change(name);
        let end = self.end.change(name);
        Self {
            begin, end, ..self
        }
    }
    pub fn author(self, name: &str) -> Self {
        let copyright = self.copyright.change(name);
        Self {
            copyright, ..self
        }
    }
    fn get_desc(&self) -> String{
        self._debug_line.data.clone()
    }
}

impl Display for SurvexMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_line = String::new();
        debug_line.push(self._debug_line._begin.as_char());
        debug_line += &self._debug_line.data.replace(' ', self._separator.to_string().as_str());
            writeln!(f, "{}", debug_line)?;
            write!(f, "{}", self.begin)?;
            write!(f, "{}", self.copyright)?;
            write!(f, "{}", self.data)?;
            for option in &self.options {
                write!(f, "{}", option)?;
            }
            Ok(())
    }
}
impl Default for SurvexMeta {
    fn default() -> Self {
        Self {
            _separator: Sep::default(),
            _debug_line: DEBUG_LINE.to_meta(),
            begin: MetaData::new("begin", None),
            copyright: MetaData::new("copyright",None),
            data: MetaData::new("data diving from to tape compass fromdepth todepth ignoreall", None),
            end: MetaData::new("end", None),
            options: SurvexOptions::default(),
        }
    }
}