use anyhow::Result;
use common::{npm_registry::NpmPackage, release::Release, semver::Semver};
use futures::future::try_join_all;

pub mod pnpm;

pub use crate::pnpm::{parse_input, PnpmOutdatedOutput, PnpmOutdatedPackage};

pub async fn fetch_all_releases(parsed: PnpmOutdatedOutput) -> Result<Vec<Release>> {
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
