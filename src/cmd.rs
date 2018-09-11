use std::{
    iter,
    str,
    fmt,
    ffi::OsStr,
};

use ::parse::Parse;

#[derive(Clone, Debug)]
pub struct Cmd<'a> {
    pub command: &'a OsStr,
    pub args: iter::Map<
        str::SplitWhitespace<'a>,
        fn(&'a str) -> &'a OsStr
    >,
}

#[derive(Clone, Debug)]
pub enum ParseCmdError {
    NoInput,
    Other,
}

impl<'a> Parse<'a> for Cmd<'a> {
    type Error = ParseCmdError;
    fn parse_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut args = s.trim().split_whitespace()
            .map(OsStr::new as fn(&'a str) -> &'a OsStr);
        let command = args.next().ok_or(ParseCmdError::NoInput)?;
        let command = Cmd {
            command,
            args,
        };
        Ok(command)
    }
}

impl<'a> fmt::Display for Cmd<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.command.to_string_lossy())?;
        for ref arg in self.args.clone() {
            write!(f, " {}", arg.to_string_lossy())?;
        }
        Ok(())
    }
}
