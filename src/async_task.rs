use smol::channel::{self, Receiver, Sender};

#[derive(Debug, Clone)]
pub enum AsyncEvent<T> {
    LoadingStarted,
    LoadingComplete(T),
    LoadingError(String),
}

pub struct AsyncTask<T> {
    sender: Option<Sender<AsyncEvent<T>>>,
    receiver: Receiver<AsyncEvent<T>>,
    _join_handle: Option<smol::Task<()>>,
}

impl<T: Send + 'static> AsyncTask<T> {
    pub fn new() -> Self {
        let (sender, receiver) = channel::bounded(1);
        Self {
            sender: Some(sender),
            receiver,
            _join_handle: None,
        }
    }

    pub fn spawn<F>(&mut self, future: F) 
    where 
        F: futures_core::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
    {
        // Cancel previous task if still running
        self.cancel();

        let (new_sender, new_receiver) = channel::bounded(1);
        self.sender = Some(new_sender.clone());
        self.receiver = new_receiver;

        // Send loading state immediately using smol's async channel
        let loading_sender = new_sender.clone();
        smol::spawn(async move {
            let _ = loading_sender.send(AsyncEvent::LoadingStarted).await;
        }).detach();

        // Use smol::spawn instead of thread::spawn + smol::block_on
        let task = smol::spawn(async move {
            let result = future.await;

            match result {
                Ok(output) => {
                    let _ = new_sender.send(AsyncEvent::LoadingComplete(output)).await;
                }
                Err(err) => {
                    let _ = new_sender.send(AsyncEvent::LoadingError(
                        format!("Job failed: {}", err)
                    )).await;
                }
            }
        });

        self._join_handle = Some(task);
    }

    pub fn try_recv(&mut self) -> Option<AsyncEvent<T>> {
        // Use smol's async channel try_recv
        match self.receiver.try_recv() {
            Ok(event) => Some(event),
            Err(channel::TryRecvError::Empty) => None,
            Err(channel::TryRecvError::Closed) => None,
        }
    }

    // Async version for even better integration
    pub async fn recv(&mut self) -> Option<AsyncEvent<T>> {
        match self.receiver.recv().await {
            Ok(event) => Some(event),
            Err(_) => None,
        }
    }

    pub fn cancel(&mut self) {
        if let Some(sender) = self.sender.take() {
            drop(sender); // Close channel to signal cancellation
        }
        self._join_handle = None; // Drop the task
    }

    pub fn is_running(&self) -> bool {
        self._join_handle.is_some()
    }

    /// Create an async event stream for use with smol::select!
    pub fn event_stream(&mut self) -> impl futures_core::Stream<Item = AsyncEvent<T>> + '_ {
        futures_util::stream::unfold(&mut self.receiver, |receiver| async move {
            match receiver.recv().await {
                Ok(event) => Some((event, receiver)),
                Err(_) => None,
            }
        })
    }
}

// Convenience functions for common async patterns
impl<T: Send + 'static> AsyncTask<T> {
    /// Spawn a simple async function that returns T directly
    pub fn spawn_simple<F, Fut>(&mut self, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: futures_core::Future<Output = T> + Send + 'static,
    {
        self.spawn(async move {
            Ok(f().await)
        });
    }

    /// Spawn a fallible async function
    pub fn spawn_fallible<F, Fut>(&mut self, f: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: futures_core::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
    {
        self.spawn(f());
    }
}