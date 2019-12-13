use super::future::CommandFuture;
use std::{collections::HashMap, fmt, path::PathBuf};

#[derive(Debug)]
pub struct Command<'a> {
  pub name: &'a str,
  pub cwd: Option<PathBuf>,
  pub args: Vec<String>,
  pub shell: PathBuf,
  pub dependencies: &'a Vec<String>,
  pub environments: &'a HashMap<String, String>,
}

impl<'a> Command<'a> {
  pub fn execute(mut self) -> CommandFuture {
    CommandFuture::new(&mut self)
  }

  pub fn debug(&self) {
    print!("\n\n");

    if let Some(cwd) = &self.cwd {
      print!("\nFrom: {} ", cwd.to_string_lossy());
    }

    print!("with {}\n", self.shell.to_string_lossy());
    println!("Run {}\n", self.name);
  }

  pub fn display(&self) {
    println!("{}", self);
  }
}

impl<'a> std::fmt::Display for Command<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\n\n")?;
    write!(f, "Run: {}\n", self.name)?;
    write!(f, "Dependencies: ")?;
    write!(f, "{}\n", self.dependencies.join(", "))?;

    write!(f, "Environments:")?;
    for (key, value) in self.environments {
      write!(f, " {}={}", key, value)?;
    }
    write!(f, "\n")?;

    if let Some(cwd) = &self.cwd {
      writeln!(f, "From: {}", cwd.to_string_lossy())?;
    }

    writeln!(f, "Shell: {}", self.shell.to_string_lossy())?;
    writeln!(f, "Command: {:?}", self.args.join(" "))?;

    Ok(())
  }
}
