# Modular AsyncTask Design

The `AsyncTask<T>` is now completely decoupled from the specific work being performed. Here's how it can be used:

## Generic Interface

```rust
// AsyncTask can execute ANY Future<Output = Result<T, Error>>
let mut task: AsyncTask<String> = AsyncTask::new();

// Example 1: HTTP request
task.spawn(async {
    let content = fetch_url("https://api.example.com").await?;
    Ok(content)
});

// Example 2: File I/O
task.spawn(async {
    let content = smol::unblock(|| {
        std::fs::read_to_string("large_file.txt")
    }).await?;
    Ok(content)
});

// Example 3: Computation
task.spawn(async {
    let result = expensive_computation().await?;
    Ok(format!("Result: {}", result))
});

// Example 4: Multiple different task types
let mut string_task: AsyncTask<String> = AsyncTask::new();
let mut number_task: AsyncTask<i32> = AsyncTask::new();
let mut json_task: AsyncTask<serde_json::Value> = AsyncTask::new();
```

## Job-Specific Modules

Jobs are completely separate from the async executor:

```rust
// HTTP Job
pub struct HttpJob {
    url: String,
}

impl HttpJob {
    pub async fn execute(self) -> Result<String, Error> {
        get(&self.url).await
    }
}

// File Job  
pub struct FileJob {
    path: String,
}

impl FileJob {
    pub async fn execute(self) -> Result<String, Error> {
        let content = smol::unblock(|| std::fs::read_to_string(&self.path)).await?;
        Ok(content)
    }
}

// Usage
let mut task: AsyncTask<String> = AsyncTask::new();

// Can easily switch between job types
task.spawn(HttpJob::new(url).execute());
task.spawn(FileJob::new(path).execute());
```

## Benefits of This Design

✅ **Complete Separation**: AsyncTask knows nothing about HTTP, files, or specific operations
✅ **Type Safety**: Each task has a specific output type (`AsyncTask<String>`, `AsyncTask<i32>`, etc.)
✅ **Flexibility**: Can execute any async operation, not just network requests
✅ **Testability**: Easy to mock different job types
✅ **Extensibility**: Add new job types without touching the async executor
✅ **Cancellation**: Clean cancellation via channel closure
✅ **No Polling**: Event-driven communication only

## Architecture

```
[App]           [Jobs]           [AsyncTask<T>]        [Background Thread]
  |                |                    |                      |
  |---creates---->|                    |                      |
  |                |---provides-------->|                      |
  |                |   future           |                      |
  |                |                    |---spawns------------>|
  |                |                    |                      |---executes future
  |                |                    |                      |
  |<--events-------|<--sends events----|<--sends events-------|
```

The AsyncTask is just a generic async executor that bridges sync UI code with async background work.