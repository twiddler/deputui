use smol::channel::Sender;
use std::sync::{Arc, Mutex};

pub struct AsyncTask<T, Input> {
    status: Arc<Mutex<AsyncTaskStatus<T>>>,
    notify_subscribers: Sender<()>,
    operation: Box<dyn Fn(Input) -> anyhow::Result<T> + Send + Sync>,
}

impl<T, Input> AsyncTask<T, Input>
where
    T: Send + Clone + 'static,
    Input: Send + 'static,
{
    pub fn new<F>(operation: F, notify_subscribers: Sender<()>) -> Self
    where
        F: Fn(Input) -> anyhow::Result<T> + Send + Sync + 'static,
    {
        Self {
            status: Arc::new(Mutex::new(AsyncTaskStatus::Idle)),
            notify_subscribers,
            operation: Box::new(operation),
        }
    }

    pub fn start_operation(&self, input: Input) {
        self.set_status(AsyncTaskStatus::Loading);

        // Clone necessary data for the async task
        let status_clone = Arc::clone(&self.status);
        let notify_subscribers_clone = self.notify_subscribers.clone();
        let operation = &self.operation;

        // Create a future from the operation closure
        let result = (operation)(input);

        // Spawn the async task
        smol::spawn(async move {
            let new_status = match result {
                Ok(result) => AsyncTaskStatus::Loaded(result),
                Err(e) => AsyncTaskStatus::Error(e.to_string()),
            };

            {
                let mut status = status_clone.lock().unwrap();
                *status = new_status;
            }

            notify_subscribers_clone.try_send(()).ok();
        })
        .detach();
    }

    pub fn status(&self) -> AsyncTaskStatus<T> {
        self.status.lock().unwrap().clone()
    }

    fn set_status(&self, new_status: AsyncTaskStatus<T>) {
        {
            let mut status = self.status.lock().unwrap();
            *status = new_status;
        }

        self.notify_subscribers.try_send(()).ok();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsyncTaskStatus<T> {
    Idle,
    Loading,
    Loaded(T),
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol::channel;

    #[test]
    fn test_async_task_initial_state() {
        let (tx, _rx) = channel::bounded::<()>(1);
        let task = AsyncTask::<String, ()>::new(|_| Ok("test".to_string()), tx);
        assert_eq!(task.status(), AsyncTaskStatus::Idle);
    }

    #[test]
    fn test_async_task_success_operation() {
        let (tx, rx) = channel::bounded::<()>(1);
        let echo = AsyncTask::<String, String>::new(|input| Ok(input), tx);

        echo.start_operation("test_input".to_string());

        smol::block_on(async {
            while matches!(echo.status(), AsyncTaskStatus::Loading) {
                rx.recv().await.ok();
            }
        });

        assert_eq!(
            echo.status(),
            AsyncTaskStatus::Loaded("test_input".to_string())
        );
    }

    #[test]
    fn test_async_task_error_operation() {
        let (tx, rx) = channel::bounded::<()>(1);
        let task = AsyncTask::<String, ()>::new(|_| Err(anyhow::anyhow!("test error")), tx);

        task.start_operation(());

        smol::block_on(async {
            while matches!(task.status(), AsyncTaskStatus::Loading) {
                rx.recv().await.ok();
            }
        });

        assert_eq!(
            task.status(),
            AsyncTaskStatus::Error("test error".to_string())
        );
    }
}
