use clap::Parser;

/// Reads JSON from stdin (e.g. from `pnpm outdated --format json`), queries the NPM registry for each outdated package, filters to minor version updates only, and renders a TUI for reviewing release notes. Releases can be selected and will be printed to stdout when confirmed.
#[derive(Parser)]
#[command(name = "deputui")]
#[command(version)]
#[command(after_help = "EXAMPLES:
    # Basic usage:
    pnpm outdated --format json | deputui

    # Install selected releases:
    pnpm outdated --format json | deputui | xargs pnpm update")]
pub struct Args {}
