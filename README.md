<img src="./logo.svg" alt="Alt Text" width="100" />

`deputui` is a little TUI for reviewing minor version updates of NPM dependencies at the speed of light and with a smile. 🦸

# Motivation

Reviewing version updates of NPM dependencies can be quite tedious. Run `pnpm outdated`, go to the repositories, the release tags, and hunt down the relevant ones. That's no fun. 🙁

When dependencies follow the semver semantics properly, we know that

- **Major versions** have breaking changes. Migrating to that version might take minutes or days. These are released infrequently.
- **Minor versions** have new features. We want to review those and see whether we can use the new features in our own code. These are released regularly.
- **Patch versions** have bug fixes only. We can always update those, essentially without reviewing. (You should defend against malicious updates with [`minimumReleaseAge`](https://pnpm.io/settings#minimumreleaseage) or similar measures.) These are released frequently.

So minor version updates are those that we must review and will be reviewing most often, so it'd be great to make that process fast and convenient. This is what `deputui` is for. 🏎️💨

# How to use

Pipe your pnpm output directly into `deputui`:

```console
$ pnpm outdated --format json | deputui
```

Then, in `deputui`, review the minor version updates and select those you want to update to with <kbd>Space</kbd>. When you're done, hit <kbd>Enter</kbd> to confirm. The `package@version` identifiers you selected will be printed to stdout.

If you want to update to the selected releases, you can pipe the output back to pnpm:

```console
$ pnpm outdated --format json | deputui | xargs pnpm update
```

# Configuration

To avoid getting rate-limited by GitHub when fetching release information, you can set the `DEPUTUI_GITHUB_TOKEN` environment variable with your [GitHub personal access token](https://github.com/settings/personal-access-tokens):

```console
$ export DEPUTUI_GITHUB_TOKEN=your_github_token_here
$ pnpm outdated --format json | deputui | xargs pnpm update
```

# Installation

## Manually

If you're using nix, run

```console
$ nix develop
```

You will enter a nix shell that has access to all the binaries in this project, including the rust toolchain.

If you're not using nix, reconsider your life decisions, then [install rust](https://rust-lang.github.io/rustup/) and

```console
$ make all
```

This will build and install all three binaries:

- `deputui` (all-in-one binary)
- `deputui-pnpm` (fetching binary)
- `deputui-review` (TUI binary)

## Via flake.nix

If you want to use this in an SSTv2 project that has a `flake.nix`, this is the minimal setup for having the binaries in your dev shell:

```
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
     deputui.url = "git+ssh://git@github.com/twiddler/deputui.git";
  };

   outputs = { self, nixpkgs, deputui }:
    let
      system = "x86_64-linux";

      pkgs = import nixpkgs {
        inherit system;
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        name = "dev-shell";

        buildInputs = with pkgs; [
          deputui.packages.${system}.deputui
        ];
      };
    };
}
```

# What about npm, yarn, …?

If you're using a package manager other than pnpm, you can still use this! You only need to transform its output to a JSON dictionary that matches this schema:

```json
{
  "foo": { "current": "1.0.0", "wanted": "1.0.1" },
  "bar": { "current": "2.0.0", "wanted": "2.3.2" }
}
```

You might want to try `jq` for transforming. After that, you can pipe to `some-package-manager outdated | jq <your transform> | deputui` just like you would if you were using pnpm. Pretty pipes! 🪠

# Advanced Usage

This project provides both an all-in-one binary and a modular architecture:

- **`deputui`**: All-in-one binary that handles parsing, fetching, and review in one command
- **`deputui-pnpm`**: Reads `pnpm outdated --format json`, fetches minor version updates from the NPM registry and outputs releases to review
- **`deputui-review`**: The TUI application for reviewing and selecting updates

## Save release list for later review

```console
$ pnpm outdated --format json | deputui-pnpm > releases.json
$ cat releases.json | deputui-review
```

## Filter releases with custom tools

```console
$ pnpm outdated --format json | deputui-pnpm | jq 'select(.package | startswith("@types"))' | deputui-review
```
