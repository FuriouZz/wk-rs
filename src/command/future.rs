use super::command::Command;
use crate::error::Error;
use std::{
  future::Future,
  pin::Pin,
  process::Child,
  task::{Context, Poll},
};

pub type CommandResult = Result<Option<i32>, Error>;

pub struct CommandFuture {
  process: Option<Result<Child, std::io::Error>>,
}

impl CommandFuture {
  pub fn new(command: &mut Command) -> Self {
    let mut cmd = std::process::Command::new(&command.shell);

    // Set shell
    cmd.arg(command.args.remove(0));

    // Set arguments
    cmd.arg(command.args.join(" "));

    // Set current directory
    if let Some(cwd) = &command.cwd {
      cmd.current_dir(cwd);
    }

    for env in command.environments.iter() {
      cmd.env(env.0, env.1);
    }

    // Execute and store child process
    let child = cmd.spawn();

    Self {
      process: Some(child),
    }
  }
}

impl Future for CommandFuture {
  type Output = CommandResult;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let runner = self.get_mut();
    let wake = || {
      let w = cx.waker().clone();
      w.wake();
    };

    match runner.process.take() {
      Some(Ok(mut child)) => match child.wait() {
        Ok(e) => Poll::Ready(Ok(e.code())),
        Err(e) => Poll::Ready(Err(e.into())),
      },
      Some(Err(e)) => Poll::Ready(Err(e.into())),
      None => {
        wake();
        Poll::Pending
      }
    }
  }
}
