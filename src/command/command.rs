use super::future::CommandFuture;
use std::{collections::HashMap, fmt, path::PathBuf};

#[derive(Debug)]
pub struct Command {
  pub name: String,
  pub cwd: Option<PathBuf>,
  pub args: Vec<String>,
  pub shell: PathBuf,
  pub dependencies: Vec<String>,
  pub environments: HashMap<String, String>,
}

impl Command {
  pub fn execute(mut self) -> CommandFuture {
    CommandFuture::new(&mut self)
  }

  pub fn debug(&self) {
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

impl std::fmt::Display for Command {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Dependencies: ")?;
    write!(f, "{}\n", self.dependencies.join(", "))?;

    write!(f, "Environments:")?;
    for (key, value) in &self.environments {
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
