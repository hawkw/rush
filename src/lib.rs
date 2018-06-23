// extern crate combine;
// extern crate combine_language;
use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::Command,
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
}

impl Env {
    pub fn new() -> io::Result<Self> {
        Ok(Env {
            history: Vec::new(),
            pwd: env::current_dir()?,
        })
    }

    pub fn prompt(&self) -> String {
        format!("{} $", self.pwd.display())
    }

    pub fn next_command(&mut self) -> io::Result<()> {
        let mut line = String::new();
        print!("{}", self.prompt());
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;
        self.history.push(line);
        let cmd: Cmd = self.history.last()
            .expect("last was just pushed")
            .parse_into()
            .unwrap();
        let result = Command::new(cmd.command)
            .args(cmd.args)
            .current_dir(&self.pwd)
            .status();
        println!("{:?}", result);
        Ok(())
    }

    pub fn run_loop(&mut self) -> io::Result<()> {
        loop {
           self.next_command()?;
        }
        Ok(())
    }
}




