# Implementation Checklist for "fetch-release-notes"

- [x] **Enhance NpmVersion struct to include repository information from NPM registry**
  - Update `NpmVersion` struct in `crates/common/src/npm_registry.rs` to include optional `repository` field
  - Add `Repository` struct with `url` and `type` fields to match NPM registry format
  - Handle optional repository field gracefully (not all packages have repositories)
  - **Success**: NpmVersion can parse repository information from NPM registry responses

- [x] **Update Release struct to include repository URL**
  - Add optional `repository_url` field to `Release` struct in `crates/common/src/release.rs`
  - Update serialization support for the new field
  - Ensure backward compatibility with existing code
  - **Success**: Release struct can carry repository information

- [x] **Update NpmPackage::fetch_releases to populate repository URLs**
  - Modify `fetch_releases` to extract repository URL from version info
  - Update Release creation to include repository URL when available
  - Handle cases where repository information is missing
  - **Success**: Releases created by outdat-list contain repository URLs when available

- [x] **Create release_notes module with unified fetch function**
  - Create `crates/outdatui/src/release_notes.rs` module (moved from common as requested)
  - Create `ReleaseWithNotes` newtype following Rust-by-Example pattern with tuple struct `pub struct ReleaseWithNotes<'a>(pub &'a Release)`
  - Implement `Deref` trait for transparent access to Release methods (e.g., `release_with_notes.package`, `release_with_notes.to_string()`)
  - Move entire `fetch_release_notes` logic into `ReleaseWithNotes::fetch_release_notes()` method - no standalone stub function
  - Drop `new()` method for better ergonomics - use tuple constructor `ReleaseWithNotes(&release)` directly
  - Move `is_github_url()` out as standalone function - it doesn't need to be instance method
  - Add comprehensive comment explaining orphan rule and why we use newtype pattern instead of direct impl
  - Integrate with existing GitHub module for GitHub repositories
  - Return appropriate results for both success and failure cases
  - **Success**: Well-documented newtype pattern with clear rationale for design choice

- [x] **Implement non-GitHub repository handling in release_notes module**
  - Add logic to detect non-GitHub repository URLs
  - Return "Only GitHub repositories are supported" message for non-GitHub repos
  - Ensure graceful handling of malformed URLs
  - **Success**: Non-GitHub repositories show appropriate unsupported message
