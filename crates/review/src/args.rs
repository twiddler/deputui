use clap::Parser;

/// Reads JSON from stdin (e.g. from `pnpm outdated --format json | deputui-pnpm`) and renders a TUI for reviewing release notes. Releases can be selected and will be printed to stdout when confirmed.
#[derive(Parser)]
#[command(name = "deputui-review")]
#[command(version)]
#[command(after_help = "EXAMPLES:
    # Basic usage:
    pnpm outdated --format json | deputui-pnpm | deputui-review

    # Install selected releases:
    pnpm outdated --format json | deputui-pnpm | deputui-review | xargs pnpm update

ENVIRONMENT:
    DEPUTUI_GITHUB_TOKEN    GitHub personal access token for authenticated API requests.
                            Unauthenticated requests are limited to 60/hour; authenticated
                            requests are limited to 5,000/hour.")]
pub struct Args {}
