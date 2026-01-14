#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct Release {
    pub package: String,
    pub semver: String,
}

impl ToString for Release {
    fn to_string(&self) -> String {
        format!("{}@{}", self.package, self.semver)
    }
}

impl Ord for Release {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.package.cmp(&other.package) {
            std::cmp::Ordering::Equal => self.semver.cmp(&other.semver),
            other => other,
        }
    }
}

impl PartialOrd for Release {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
