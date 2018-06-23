use std::{
    env,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

pub mod cmd;
use cmd::Cmd;


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

    pub fn next_command(&mut self) -> io::Result<&mut Self> {
        let mut line = String::new();
        print!("{}", self.prompt());
        io::stdout().flush()?;
        io::stdin().read_line(&mut line)?;
        let cmd: Cmd = line.parse().unwrap();
        let result = Command::new(cmd.command)
            .args(cmd.args)
            .current_dir(&self.pwd)
            .status();
        println!("{:?}", result);
        self.history.push(line);
        Ok(self)
    }

    pub fn run_loop(&mut self) -> io::Result<()> {
        loop {
           self.next_command()?;
        }
        Ok(())
    }
}




