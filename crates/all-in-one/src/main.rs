use anyhow::Result;
use clap::Parser;
use deputui_pnpm::{fetch_all_releases, parse_input};
use deputui_review::run_review_tui;

mod args;

use crate::args::Args;

fn main() -> Result<()> {
    Args::parse();

    let parsed = parse_input()?;

    let selected_packages = smol::block_on(async {
        let releases = fetch_all_releases(parsed).await?;
        run_review_tui(releases).await
    })?;

    println!("{}", selected_packages.join(" "));

    Ok(())
}
