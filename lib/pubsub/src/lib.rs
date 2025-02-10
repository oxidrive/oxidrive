use std::task::Poll;

use futures::Stream;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Clone)]
pub struct Publisher<E> {
    tx: broadcast::Sender<E>,
}

impl<E: Clone + Send + 'static> Default for Publisher<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: Clone + Send + 'static> Publisher<E> {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }

    pub fn publish(&self, event: E) {
        let _ = self.tx.send(event);
    }

    pub fn subscribe(&self) -> Subscriber<E> {
        let rx = self.tx.subscribe();

        Subscriber {
            rx: BroadcastStream::new(rx),
        }
    }
}

pin_project_lite::pin_project! {
    pub struct Subscriber<E> {
        #[pin]
        rx: BroadcastStream<E>,
    }
}

impl<E: Clone + Send + 'static> Stream for Subscriber<E> {
    type Item = E;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.rx.poll_next(cx) {
            Poll::Ready(Some(Ok(event))) => Poll::Ready(Some(event)),
            Poll::Ready(_) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
