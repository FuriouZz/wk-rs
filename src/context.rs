use crate::{
  command::{Command, CommandBuilder, CommandKind, CommandResult},
  error::Error,
  importer::CommandImported,
};
use std::collections::HashMap;

pub struct Context {
  pub(crate) tasks: HashMap<String, CommandImported>,
}

impl Context {
  pub fn find_builder<S>(&self, name: S) -> Option<&CommandBuilder>
  where
    S: AsRef<str>,
  {
    if let Some(imported) = self.tasks.get(name.as_ref()) {
      match imported {
        CommandImported::Command(builder) => {
          return Some(builder);
        }
        _ => {}
      }
    }

    None
  }

  pub fn find_command<S>(&self, name: S) -> Option<Command>
  where
    S: AsRef<str>,
  {
    if let Some(builder) = self.find_builder(name) {
      return Some(builder.to_command());
    }

    None
  }

  pub fn create_stack<S>(&self, name: S, order: &mut Vec<String>, tasks: &mut HashMap<String, Command>)
  where
    S: AsRef<str>,
  {
    if let Some(mut command) = self.find_command(name.as_ref()) {

      // Add dependencies
      if !command.dependencies.is_empty() {
        for depname in &command.dependencies {
          match self.find_builder(depname.as_str()) {
            Some(_dep) => {
              if depname != name.as_ref() && !tasks.contains_key(depname) {
                self.create_stack(depname, order, tasks);
              }
            }
            None => {}
          }
        }
      }

      let builder = self.find_builder(&name).expect("No command builder found.");

      if let CommandKind::WK = builder.kind {
        let name: &String = &command.args[0];
        self.create_stack(name, order, tasks);
        if let Some(last) = order.last() {
          if last == name {
            order.pop();
            let mut p = tasks.remove(name).unwrap();
            p.cwd = command.cwd;
            p.shell = command.shell;
            p.args.extend(command.args[1..].to_vec());
            command = p;
          }
        }
      }

      let s = name.as_ref().to_owned();
      if !tasks.contains_key(&s) {
        order.push(s.clone());
        tasks.insert(s, command);
      }

    }
  }

  pub async fn run<S>(&self, name: S) -> Result<Vec<CommandResult>, Error>
  where
    S: AsRef<str>,
  {
    if let None = self.find_builder(name.as_ref()) {
      let err = format!("Command \"{}\" not found", name.as_ref().to_string());
      return Err(Error::CommandError(err));
    }

    let mut order: Vec<String> = Vec::new();
    let mut commands: HashMap<String, Command> = HashMap::new();
    self.create_stack(name.as_ref(), &mut order, &mut commands);

    // Run commands
    let mut results: Vec<CommandResult> = Vec::new();
    for name in order.iter() {
      if let Some(c) = commands.remove(name) {
        results.push(c.execute().await);
      }
    }

    Ok(results)
  }
}
