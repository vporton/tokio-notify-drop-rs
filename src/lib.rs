/// Call Notify::notify_one() when object is dropped.
/// It's useful to use together with https://crates.io/crates/tokio-interruptible-future

use tokio::sync::Notify;

struct NotifyDrop<'a> {
    notify: &'a Notify
}

impl<'a> NotifyDrop<'a> {
    #[allow(dead_code)]
    pub fn new(notify: &'a Notify) -> Self {
        Self { notify }
    }
}

impl<'a> Drop for NotifyDrop<'a> {
    fn drop(&mut self) {
        self.notify.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use tokio::sync::Notify;

    use crate::NotifyDrop;

    #[test]
    fn no_deadlock() {
        let n = Notify::new();
        { // block
            let _guard = NotifyDrop::new(&n);
        }
        block_on(async { n.notified().await });
    }
}
