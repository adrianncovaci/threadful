use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.clones += 1;
        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.clones -= 1;
        let last_one = inner.clones == 1;
        drop(inner);
        if last_one {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    fn send(&mut self, t: T) {
        let shared = self.shared.inner.lock().unwrap().queue.push_back(t);
        drop(shared);
        self.shared.available.notify_one();
    }
}

struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    fn recv(&self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => return Some(t),
                None if inner.clones == 0 => return None,
                None => inner = self.shared.available.wait(inner).unwrap(),
            }
        }
    }
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

struct Inner<T> {
    queue: VecDeque<T>,
    clones: usize,
}

fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        clones: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::default(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: Arc::clone(&shared),
        },
        Receiver {
            shared: Arc::clone(&shared),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn ping_pong() {
        let (mut tx, rx) = channel();
        tx.send("yooo");
        assert_eq!(rx.recv(), Some("yooo"));
    }

    #[test]
    fn dropped_sender() {
        let (tx, rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn ping_pong_threads() {
        let (mut tx, mut rx) = channel();
        let _tx = Arc::new(&tx);
        for i in 0..5 {
            thread::spawn(|| {
                &_tx.send(i);
            });
        }
    }
}
