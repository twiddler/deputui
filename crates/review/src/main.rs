use anyhow::Result;

fn main() -> Result<()> {
    let releases = deputui_review::parse_stdin()?;

    let selected_packages = smol::block_on(deputui_review::run_review_tui(releases))?;

    println!("{}", selected_packages.join(" "));

    Ok(())
}
