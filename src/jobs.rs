use crate::app::Release;
use std::time::Duration;

pub struct ReleaseNotesJob {
    release: Release,
}

impl ReleaseNotesJob {
    pub fn new(release: Release) -> Self {
        Self { release }
    }

    pub async fn execute(self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Simulate network delay for demo
        smol::Timer::after(Duration::from_millis(500)).await;
        
        // For now, return mock content based on package
        // In production, you'd use: get(&self.url).await
        match self.release.package.as_str() {
            "foo" => Ok("# Release Notes for foo@1.0.0\n\n- Added new feature\n- Fixed bugs".to_string()),
            "bar" => Ok("# Release Notes for bar@2.2.2\n\n- Major update\n- Breaking changes".to_string()),
            _ => Ok(format!("# Release Notes for {}@{}\n\nNo detailed notes available.", 
                           self.release.package, self.release.semver))
        }
    }
}

// Example of other job types that could be added:
pub struct FileReadJob {
    path: String,
}

impl FileReadJob {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub async fn execute(self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Use smol::unblock for file I/O
        let content = smol::unblock(move || {
            std::fs::read_to_string(&self.path)
        }).await?;
        
        Ok(content)
    }
}