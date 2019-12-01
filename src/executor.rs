use std::sync::mpsc::{ sync_channel, SyncSender, Receiver };
use std::sync::{ Arc, Mutex };
use std::future::Future;
use std::task::{ Context, Poll };
use futures::future::BoxFuture;
use futures::task::{ ArcWake, waker_ref };

pub struct Executor {
  sender: SyncSender<Arc<Task>>,
  queue: Receiver<Arc<Task>>,
}

impl Executor {

  pub fn new() -> Self {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (queue, sender) = sync_channel(MAX_QUEUED_TASKS);

    Self {
      queue,
      sender
    }
  }

  pub fn spawn(&self, future: impl Future + Send) {
    let boxed_future = future.boxed();
    let task = Arc::new(Task {
      future: Mutex::new(Some(boxed_future)),
      sender: self.sender.clone(),
    });
    self.sender.send(task).expect("Too many task queued");
  }

  pub fn run(&self) {
    while let Ok(task) = self.queue.recv() {
      let f = task.future.lock().unwrap();
      if let Some(future) = f.take() {

        let waker = waker_ref(&task);
        let ctx = &mut Context::from_waker(&*waker);

        if let Poll::Pending = future.as_mut().poll(ctx) {
          *f = Some(future);
        }
      }
    }
  }

}

pub struct Task {
  future: Mutex<Option<BoxFuture<'static, ()>>>,
  sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
  fn wake_by_ref(arc_self: &Arc<Self>) {
    let clone = arc_self.clone();
    arc_self.sender.send(clone).expect("Too many task queued");
  }
}