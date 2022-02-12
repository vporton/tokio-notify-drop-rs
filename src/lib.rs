/// Deliver () when the object is dropped.
/// It's useful to use together with https://crates.io/crates/tokio-interruptible-future

pub struct NotifyDrop<'a> {
    notify: &'a tokio::sync::broadcast::Sender<()>,
}

impl<'a> NotifyDrop<'a> {
    #[allow(dead_code)]
    pub fn new(notify: &'a tokio::sync::broadcast::Sender<()>) -> Self {
        Self { notify }
    }
}

impl<'a> Drop for NotifyDrop<'a> {
    fn drop(&mut self) {
        let _ = self.notify.send(());
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::NotifyDrop;

    #[test]
    fn no_deadlock() {
        let (tx, mut rx) = tokio::sync::broadcast::channel(1);
                { // block
            let _guard = NotifyDrop::new(&tx);
        }
        block_on(async { rx.recv().await.unwrap() });
    }
}
