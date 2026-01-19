use crate::github::GitHubRepo;
use anyhow::{anyhow, bail, Result};
use common::release::Release;
use std::ops::Deref;
use url::Url;

/// We cannot impl structs from other crates, so we resort to a newtype with Deref for adding functionality to Release.
pub struct ReleaseExt<'a>(pub &'a Release);

impl<'a> ReleaseExt<'a> {
    pub async fn fetch_release_notes(&self) -> Result<String> {
        if Url::parse(&self.0.repository_url).is_err() {
            bail!("Invalid repository URL".to_string());
        }

        if !is_github_url(&self.0.repository_url) {
            bail!("Only GitHub repositories are supported for release notes".to_string());
        }

        match GitHubRepo::from_github_url(&self.0.repository_url) {
            Ok(github_repo) => match github_repo.fetch_release_by_version(&self.0.semver).await {
                Ok(release) => Ok(release.body.unwrap_or("Empty release notes".into())),
                Err(e) => Err(anyhow!("Failed to fetch release notes: {}", e)),
            },
            Err(_) => {
                panic!("Invalid GitHub URL; this should have been caught earlier")
            }
        }
    }
}

impl<'a> Deref for ReleaseExt<'a> {
    type Target = Release;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn is_github_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(url) => url.host_str() == Some("github.com"),
        _ => false,
    }
}
