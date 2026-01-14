`outdatui` is a little TUI that helps you review minor version updates of NPM dependencies.

# Motivation

Reviewing version updates of NPM dependencies can be quite tedious. Run `pnpm outdated`, go to the repositories, the release tags, and hunt down the relevant ones. That's no fun. 🙁

When dependencies follow the semver semantics properly, we know that

- **Major versions** have breaking changes. Migrating to that version might take minutes or days. These are released infrequently.
- **Minor versions** have new features. We want to review those and see whether we can use the new features in our own code. These are released regularly.
- **Patch versions** have bug fixes only. We can always update those, essentially without reviewing. (You should defend against malicious updates with [`minimumReleaseAge`](https://pnpm.io/settings#minimumreleaseage) or similar measures.) These are released frequently.

So minor version updates are those that we must review and will be reviewing most often, so it'd be great to do fast and conveniently. This is what `outdatui` is for. 🏎️💨

# How to use

Pipe your pnpm output into this:

```console
$ pnpm outdated --format json | outdat-list | outdatui
```

Then, in `outdatui`, review the minor version updates and select those you want to update to with <kbd>Space</kbd>. When you're done, hit <kbd>Enter</kbd> to confirm. The `package@version` identifiers you selected will be printed to stdout.

If you want to update to the selected releases, you can pipe the output back to pnpm:

```console
$ pnpm outdated --format json | outdat-list | outdatui | xargs pnpm update
```

# What about npm, yarn, …?

If you're using a different package manager than pnpm, you can still use this! You only need to transform its output to a JSON dictionary that matches this schema:

```json
{
  "foo": { "current": "1.0.0", "wanted": "1.0.1" },
  "bar": { "current": "2.0.0", "wanted": "2.3.2" },
  …
}
```

After that, you can pipe it to `outdat-list | outdatui` just like you would if you were using pnpm. Pretty pipes! 🪠

# Architecture and Advanced Usage

This project uses a modular architecture with two main binaries:

- **`outdat-list`**: Reads `pnpm outdated --format json`, fetches minor version updates from the NPM registry and outputs releases to review
- **`outdatui`**: The TUI application for reviewing and selecting updates

The simple usage shown above handles both steps. For advanced workflows, you can also:

```console
# Save release list for later review
$ pnpm outdated --format json | outdat-list > releases.json
$ cat releases.json | outdatui

# Filter releases with custom tools
$ pnpm outdated --format json | outdat-list | jq 'select(.package | startswith("@types"))' | outdatui
```

## JSON Format

The intermediate format returned by `outdat-list` is an array of release objects:

```json
[
  { "package": "foo", "semver": "1.1.0" },
  { "package": "foo", "semver": "1.2.0" },
  { "package": "bar", "semver": "2.2.2" }
]
```

# Installation

Build from source:

```console
$ cargo build --release --workspace
```

This creates `outdatui` and `outdat-list` binaries in `target/release/`.
