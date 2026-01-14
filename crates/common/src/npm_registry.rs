use crate::{async_h1_client, release::Release, semver::Semver};
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct NpmPackage {
    pub name: String,
    pub versions: BTreeMap<String, NpmVersion>,
}

#[derive(Debug, Deserialize)]
pub struct NpmVersion {
    pub name: String,
    pub version: String,
}

impl NpmPackage {
    pub async fn fetch(package_name: &str) -> Result<NpmPackage> {
        let url = format!("https://registry.npmjs.org/{}", package_name);
        let response = async_h1_client::get(&url).await?;

        serde_json::from_str(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse NPM registry response: {}", e))
    }

    fn iter_versions(&self) -> impl Iterator<Item = &String> {
        self.versions.keys()
    }

    pub async fn fetch_releases(&self, current: Semver, latest: Semver) -> Result<Vec<Release>> {
        let all_versions = self
            .iter_versions()
            .filter_map(|version| version.parse::<Semver>().ok());

        let minor_updates = all_versions
            .filter(|semver| semver.is_minor_update_of(&current) && semver.is_at_most(&latest));

        let releases: Vec<Release> = minor_updates
            .map(|semver| Release {
                package: self.name.clone(),
                semver: semver.to_string(),
            })
            .collect();

        Ok(releases)
    }
}
