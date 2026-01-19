# Implementation Checklist: deputui All-in-One Binary

- [x] **Create library module for deputui-pnpm crate**
  - Create `crates/deputui-pnpm/src/lib.rs` file
  - Refactor the main function of `crates/deputui-pnpm/src/main.rs` into helper functions
  - Export helper functions as public library functions:

    ```rust
    // lib.rs content:
    use anyhow::Result;
    use common::release::Release;
    use crate::pnpm::PnpmOutdatedOutput;

    pub async fn fetch_all_releases(parsed: PnpmOutdatedOutput) -> Result<Vec<Release>> {
        // Move the async function from main.rs here
    }

    // … other helper functions
    ```

  - Update `main.rs` to use the library functions from `lib.rs`

- [x] **Create library module for deputui-review crate**
  - Create `crates/deputui-review/src/lib.rs` file
  - Export the TUI logic as public library functions:

    ```rust
    // lib.rs content:
    use anyhow::Result;
    use common::release::Release;
    use std::io;

    pub async fn run_review_tui(releases: Vec<Release>) -> Result<Vec<String>> {
        // Main TUI logic extracted from main.rs
        // Returns selected package strings instead of printing them
    }

    pub fn parse_releases_from_json(json_input: &str) -> Result<Vec<Release>> {
        // Move parse_json_input function from main.rs
    }
    ```

  - Extract `run_app_async()` and related TUI setup logic from `main.rs` to `lib.rs`
  - Move `parse_json_input()` function to `lib.rs` as `parse_releases_from_json()`
  - Update the function to return selected releases instead of printing them
  - Update `main.rs` to use the library functions

- [x] **Initiliaze deputui crate**
  - Create `crates/all-in-one/Cargo.toml` with these library dependencies:
    ```toml
    [dependencies]
    anyhow = "1.0.100"
    deputui-pnpm = { path = "../deputui-pnpm" }
    deputui-review = { path = "../deputui-review" }
    ```
  - Ensure the workspace still builds correctly

- [x] **Change deputui main.rs to use library functions**
  - Call library functions:

    ```rust
    // New main.rs structure:
    use anyhow::Result;
    use deputils_npm::{parse_and_fetch_releases_from_json};
    use deputils_review::{run_review_tui};
    use std::io::prelude::*;

    fn main() -> Result<()> {
        // Read pnpm output from stdin
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;

        // Use deputui-pnpm and deputui-review library functions
        let selected_packages = smol::block_on(
          // …
        )?;

        // Print final output
        println!("{}", selected_packages.join(" "));

        Ok(())
    }
    ```
