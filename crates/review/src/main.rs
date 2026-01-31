use anyhow::Result;
use clap::Parser;

mod args;

use crate::args::Args;

fn main() -> Result<()> {
    Args::parse();

    let releases = deputui_review::parse_stdin()?;

    let selected_packages = smol::block_on(deputui_review::run_review_tui(releases))?;

    println!("{}", selected_packages.join(" "));

    Ok(())
}
