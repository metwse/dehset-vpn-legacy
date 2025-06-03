use crate::{
    tls_provider::TlsProvider,
    tunnel::{Tunnel, TunnelError},
};
use std::{collections::VecDeque, sync::Arc};
use tokio::{
    io::AsyncWrite,
    sync::{Mutex, Notify},
};

/// Holds encoded payloads until they are ready to be sent over the TLS tunnel.
pub struct MessageQueue<R, W, T> {
    tunnel: Arc<Tunnel<R, W, T>>,
    /// Per-priority message queues for each connection.
    ///
    /// Each inner [`VecDeque`] represents a queue of messages at a given
    /// priority level. Messages in lower-indexed queues are considered higher
    /// priority and are sent first.
    queue: Arc<Vec<Mutex<VecDeque<Vec<u8>>>>>,
    notify: Arc<Notify>,
}

impl<R, W, T> MessageQueue<R, W, T> {
    /// Creates a new [`MessageQueue`].
    pub fn new(tunnel: Arc<Tunnel<R, W, T>>, queue_count: usize) -> MessageQueue<R, W, T> {
        let mut queue = Vec::with_capacity(queue_count);
        for _ in 0..queue_count {
            queue.push(Mutex::new(VecDeque::new()));
        }

        MessageQueue {
            tunnel,
            queue: Arc::new(queue),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Pushes a message into the queue.
    pub async fn push_message(&self, payload: Vec<u8>, importance: usize) {
        let mut queue = self.queue[importance].lock().await;
        queue.push_back(payload);
        drop(queue);
        self.notify.notify_one();
    }
}

impl<R, W, T> MessageQueue<R, W, T>
where
    W: Unpin + AsyncWrite,
    T: TlsProvider,
{
    /// Awaits and sends messages in importance order.
    pub async fn message_service(&self) -> Result<(), TunnelError> {
        loop {
            let mut message = None;

            for i in 0..self.queue.len() {
                let mut queue = self.queue[i].lock().await;
                if queue.len() > 0 {
                    message = queue.pop_front();
                    break;
                }
            }

            if let Some(message) = message {
                self.tunnel.send(&message).await?;
            } else {
                self.notify.notified().await;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{tls_provider::MockTls, tunnel::Tunnel};
    use super::MessageQueue;
    use std::sync::Arc;
    use testutil::DynResult;
    use tokio::{io::simplex, sync::Mutex};

    #[tokio::test]
    async fn message_queue() -> DynResult<()> {
        let (r, w) = simplex(usize::MAX);

        let tunnel = Arc::new(Tunnel {
            r: Mutex::new(r),
            w: Mutex::new(w),
            tls: MockTls {},
        });

        let message_queue = MessageQueue::new(Arc::clone(&tunnel), 3);

        message_queue.push_message(vec![1, 2, 3], 0).await;
        message_queue.push_message(vec![3, 4, 5], 2).await;
        message_queue.push_message(vec![2, 3, 4], 1).await;

        tokio::spawn(async move {
            message_queue.message_service().await.unwrap();
        });

        assert_eq!(tunnel.recv().await?, [1, 2, 3]);
        assert_eq!(tunnel.recv().await?, [2, 3, 4]);
        assert_eq!(tunnel.recv().await?, [3, 4, 5]);

        Ok(())
    }
}
