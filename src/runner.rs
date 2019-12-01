use std::task::{ Context, Poll };
use futures::future::Future;
use std::pin::Pin;
use std::process::Child;
use crate::command::CommandBuilder;

pub struct Runner {
  child: Option<Result<Child, std::io::Error>>
}

impl Runner {
  pub fn new() -> Self {
    Self {
      child: None
    }
  }

  pub fn execute(&mut self, builder: &CommandBuilder) {
    let cmd = builder.into_command();
    println!("{:?}", cmd);

    let res = std::process::Command::new("cmd.exe")
    .arg("/c")
    .args(cmd.args)
    .spawn();

    self.child = Some(res);
  }
}

impl Future for Runner {
  type Output = i32;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let runner = self.get_mut();
    match &mut runner.child {
      Some(Ok(child)) => {
        match child.try_wait() {
          Ok(Some(e)) => {
            println!("{:?}", e);
            Poll::Ready(0)
          },
          Ok(None) => {
            let w = cx.waker().clone();
            w.wake();
            Poll::Pending
          },
          Err(e) => {
            println!("{:?}", e);
            Poll::Ready(-1)
          }
        }
      },
      Some(Err(e)) => {
        println!("{:?}", e);
        Poll::Ready(-1)
      },
      None => {
        let w = cx.waker().clone();
        w.wake();
        Poll::Pending
      }
    }
  }
}