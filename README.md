`outdatui` is a little TUI for reviewing minor version updates of NPM dependencies at the speed of light and with a smile. 🦸

# Motivation

Reviewing version updates of NPM dependencies can be quite tedious. Run `pnpm outdated`, go to the repositories, the release tags, and hunt down the relevant ones. That's no fun. 🙁

When dependencies follow the semver semantics properly, we know that

- **Major versions** have breaking changes. Migrating to that version might take minutes or days. These are released infrequently.
- **Minor versions** have new features. We want to review those and see whether we can use the new features in our own code. These are released regularly.
- **Patch versions** have bug fixes only. We can always update those, essentially without reviewing. (You should defend against malicious updates with [`minimumReleaseAge`](https://pnpm.io/settings#minimumreleaseage) or similar measures.) These are released frequently.

So minor version updates are those that we must review and will be reviewing most often, so it'd be great to make that process fast and convenient. This is what `outdatui` is for. 🏎️💨

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

## Via flake.nix

If you want to use this in an SSTv2 project that has a `flake.nix`, this is the minimal setup for having `outdat-list` and `outdatui` in your dev shell:

```
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    outdatui.url = "git+ssh://git@github.com/twiddler/outdatui.git";
  };

  outputs = { self, nixpkgs, outdatui }:
    let
      system = "x86_64-linux";

      pkgs = import nixpkgs {
        inherit system;
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        name = "node-dev-shell";

        buildInputs = with pkgs; [
          outdatui.packages.${system}.oudat-list
          outdatui.packages.${system}.outdatui
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

You might want to try `jq` for transforming. After that, you can pipe to `some-package-manager outdated | jq <your transform> | outdat-list | outdatui` just like you would if you were using pnpm. Pretty pipes! 🪠

# Advanced Usage

This project uses a modular architecture with two main binaries:

- **`outdat-list`**: Reads `pnpm outdated --format json`, fetches minor version updates from the NPM registry and outputs releases to review
- **`outdatui`**: The TUI application for reviewing and selecting updates

This allows for advanced workflows, e.g.:

## Save release list for later review

```console
$ pnpm outdated --format json | outdat-list > releases.json
$ cat releases.json | outdatui
```

## Filter releases with custom tools

```console
$ pnpm outdated --format json | outdat-list | jq 'select(.package | startswith("@types"))' | outdatui
```
