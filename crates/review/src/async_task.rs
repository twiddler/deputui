use smol::channel::Sender;
use std::future::Future;
use std::sync::{Arc, Mutex};

use crate::UiMessage;

pub struct AsyncTaskRunner<T> {
    status: Arc<Mutex<AsyncTaskStatus<T>>>,
    notify_subscribers: Sender<UiMessage>,
    task_id: Arc<Mutex<u64>>,
}

impl<T> AsyncTaskRunner<T>
where
    T: Send + Clone + 'static,
{
    pub fn new(notify_subscribers: Sender<UiMessage>) -> Self {
        Self {
            status: Arc::new(Mutex::new(AsyncTaskStatus::Idle)),
            notify_subscribers,
            task_id: Arc::new(Mutex::new(0)),
        }
    }

    pub fn start_operation<F>(&self, future: F)
    where
        F: Future<Output = anyhow::Result<T>> + Send + 'static,
    {
        // Increment task ID to cancel any existing task
        let current_task_id = {
            let mut task_id = self.task_id.lock().unwrap();
            *task_id += 1;
            *task_id
        };

        self.set_status(AsyncTaskStatus::Loading);

        // Clone necessary data for the async task
        let status_clone = Arc::clone(&self.status);
        let notify_subscribers_clone = self.notify_subscribers.clone();
        let task_id_clone = Arc::clone(&self.task_id);

        // Spawn the async task
        smol::spawn(async move {
            let task_result = future.await;

            // Check if this task is still the current one
            let is_current_task = {
                let task_id = task_id_clone.lock().unwrap();
                *task_id == current_task_id
            };

            if !is_current_task {
                // This task has been superseded by a newer one
                return;
            }

            let new_status = match task_result {
                Ok(value) => AsyncTaskStatus::Loaded(value),
                Err(e) => AsyncTaskStatus::Error(e.to_string()),
            };

            {
                let mut status = status_clone.lock().unwrap();
                *status = new_status;
            }

            notify_subscribers_clone
                .try_send(UiMessage::TaskComplete)
                .ok();
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

        self.notify_subscribers
            .try_send(UiMessage::TaskComplete)
            .ok();
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
        let (tx, _rx) = channel::bounded::<UiMessage>(1);
        let runner = AsyncTaskRunner::<String>::new(tx);
        assert_eq!(runner.status(), AsyncTaskStatus::Idle);
    }

    #[test]
    fn test_async_task_success_operation() {
        let (tx, rx) = channel::bounded::<UiMessage>(1);
        let runner = AsyncTaskRunner::<String>::new(tx);

        runner.start_operation(async { Ok("test_input".to_string()) });

        smol::block_on(async {
            while matches!(runner.status(), AsyncTaskStatus::Loading) {
                rx.recv().await.ok();
            }
        });

        assert_eq!(
            runner.status(),
            AsyncTaskStatus::Loaded("test_input".to_string())
        );
    }

    #[test]
    fn test_async_task_error_operation() {
        let (tx, rx) = channel::bounded::<UiMessage>(1);
        let runner = AsyncTaskRunner::<String>::new(tx);

        runner.start_operation(async { Err(anyhow::anyhow!("test error")) });

        smol::block_on(async {
            while matches!(runner.status(), AsyncTaskStatus::Loading) {
                rx.recv().await.ok();
            }
        });

        assert_eq!(
            runner.status(),
            AsyncTaskStatus::Error("test error".to_string())
        );
    }

    #[test]
    fn test_multiple_operations_rapid_succession() {
        let (tx, rx) = channel::bounded::<UiMessage>(1);
        let runner = AsyncTaskRunner::<String>::new(tx);

        // Start multiple operations rapidly
        runner.start_operation(async {
            smol::Timer::after(std::time::Duration::from_millis(100)).await;
            Ok("first".to_string())
        });

        runner.start_operation(async {
            smol::Timer::after(std::time::Duration::from_millis(10)).await;
            Ok("second".to_string())
        });

        // Wait for the final operation to complete
        smol::block_on(async {
            while matches!(runner.status(), AsyncTaskStatus::Loading) {
                rx.recv().await.ok();
            }
        });

        // Should have the result from the last operation
        assert_eq!(
            runner.status(),
            AsyncTaskStatus::Loaded("second".to_string())
        );
    }

    #[test]
    fn test_multiple_operations_mixed_success_error() {
        let (tx, rx) = channel::bounded::<UiMessage>(1);
        let runner = AsyncTaskRunner::<String>::new(tx);

        // Start with a successful operation
        runner.start_operation(async {
            smol::Timer::after(std::time::Duration::from_millis(100)).await;
            Ok("success".to_string())
        });

        // Then start an error operation
        runner.start_operation(async {
            smol::Timer::after(std::time::Duration::from_millis(10)).await;
            Err(anyhow::anyhow!("error operation"))
        });

        // Wait for the final operation to complete
        smol::block_on(async {
            while matches!(runner.status(), AsyncTaskStatus::Loading) {
                rx.recv().await.ok();
            }
        });

        // Should have the error from the last operation
        assert_eq!(
            runner.status(),
            AsyncTaskStatus::Error("error operation".to_string())
        );
    }

    #[test]
    fn test_cancellation_prevents_stale_results() {
        let (tx, rx) = channel::bounded::<UiMessage>(10);
        let runner = AsyncTaskRunner::<String>::new(tx);

        // Start a slow operation that will be superseded
        runner.start_operation(async {
            smol::Timer::after(std::time::Duration::from_millis(100)).await;
            Ok("slow_result".to_string())
        });

        // Start a fast operation that should complete first and be the final result
        runner.start_operation(async {
            smol::Timer::after(std::time::Duration::from_millis(10)).await;
            Ok("fast_result".to_string())
        });

        // Wait for the fast operation to complete (loading -> loaded transition)
        smol::block_on(async {
            while matches!(runner.status(), AsyncTaskStatus::Loading) {
                rx.recv().await.ok();
            }
        });

        // Verify we got the fast result, not the slow one
        assert_eq!(
            runner.status(),
            AsyncTaskStatus::Loaded("fast_result".to_string())
        );

        // Wait longer to ensure the slow operation had time to complete
        // (but it shouldn't affect the status since it was cancelled)
        smol::block_on(async {
            smol::Timer::after(std::time::Duration::from_millis(200)).await;
        });

        // Status should still be the fast result, proving cancellation worked
        assert_eq!(
            runner.status(),
            AsyncTaskStatus::Loaded("fast_result".to_string())
        );
    }
}
