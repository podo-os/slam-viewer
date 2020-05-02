use std::thread;

pub struct Engine {
    thread: thread::JoinHandle<()>,
}

impl Engine {
    pub(super) fn new<F>(handle: F) -> Self
    where
        F: 'static + FnOnce() + Send,
    {
        Self {
            thread: thread::spawn(handle),
        }
    }

    pub fn wait(self) {
        self.thread.join().unwrap();
    }
}
