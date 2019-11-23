lazy_static! {
  static ref WK_REGEX: regex::Regex = regex::Regex::new("^wk:").unwrap();
}

#[derive(Debug, Clone)]
pub enum CommandKind {
  WK,
  Shell,
}

#[derive(Debug, Clone)]
pub struct CommandBuilder {
  cwd: Option<std::path::PathBuf>,
  args: Vec<String>,
  name: String,
  kind: CommandKind,
  hidden: bool,
  source: std::path::PathBuf,
  command: String,
  variables: std::collections::HashMap<String, String>,
  description: Option<String>,
  dependencies: Vec<String>,
}

impl CommandBuilder {

  pub fn new() -> Self {
    Self {
      cwd: None,
      args: Vec::new(),
      name: String::from("command"),
      kind: CommandKind::Shell,
      hidden: false,
      source: std::path::PathBuf::new(),
      command: String::from(""),
      variables: std::collections::HashMap::new(),
      description: None,
      dependencies: Vec::new(),
    }
  }

  pub fn with_name<S>(&mut self, name: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.name = name.into();
    self
  }

  pub fn with_command<S>(&mut self, command: S) -> &mut Self
  where
    S: Into<String>,
  {
    let cmd = command.into();
    let parameters: Vec<&str> = cmd.split_whitespace().collect();

    let mut iterator = parameters.into_iter().enumerate();
    while let Some((index, param)) = iterator.next() {
      if index == 0 {
        if WK_REGEX.is_match(param) {
          self.kind = CommandKind::WK;
          self.command = WK_REGEX.replace(param, "").into();
        } else {
          self.kind = CommandKind::Shell;
          self.command = param.into();
        }
      } else {
        self.args.clear();
        self.args.push(param.into());
      }
    }

    self
  }

  pub fn with_description<S>(&mut self, description: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.description = Some(description.into());
    self
  }

  pub fn with_cwd<S>(&mut self, cwd: Option<S>) -> &mut Self
  where
    S: Into<std::path::PathBuf>,
  {
    self.cwd = cwd.map(|s| s.into());
    self
  }

  pub fn with_source<S>(&mut self, source: S) -> &mut Self
  where
    S: Into<std::path::PathBuf>,
  {
    self.source = source.into();
    self
  }

  pub fn with_hidden(&mut self, hidden: bool) -> &mut Self {
    self.hidden = hidden;
    self
  }

  pub fn with_dependency<S>(&mut self, dependency: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.dependencies.push(dependency.into());
    self
  }

  pub fn with_dependencies<I, S>(&mut self, dependencies: I) -> &mut Self
  where
    I: IntoIterator<Item=S>,
    S: Into<String>,
  {
    for dependency in dependencies {
      self.with_dependency(dependency);
    }
    self
  }

  pub fn with_arg<S>(&mut self, arg: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.args.push(arg.into());
    self
  }

  pub fn with_args<I, S>(&mut self, args: I) -> &mut Self
  where
    I: IntoIterator<Item=S>,
    S: Into<String>,
  {
    for arg in args {
      self.with_arg(arg);
    }
    self
  }

  pub fn with_variables(&mut self, variables: std::collections::HashMap<String, String>) -> &mut Self {
    self.variables.extend(variables);
    self
  }

  pub fn into_command(mut self) {
    self.args.insert(0, self.command);

    let mut variables = self.variables;
    let mut args = self.args.into_iter();
    while let Some(arg) = args.next() {

      for (key, value) in variables.iter() {
        let r_key = format!("${{{}}}", key);
        let r_key: &str = r_key.as_str();
        let RE: regex::Regex = regex::Regex::new(r_key).unwrap();

        println!("{}", RE.replace(arg.as_str(), |c: &regex::Captures| value));
      }


      // let capture_option = RE.captures(value.as_str()).and_then(|capture| {

      //   for (key, value) in variables.iter() {
      //     // if let Some(m) = capture.get(1) {
      //     //   RE.replace(m.as_str(), )
      //     // }
      //   //   let r_key = format!("${{{}}}", key);
      //   //   println!("{:?}", r_key);
      //   //   println!("{:?}", capture);
      //   //   println!("{:?}", capture.name(r_key.as_str()));
      //   //   // let v = capture.name(key).map(|| value);
      //   //   // println!(v);
      //   }

      //   Some("")
      // });
    }

    // let mut variables = self.variables.into_iter();
    // while let Some((key, value)) = variables.next() {

    //   RE.captures
    // }

    // let mut args = self.args
    // .iter()
    // .map(|arg: &String| {

    //   // arg.as_str()

    //   let captures_option: Option<regex::Captures> = RE.captures(arg);
    //   if let Some(captures) = captures_option {
    //     let m_options = captures.get(1);
    //     if let Some(m) = m_options {
    //       let key = m.as_str();
    //       let value_option = variables.get(key);
    //       if let Some(value) = value_option {
    //         println!("{:?}", RE.replace(arg.as_str(), |caps: &regex::Captures| {
    //           println!("{:?}", caps);
    //           ""
    //         }));
    //       }
    //     }
    //   }
    //   return ();
    // });

    // args.next();
    // args.next();
    // args.next();

    // Command {
    //   args: self.args,
    //   dependencies: Vec::new(),
    //   kind: self.kind
    // }
  }

}

impl std::str::FromStr for CommandBuilder {
  type Err = std::str::Utf8Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut command = CommandBuilder::new();
    command.with_command(s);
    Ok(command)
  }
}

pub struct Command {
  // cwd: Option<std::path::PathBuf>,
  args: Vec<String>,
  // name: String,
  kind: CommandKind,
  // hidden: bool,
  // source: std::path::PathBuf,
  // command: String,
  // variables: std::collections::HashMap<String, String>,
  // description: Option<String>,
  dependencies: Vec<String>,
}