use crossbeam::channel::{unbounded, Receiver, Sender};
use futures::task::{waker_ref, ArcWake};
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    // Shared state between the future and
    // the sleeping thread
    state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    // This is our Waker. If it exists, we'll
    // call wake on it from the sleeping thread
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.completed {
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up
            // the current task when the timer has completed
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let thread_state = state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut state = thread_state.lock().unwrap();
            state.completed = true;
            if let Some(waker) = state.waker.take() {
                waker.wake()
            }
        });
        TimerFuture { state }
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    task_sender: Sender<Arc<Task>>,
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: Sender<Arc<Task>>,
}

////////////////////////////////////////////////////////////////////////////////

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back
        // onto the task channel, so that it will be
        // polled again by the executor.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

////////////////////////////////////////////////////////////////////////////////

fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (task_sender, ready_queue) = unbounded();
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = Box::pin(future);
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("channel disconnected");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // Create a Waker from the task itself
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // This Future is non-leaf
    spawner.spawn(async {
        println!("howdy!");
        // And it's a leaf future
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    drop(spawner);

    executor.run();
}
