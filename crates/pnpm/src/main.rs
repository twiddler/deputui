use anyhow::Result;
use futures::future::try_join_all;
use serde_json;
use smol::block_on;

mod pnpm;

use crate::pnpm::{parse_input, PnpmOutdatedOutput};
use common::{npm_registry::NpmPackage, release::Release, semver::Semver};

fn main() -> Result<()> {
    let parsed = parse_input()?;

    let mut releases: Vec<Release> = block_on(fetch_all_releases(parsed))?;

    releases.sort();

    let json_output = serde_json::to_string(&releases)?;
    println!("{}", json_output);

    Ok(())
}

async fn fetch_all_releases(parsed: PnpmOutdatedOutput) -> Result<Vec<Release>> {
    let package_futures: Vec<_> = parsed
        .into_iter()
        .map(async |(package_name, package_info)| {
            let npm_package = NpmPackage::fetch(&package_name).await?;
            let current: Semver = package_info.current.parse()?;
            let latest: Semver = package_info.latest.parse()?;
            let releases = npm_package.fetch_releases(current, latest).await?;
            Ok::<Vec<Release>, anyhow::Error>(releases)
        })
        .collect();

    let all_releases: Vec<Release> = try_join_all(package_futures)
        .await?
        .into_iter()
        .flatten()
        .collect();

    Ok(all_releases)
}
