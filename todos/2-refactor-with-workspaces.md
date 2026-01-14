# Implementation Checklist for "outdatui restructuring - modular architecture"

- [x] **Create Cargo workspace structure**
  - Set up workspace with separate binary crates
  - Create list binary with "hello world" smoke test
  - **Success**: Binary compiles and echoes "hello world"

- [x] **Create common library crate**
  - Create a dedicated library crate for shared functionality
  - Move `Release` struct from `src/app.rs` to the common library
  - Modify `NpmPackage.fetch_releases()` to return `Vec<common::Release>` instead of `app::Release`
  - **Success**: Common library crate compiles and exports all shared functionality

- [x] **Implement serialization and deserialization for interfacing between the binaries**
  - Add serde serialization/deserialization support to `Release` in common library
  - Add a test that checks: Release → string → Release matches the original release
  - Add a test that checks: string → Release → string matches the original string
  - **Success**: Tests pass

- [x] **Implement pnpm input parsing in list-builder**
  - Parse `pnpm outdated --format json` from stdin using existing pnpm parsing logic
  - **Success**: Program can parse pnpm output from stdin

- [x] **Implement release fetching logic in list-builder**
  - Move existing `fetch_all_releases()` logic from current `src/main.rs` to list binary
  - **Success**: Program fetches all minor releases from NPM registry

- [x] **Implement JSON output serialization in list-builder**
  - Serialize `Vec<common::Release>` to stdout using serde_json
  - Format output as JSON array (not pretty-printed for pipeline usage)
  - **Success**: Program outputs valid JSON array of common::Release structs

- [x] **Implement JSON input parsing in main binary**
  - Read JSON array from stdin and deserialize to `Vec<common::Release>` with serde
  - Preserve all existing TUI behavior and user experience
  - **Success**: TUI works identically to current version but reads JSON input

- [x] **Update workspace build configuration**
  - Configure workspace to build binaries with correct names (outdat-list, outdatui)
  - Ensure all binaries build correctly with `cargo build --workspace`
  - **Success**: `cargo build --release --workspace` produces both binaries

- [x] **Test complete workflow end-to-end**
  - Test `pnpm outdated --format json | outdat-list` produces valid JSON
  - Test `pnpm outdated --format json | outdat-list | outdatui` works end-to-end
  - **Success**: All usage patterns work as expected

- [x] **Update documentation and examples**
  - Update README.md with new usage examples and architecture description
  - Document the JSON format for Release structs from the common library
  - Add examples of chaining programs and potential filtering
  - Update installation/build instructions for workspace
  - **Success**: Documentation accurately reflects new modular structure

- [x] **Organize Cargo.toml files**
  - Move binary-specific dependencies to their respective crate Cargo.toml files (e.g. ratatui, tui-markdown, etc.)
  - Configure workspace dependencies for the common library
  - **Success**: cargo check exits with 0
