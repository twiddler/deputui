# Implementation Checklist for "show-release-notes"

- [x] **Design AsyncTask for automated testing**
  - Create generic `AsyncTask<T, Input>` struct with clear, testable interfaces
  - Add generic `AsyncTaskStatus<T>` enum (Idle, Loading, Loaded(T), Error(String))
  - Add subscriber mechanism - field to store notification sender (like `render_tx`)
  - Design with dependency injection to enable easy mocking and unit testing
  - AsyncTask should be standalone - independent of App and reusable for any async operation
  - Ensure AsyncTask is modular and reusable, works for any async operation
  - **Success**: Async task pattern that can be easily tested in isolation without UI dependencies

- [x] **Implement AsyncTask with baked-in operation**
  - AsyncTask should store a closure that creates a future from input data
  - Add `start_operation(input)` method that cancels previous task and starts new one
  - Generic task handles its own completion and status updates internally
  - Operation closure is provided on creation, making AsyncTask modular and self-contained
  - Task should be independent of App, easy to understand in isolation
  - **Success**: Task that knows WHAT to do (closure) and gets WHEN to do it (input data)

- [x] **Replace scattered App fields with AsyncTask for release notes**
  - Remove `release_notes`, `release_notes_error`, and `current_release_notes_task` fields
  - Add single `release_notes_task: AsyncTask<String, Release>` field in App struct
  - Provide closure that creates ReleaseWithNotes future from Release input on creation
  - AsyncTask remains generic - App just provides the specific operation logic
  - Update App methods to use unified interface: `release_notes_task.start_operation(release)`
  - Just pass data to start operation - clean interface
  - **Success**: Cleaner App struct with encapsulated async behavior, AsyncTask handles lifecycle

- [x] **Implement subscriber notification pattern**
  - Generic status changes automatically trigger notifications to subscribers
  - App wires main loop to receive task notifications for re-render triggers
  - App handles the connection between AsyncTask and main event loop
  - No manual polling needed - notifications flow naturally
  - Ensure JavaScript-like behavior: Task completion → automatic notification → UI update
  - **Success**: Automatic UI updates when state changes, like JavaScript Promises

- [x] **Update rendering to use unified status**
  - Modify `Widget for &App` to read from `release_notes_task.status`
  - Handle all status states (Idle, Loading, Loaded, Error) in one place
  - Ensure automatic notifications trigger re-renders
  - **Success**: Release notes pane displays appropriate content based on task state

- [x] **Integrate with main event loop**
  - Update `EventSource::Channel` handler to process task notifications
  - Task notifications trigger re-renders automatically
  - Remove manual task completion checking
  - Ensure clean event loop with automatic UI updates
  - **Success**: Clean event loop with automatic UI updates

- [x] **Clean up legacy code**
  - Remove `get_release_notes_of()` stub function
  - Remove unused imports and dead code
  - Ensure all hardcoded release notes logic is removed
  - Verify only production async implementation remains
  - **Success**: Clean codebase with only production async implementation
