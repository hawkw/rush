use std::{
    // io,
    // path::Path,
    process::Command,
    str::FromStr,
    ffi::{OsStr, OsString},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct Cmd {
    pub command: OsString,
    pub args: Vec<OsString>,
}

#[derive(Clone, Debug)]
pub enum ParseCmdError {
    NoInput,
    Other,
}

impl FromStr for Cmd {
    type Err = ParseCmdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace().map(|ref s| OsStr::new(s).to_os_string());
        let command = parts.next().ok_or(ParseCmdError::NoInput)?;
        let args = parts.collect::<Vec<_>>();
        Ok(Cmd{
            command,
            args,
        })
    }
}

