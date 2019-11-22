pub fn command_from_str<S>(s: S) -> std::process::Command
where
  S: Into<String>,
{

  let mut program = String::from("");
  let mut args: Vec<String> = Vec::new();

  let cmd: String = s.into();
  let parameters: Vec<&str> = cmd.split_whitespace().collect();

  let mut iterator = parameters.into_iter().enumerate();
  while let Some((index, param)) = iterator.next() {
    if index == 0 {
      program = param.into();
    } else {
      args.push(param.into());
    }
  }

  let mut command = std::process::Command::new(program);
  command.args(args);
  return command;

}