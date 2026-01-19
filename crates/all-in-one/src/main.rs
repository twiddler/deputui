use anyhow::Result;
use deputui_pnpm::{fetch_all_releases, parse_input};
use deputui_review::run_review_tui;

fn main() -> Result<()> {
    let parsed = parse_input()?;

    let selected_packages = smol::block_on(async {
        let releases = fetch_all_releases(parsed).await?;
        run_review_tui(releases).await
    })?;

    println!("{}", selected_packages.join(" "));

    Ok(())
}
