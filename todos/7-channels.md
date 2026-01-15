# Implementation Checklist for "channels" - Channel-Based Architecture Refactor

## Overview

Complete the channel-based architecture refactor by replacing the current EventSource pattern with a unified channel system for handling both keyboard events and async task notifications. This will simplify the event loop and improve responsiveness.

## Todos

- [x] **Create keyboard event reader task using EventStream**
  - Spawn background task that reads keyboard events using EventStream
  - Send keyboard events through the unified ui_tx channel
  - Ensure the keyboard reader runs continuously without blocking the main loop
  - Use proper error handling for EventStream operations
  - **Success**: Keyboard events are asynchronously sent through unified channel

- [x] **Replace EventSource enum and future::or() logic with unified channel handling**
  - Remove the EventSource enum entirely from main.rs
  - Remove the future::or() pattern for waiting on multiple event sources
  - Simplify the event handling logic to only wait on ui_rx.recv().await
  - Eliminate the complex event source matching logic
  - **Success**: Clean, simple event loop that waits only on unified channel

- [x] **Update App::new() and AsyncTaskRunner to use unified ui_tx instead of render_tx**
  - Change App::new() parameter from render_tx to ui_tx
  - Update AsyncTaskRunner constructor calls throughout the codebase
  - Ensure all notification sends go through the unified ui_tx
  - Verify no references to render_tx remain in the code
  - **Success**: All async components use unified channel for notifications

- [x] **Modify run_app_async function signature to use ui_rx instead of data_ready_rx**
  - Update function parameter from data_ready_rx to ui_rx
  - Update the parameter type annotation to match the unified channel
  - Ensure the main() function call passes the correct receiver
  - **Success**: Function signature reflects unified channel architecture

- [x] **Simplify main event loop to only wait on ui_rx.recv().await**
  - Replace the complex EventSource matching with simple channel receive
  - Remove the nested async blocks for key_event and channel_event
  - Handle all events (keyboard and task notifications) from the channel
  - Maintain the same app.handle_key() logic for keyboard events
  - **Success**: Clean event loop with single channel wait point

- [x] **Handle both keyboard events and task completion notifications in unified manner**
  - Design a simple enum or message type for the unified channel if needed
  - Ensure keyboard events are properly distinguished from task notifications
  - Maintain existing behavior for both types of events
  - Test that keyboard input still triggers the same app responses
  - **Success**: Both event types work seamlessly through unified channel

- [x] **Remove old bounded channel (data_ready_tx, data_ready_rx) creation**
  - Remove any remaining references to the old bounded channels
  - Clean up imports related to the old channel system
  - Ensure no dead code remains from the previous architecture
  - Verify the new unified channel handles all communication
  - **Success**: Old channel system completely removed

- [x] **Clean up unused imports and simplify event handling code**
  - Remove unused imports like EventSource, future::or
  - Clean up any remaining dead code from old architecture
  - Organize imports and remove unnecessary use statements
  - Simplify any complex or redundant code patterns
  - **Success**: Clean, minimal codebase with no unused imports
