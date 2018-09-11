// extern crate combine;
// extern crate combine_language;
use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::{self, Command},
};

pub mod cmd;
pub mod parse;

use cmd::Cmd;
use parse::ParseInto;

#[derive(Clone, Debug)]
pub struct Env {
    // TODO: Vec<Command>?`
    history: Vec<String>,
    pwd: PathBuf,
    last_status: Option<process::ExitStatus>,
}

#[derive(Debug)]
pub enum CommandError {
    Io(io::Error),
    Parse(cmd::ParseCmdError),
}

impl Env {
    pub fn new() -> io::Result<Self> {
        Ok(Env {
            history: Vec::new(),
            pwd: env::current_dir()?,
            last_status: None,
        })
    }

    pub fn prompt(&self) -> String {
        format!("{} $ ", self.pwd.display())
    }

    pub fn next_command(&mut self) -> Result<(), CommandError> {
        let mut line = String::new();
        print!("{}", self.prompt());
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;
        self.history.push(line);
        let cmd: Cmd = self.history.last()
            .expect("last was just pushed")
            .parse_into()?;
        let result = Command::new(cmd.command)
            .args(cmd.clone().args)
            .current_dir(&self.pwd)
            .status();
        match result {
            Err(e) => println!("Error evaluating '{}': {}", cmd, e),
            Ok(status) => self.last_status = Some(status),
        }

        Ok(())
    }

    pub fn run_loop(&mut self) -> io::Result<()> {
        loop {
           match self.next_command() {
               Err(CommandError::Parse(e)) => println!("Parse error: {:?}", e),
               Err(CommandError::Io(e)) => Err(e)?,
               _ => {}
           };
        }
        Ok(())
    }
}

impl From<io::Error> for CommandError {
    fn from(e: io::Error) -> Self {
        CommandError::Io(e)
    }
}

impl From<cmd::ParseCmdError> for CommandError {
    fn from(e: cmd::ParseCmdError) -> Self {
        CommandError::Parse(e)
    }
}
