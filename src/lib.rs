use tokio::sync::Notify;

/// Deliver () when the object is dropped.
/// It's useful to use together with https://crates.io/crates/tokio-interruptible-future

pub struct NotifyDrop<'a> {
    notify: &'a Notify,
}

impl<'a> NotifyDrop<'a> {
    #[allow(dead_code)]
    pub fn new(notify: &'a Notify) -> Self {
        Self { notify }
    }
}

impl<'a> Drop for NotifyDrop<'a> {
    fn drop(&mut self) {
        self.notify.notify_waiters();
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use tokio::sync::Notify;

    use crate::NotifyDrop;

    #[test]
    fn no_deadlock() {
        let notify = Notify::new();
        block_on(async {
            let notified = notify.notified(); //.clone().await;
            { // block
                let _guard = NotifyDrop::new(&notify);
            }
            notified.await;
        });
    }
}
