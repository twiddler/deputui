use anyhow::Result;
use serde_json;
use smol::block_on;

use deputui_pnpm::{fetch_all_releases, parse_input};

fn main() -> Result<()> {
    let parsed = parse_input()?;

    let mut releases = block_on(fetch_all_releases(parsed))?;
    releases.sort();

    let json_output = serde_json::to_string(&releases)?;
    println!("{}", json_output);

    Ok(())
}
