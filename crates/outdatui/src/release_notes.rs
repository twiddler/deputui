use crate::github::GitHubRepo;
use anyhow::{anyhow, Result};
use common::release::Release;
use std::ops::Deref;
use url::Url;

/// We cannot impl structs from other crates, so we resort to a newtype for adding functionality to Release. The Deref trait provides transparent access to the inner Release.
pub struct ReleaseWithNotes<'a>(pub &'a Release);

impl<'a> ReleaseWithNotes<'a> {
    pub async fn fetch_release_notes(&self) -> Result<Option<String>> {
        if Url::parse(&self.0.repository_url).is_err() {
            return Ok(Some("Invalid repository URL".to_string()));
        }

        if !is_github_url(&self.0.repository_url) {
            return Ok(Some(
                "Only GitHub repositories are supported for release notes".to_string(),
            ));
        }

        match GitHubRepo::from_github_url(&self.0.repository_url) {
            Ok(github_repo) => {
                match github_repo.fetch_release_by_version(&self.0.semver).await {
                    Ok(Some(release)) => Ok(release.body),
                    Ok(None) => Ok(None), // No release found
                    Err(e) => Err(anyhow!("Failed to fetch release notes: {}", e)),
                }
            }
            Err(_) => {
                panic!("Invalid GitHub URL; this should have been caught earlier")
            }
        }
    }
}

// Implement Deref to provide transparent access to inner Release methods
impl<'a> Deref for ReleaseWithNotes<'a> {
    type Target = Release;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn is_github_url(url_str: &str) -> bool {
    if let Ok(url) = Url::parse(url_str) {
        url.host_str() == Some("github.com")
    } else {
        false
    }
}
