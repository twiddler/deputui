# Implementation Checklist for "fetch-list-of-releases-from-npmjs"

- [x] **Create npm_registry.rs module with serde structures for NPM package metadata**
  - An example response for package `lodash` from registry.npmjs.org looks like this:

    ```json
    {
      "name": "lodash",
      "versions": {
        "4.17.21": {
          "name": "lodash",
          "version": "4.17.21",
          "homepage": "https://lodash.com/",
          "repository": {
            "url": "git+https://github.com/lodash/lodash.git",
            "type": "git"
          }
        }
      }
    }
    ```

  - Define `NpmPackage` struct with `versions: BTreeMap<String, NpmVersion>` field
  - Define `NpmVersion` struct with `name`, `version` fields (do not use `#[serde(default)]` for optional fields). `NpmPackage.versions` is a `Map<String, NpmVersionInfo>`. Version strings are object keys (not in an array).
  - Add serde deserialization support
  - Handle JSON parsing errors → return Err for graceful handling
  - **Success**: Serde can successfully deserialize the actual NPM registry JSON response

- [x] **Implement NpmPackage::fetch() function using existing async_h1_client**
  - Use `https://registry.npmjs.org/{package-name}` endpoint
  - Return `NpmPackage` or error
  - Leverage existing `async_h1_client::get()` function for HTTP requests
  - Handle network errors → return Err for graceful handling
  - **Success**: Function can fetch and parse real package metadata from NPM registry

- [x] **Add NpmPackage.iter_versions() function to get version list from NPM response**
  - Returns an iterator over version strings (Version strings are `NpmPackage.versions.keys()`).
  - **Success**: Function returns an iterator over version strings

- [x] **Update main.rs to fetch releases from NPM instead of using hardcoded data**
  - Replace hardcoded releases in `main.rs:26-32`
  - Call NPM fetching for each package from pnpm input
  - Integrate with existing semver filtering logic
  - Maintain existing async pattern with smol runtime
  - Use existing channel pattern for TUI re-render triggers
  - **Success**: TUI displays actual available releases instead of hardcoded ones
