use anyhow::{anyhow, bail, Result};
use http_types::{Method, Request};
use serde::{Deserialize, Serialize};
use smol::io::AsyncReadExt;
use std::env;
use url::Url;

use common::async_h1_client;

#[derive(Debug, Clone)]
pub struct GitHubRepo {
    pub owner: String,
    pub repo: String,
}

impl GitHubRepo {
    pub fn from_github_url(url_str: &str) -> Result<GitHubRepo> {
        let url = Url::parse(url_str)?;

        if url.host_str() != Some("github.com") {
            bail!("Not a GitHub URL: {}", url_str);
        }

        let path_parts: Vec<&str> = url
            .path_segments()
            .ok_or_else(|| anyhow!("Invalid URL path: {}", url_str))?
            .collect();

        if path_parts.len() < 2 {
            bail!("URL path must contain owner and repo: {}", url_str);
        }

        let owner = path_parts[0].to_string();
        if owner.is_empty() {
            bail!("Empty owner name in URL: {}", url_str);
        }

        let repo = path_parts[1].to_string();
        let repo = repo.trim_end_matches(".git").to_string();
        if repo.is_empty() {
            bail!("Empty repo name in URL: {}", url_str);
        }

        Ok(GitHubRepo { owner, repo })
    }

    pub async fn fetch_release(&self, tag: &str) -> Result<GitHubRelease> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/tags/{}",
            self.owner, self.repo, tag
        );

        let mut req = Request::new(Method::Get, Url::parse(&url)?);
        req.insert_header("User-Agent", "deputui-review/0.1.0");
        req.insert_header("Accept", "application/vnd.github.v3+json");

        if let Ok(token) = env::var("DEPUTUI_GITHUB_TOKEN") {
            req.insert_header("Authorization", format!("Bearer {}", token));
        }

        let mut resp = async_h1_client::fetch(req).await?;

        if resp.status() != 200 {
            bail!("GitHub API error: {}", resp.status());
        }

        let mut body = Vec::new();
        resp.read_to_end(&mut body).await?;
        let response_text = String::from_utf8_lossy(&body);

        serde_json::from_str(&response_text)
            .map_err(|e| anyhow!("Failed to parse GitHub API response: {}", e))
    }

    pub async fn fetch_release_by_version(&self, version: &str) -> Result<GitHubRelease> {
        let tags_to_try = vec![version.to_string(), format!("v{}", version)];

        for tag in tags_to_try {
            match self.fetch_release(&tag).await {
                Ok(release) => return Ok(release),
                Err(_) => continue, // Try next tag format
            }
        }

        bail!("No release found")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubRelease {
    pub url: String,
    pub html_url: String,
    pub id: u64,
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub author: GitHubAuthor,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubAuthor {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub html_url: String,
    #[serde(default)]
    pub r#type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_github_urls() {
        let repo = GitHubRepo::from_github_url("https://github.com/rust-lang/rust.git").unwrap();
        assert_eq!(repo.owner, "rust-lang");
        assert_eq!(repo.repo, "rust");
    }

    #[test]
    fn test_parse_invalid_github_urls() {
        let invalid_cases = vec![
            "https://foo.bar/repo",
            "https://github.com",
            "https://github.com/owner",
            "https://github.com//repo",
            "not-a-url",
            "",
            "https://gitlab.com/user/repo",
        ];

        for url in invalid_cases {
            assert!(
                GitHubRepo::from_github_url(url).is_err(),
                "Should fail for URL: {}",
                url
            );
        }
    }
}
