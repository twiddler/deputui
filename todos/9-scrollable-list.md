# Implementation Checklist for "Releases List Scroll" - Auto-Scroll When Cursor Moves Off-Screen

## Overview

The TUI has a releases list column on the left. When there are more releases than the visible area can display, items at the bottom are cut off. Currently, even when the cursor (focused item) moves to an off-screen item via `j`/`k` navigation, the list does not scroll to keep the focused item visible.

**Goal**: When the user navigates with `j` (down) or `k` (up) and the cursor moves to an item that would be off-screen, the list should automatically scroll to keep the focused item visible.

**Solution**: Use ratatui's `ListState` which automatically manages scroll offset based on the selected index. When `List` is rendered as a `StatefulWidget` with a `ListState`, ratatui will adjust the visible window to ensure the selected item is always visible.

## Todos

- [x] **Add `ListState` import and field to `MultiSelect` struct**
  - File: `crates/review/src/multi_select.rs`
  - Add `ListState` to the imports from `ratatui::widgets`
  - Add `StatefulWidget` to the imports from `ratatui::widgets`
  - Add a new field `list_state: ListState` to the `MultiSelect<T>` struct
  - In `MultiSelect::new()`, initialize `list_state` with `ListState::default().with_selected(Some(0))` to start with the first item selected
  - **Why**: `ListState` tracks both the scroll offset and selected index; ratatui uses this to auto-scroll during render

- [x] **Sync `ListState.selected` with cursor on navigation**
  - File: `crates/review/src/multi_select.rs`
  - In `previous()` method: after updating `self.cursor`, add `self.list_state.select(Some(self.cursor));`
  - In `next()` method: after updating `self.cursor`, add `self.list_state.select(Some(self.cursor));`
  - **Why**: `ListState.selected` must match `cursor` so ratatui knows which item should be visible

- [x] **Add getter method for mutable `ListState` access**
  - File: `crates/review/src/multi_select.rs`
  - Add a new public method:
    ```rust
    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list_state
    }
    ```
  - **Why**: `MultiSelectView` needs mutable access to pass `ListState` to `StatefulWidget::render()`

- [x] **Change `MultiSelectView` to accept mutable reference**
  - File: `crates/review/src/multi_select.rs`
  - Change the `multi_select` field from `&'a MultiSelect<T>` to `&'a mut MultiSelect<T>`
  - **Why**: `StatefulWidget::render()` requires `&mut ListState`, which means we need mutable access to `MultiSelect`

- [x] **Use `StatefulWidget::render` instead of `Widget::render` in `MultiSelectView`**
  - File: `crates/review/src/multi_select.rs`
  - In the `Widget::render` implementation for `MultiSelectView`:
  - Replace:
    ```rust
    List::new(list_items).block(self.block).render(area, buf);
    ```
  - With:
    ```rust
    StatefulWidget::render(
        List::new(list_items).block(self.block),
        area,
        buf,
        self.multi_select.list_state_mut(),
    );
    ```
  - **Why**: `StatefulWidget::render` uses `ListState` to automatically adjust the scroll offset to keep the selected item visible

- [x] **Change `Widget` impl for `App` to use mutable reference**
  - File: `crates/review/src/app.rs`
  - Change `impl Widget for &App` to `impl Widget for &mut App`
  - Update the `MultiSelectView` instantiation to use `&mut self.multiselect` instead of `&self.multiselect`
  - **Why**: Since `MultiSelectView` now requires `&mut MultiSelect`, `App` must provide mutable access

- [x] **Update render call in `run_app_async` to pass mutable reference**
  - File: `crates/review/src/lib.rs`
  - Line 92: Change `frame.render_widget(&*app, frame.area())` to `frame.render_widget(&mut *app, frame.area())`
  - **Why**: The `Widget` impl now expects `&mut App`, so the render call must pass a mutable reference

- [x] **Verify the implementation works correctly**
  - Run `cargo build` to ensure no compilation errors
  - Run `cargo test` to ensure existing tests pass
  - Manually test the TUI with a list of releases longer than the visible area:
    - Navigate down with `j` past the visible area - list should scroll to keep cursor visible
    - Navigate up with `k` past the visible area - list should scroll to keep cursor visible
    - Verify the cursor indicator (`>`) is always visible on screen
  - **Success**: The focused item is always visible regardless of list length or navigation
