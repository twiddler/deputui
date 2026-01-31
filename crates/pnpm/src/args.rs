use clap::Parser;

/// Reads JSON from stdin (e.g. from `pnpm outdated --format json`), queries the NPM registry for each outdated package, filters to minor version updates only, and outputs release information as JSON to stdout.
#[derive(Parser)]
#[command(name = "deputui-pnpm")]
#[command(version)]
#[command(after_help = "EXAMPLES:
    # Basic usage - pipe pnpm output directly:
    pnpm outdated --format json | deputui-pnpm

    # Save releases for later review:
    pnpm outdated --format json | deputui-pnpm > releases.json

    # Filter with jq before reviewing:
    pnpm outdated --format json | deputui-pnpm | jq 'select(.package | startswith(\"@types\"))'")]
pub struct Args {}
