#[cfg(test)]
mod tests {
    use crate::async_task::{AsyncTask, AsyncEvent};
    use std::time::Duration;

    #[test]
    fn test_async_task_loading() {
        smol::block_on(async {
            let mut task: AsyncTask<String> = AsyncTask::new();

            // Start a simple async task
            task.spawn(async {
                Ok("test result".to_string())
            });

            // Should get loading state immediately
            let event = task.try_recv();
            assert!(matches!(event, Some(AsyncEvent::LoadingStarted)));

            // Wait for completion
            smol::Timer::after(Duration::from_millis(100)).await;

            // Should get completion event
            let event = task.try_recv();
            assert!(matches!(event, Some(AsyncEvent::LoadingComplete(_))));
        })
    }

    #[test]
    fn test_cancellation() {
        smol::block_on(async {
            let mut task: AsyncTask<String> = AsyncTask::new();

            // Start a long-running task
            task.spawn(async {
                smol::Timer::after(Duration::from_millis(1000)).await;
                Ok("test result".to_string())
            });
            assert!(task.is_running());

            // Cancel immediately
            task.cancel();
            assert!(!task.is_running());
        })
    }

    #[test]
    fn test_different_job_types() {
        smol::block_on(async {
            let mut string_task: AsyncTask<String> = AsyncTask::new();
            let mut number_task: AsyncTask<i32> = AsyncTask::new();

            // Different job types can run simultaneously
            string_task.spawn(async {
                Ok("string result".to_string())
            });

            number_task.spawn(async {
                Ok(42)
            });

            // Both should be running
            assert!(string_task.is_running());
            assert!(number_task.is_running());

            // Wait for completion
            smol::Timer::after(Duration::from_millis(50)).await;

            let event1 = string_task.try_recv();
            let event2 = number_task.try_recv();

            assert!(matches!(event1, Some(AsyncEvent::LoadingComplete(_))));
            assert!(matches!(event2, Some(AsyncEvent::LoadingComplete(_))));
        })
    }

    #[test]
    fn test_async_recv() {
        smol::block_on(async {
            let mut task: AsyncTask<String> = AsyncTask::new();

            // Start a simple async task
            task.spawn(async {
                smol::Timer::after(Duration::from_millis(50)).await;
                Ok("async result".to_string())
            });

            // Should get loading state
            let event = task.try_recv();
            assert!(matches!(event, Some(AsyncEvent::LoadingStarted)));

            // Wait for completion async
            let event = task.recv().await;
            assert!(matches!(event, Some(AsyncEvent::LoadingComplete(content)) if content == "async result"));
        })
    }
}