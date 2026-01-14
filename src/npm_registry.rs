use crate::{app::Release, async_h1_client, semver::Semver};
use anyhow::Result;
use serde::Deserialize;
use std::{collections::BTreeMap, error::Error};

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

    pub async fn fetch_releases(
        &self,
        current: Semver,
        latest: Semver,
    ) -> Result<Vec<Release>, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let json = r#"
        {
            "name": "test-package",
            "versions": {
                "1.0.0": {
                    "name": "test-package",
                    "version": "1.0.0"
                }
            }
        }
        "#;

        let result: Result<NpmPackage, _> = serde_json::from_str(json);
        assert!(
            result.is_ok(),
            "Failed to deserialize NPM package: {:?}",
            result
        );

        let package = result.unwrap();
        assert_eq!(package.name, "test-package");

        let version = package.versions.get("1.0.0").unwrap();
        assert_eq!(version.name, "test-package");
        assert_eq!(version.version, "1.0.0");
    }
}
