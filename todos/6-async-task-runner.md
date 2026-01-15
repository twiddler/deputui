# AsyncTaskRunner Refactor - Linear Implementation Checklist

## Overview

Refactor `AsyncTask` to `AsyncTaskRunner` that accepts futures directly, eliminating the blocking behavior that causes UI sluggishness.

## Todos

- [x] Update struct definition in async_task.rs:
  - Rename `AsyncTask` to `AsyncTaskRunner`
  - Remove `operation: Box<dyn Fn(Input) -> anyhow::Result<T> + Send + Sync>` field entirely
  - Change generic parameters from `<T, Input>` to `<T>`
  - Keep `status: Arc<Mutex<AsyncTaskStatus<T>>>` field unchanged
  - Keep `notify_subscribers: Sender<()>` field unchanged
  - Remove `std::pin::Pin` and `std::future::Future` imports (not needed)
  - Keep existing `smol::channel::Sender` and `std::sync::*` imports

- [x] Update implementation block for AsyncTaskRunner:
  - Change `impl<T, Input> AsyncTask<T, Input>` to `impl<T> AsyncTaskRunner<T>`
  - Update trait bounds to keep only `T: Send + Clone + 'static`
  - Remove `Input: Send + 'static` trait bound

- [x] Update constructor method:
  - Change `new<F>(operation: F, notify_subscribers: Sender<()>)` to `new(notify_subscribers: Sender<()>)`
  - Remove operation parameter entirely
  - Remove operation field assignment
  - Keep status and notify_subscribers initialization unchanged

- [x] Update start_operation method signature:
  - Change from `start_operation(&self, input: Input)` to `start_operation<F>(&self, future: F)`
  - Add trait bound: `where F: Future<Output = anyhow::Result<T>> + Send + 'static`
  - Remove all `Input` related parameters and code

- [x] Rewrite start_operation method body:
  - Keep `self.set_status(AsyncTaskStatus::Loading);` call
  - Remove line `let result = (operation)(input);` (blocking sync call)
  - Remove `let operation = &self.operation;` line
  - Keep `status_clone` and `notify_subscribers_clone` cloning
  - Replace spawn block with simple future handling:

  ```rust
  smol::spawn(async move {
      let new_status = match future.await {
          Ok(result) => AsyncTaskStatus::Loaded(result),
          Err(e) => AsyncTaskStatus::Error(e.to_string()),
      };
      // Rest of status update logic unchanged
  }).detach();
  ```

- [x] Update status() method:
  - No changes needed to status() method implementation
  - Ensure it still returns `AsyncTaskStatus<T>` correctly

- [x] Update set_status() method:
  - No changes needed to set_status() method implementation
  - Keep existing status update and notification logic

- [x] Keep AsyncTaskStatus enum unchanged:
  - Verify `AsyncTaskStatus<T>` enum remains unchanged
  - Ensure all variants (Idle, Loading, Loaded, Error) are preserved

- [x] Update app.rs integration:
  - Rename `release_notes_task` variable to `release_notes_runner` throughout app.rs
  - Change variable declaration from `AsyncTask<String, Release>` to `AsyncTaskRunner<String>`
  - Update constructor call from `AsyncTask::new(operation, render_tx.clone())` to `AsyncTaskRunner::new(render_tx.clone())`
  - Remove the blocking operation: `|release: Release| smol::block_on(async move { ReleaseExt(&release).fetch_release_notes().await })`
  - Update operation usage from `self.release_notes_task.start_operation(release.clone())` to:

  ```rust
  self.release_notes_runner.start_operation(async move {
      ReleaseExt(&release).fetch_release_notes().await
  });
  ```

  - Update all method calls from `self.release_notes_task.status()` to `self.release_notes_runner.status()`

- [x] Update test_async_task_initial_state():
  - Rename variable from `task` to `runner`
  - Change constructor call to `AsyncTaskRunner::new(tx)`
  - Remove operation parameter
  - Verify assertion still works with `AsyncTaskStatus::Idle`

- [x] Update test_async_task_success_operation():
  - Rename variable from `echo` to `runner`
  - Change constructor call to `AsyncTaskRunner::new(tx)`
  - Replace operation call with future:

    ```rust
    runner.start_operation(async { Ok("test_input".to_string()) });
    ```

  - Keep the async wait and assertion logic unchanged

- [x] Update test_async_task_error_operation():
  - Rename variable from `task` to `runner`
  - Change constructor call to `AsyncTaskRunner::new(tx)`
  - Replace operation call with future:

    ```rust
    runner.start_operation(async { Err(anyhow::anyhow!("test error")) });
    ```

  - Keep the async wait and assertion logic unchanged

- [x] Add new test for multiple operations:
  - Create test that starts multiple operations rapidly
  - Verify that only the latest operation result is kept
  - Test with different input scenarios
