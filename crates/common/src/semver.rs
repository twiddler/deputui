use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Semver {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Debug, PartialEq)]
pub enum SemverParsingError {
    Prerelease,
    InvalidFormat,
    InvalidNumber(&'static str),
}

impl fmt::Display for SemverParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemverParsingError::Prerelease => write!(f, "pre-release versions are not supported"),
            SemverParsingError::InvalidFormat => write!(f, "invalid semver format"),
            SemverParsingError::InvalidNumber(part) => write!(f, "invalid {} number", part),
        }
    }
}

impl std::error::Error for SemverParsingError {}

impl std::str::FromStr for Semver {
    type Err = SemverParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Handle pre-release versions like "1.0.0-alpha"
        if s.contains('-') {
            return Err(SemverParsingError::Prerelease);
        }

        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(SemverParsingError::InvalidFormat);
        }

        let major = parts[0]
            .parse()
            .map_err(|_| SemverParsingError::InvalidNumber("major"))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| SemverParsingError::InvalidNumber("minor"))?;
        let patch = parts[2]
            .parse()
            .map_err(|_| SemverParsingError::InvalidNumber("patch"))?;

        Ok(Semver {
            major,
            minor,
            patch,
        })
    }
}

impl fmt::Display for Semver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Semver {
    pub fn is_minor_update_of(&self, other: &Semver) -> bool {
        self.major == other.major && self.minor > other.minor && self.patch == 0
    }

    pub fn is_at_most(&self, max: &Semver) -> bool {
        self <= max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_equality() {
        let v1 = Semver {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let v2 = Semver {
            major: 1,
            minor: 2,
            patch: 3,
        };
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_semver_inequality() {
        let v1 = Semver {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let v2 = Semver {
            major: 1,
            minor: 2,
            patch: 4,
        };
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_semver_major_comparison() {
        let v1 = Semver {
            major: 1,
            minor: 0,
            patch: 0,
        };
        let v2 = Semver {
            major: 2,
            minor: 0,
            patch: 0,
        };
        assert!(v1 < v2);
        assert!(v2 > v1);
    }

    #[test]
    fn test_semver_minor_comparison() {
        let v1 = Semver {
            major: 1,
            minor: 2,
            patch: 0,
        };
        let v2 = Semver {
            major: 1,
            minor: 3,
            patch: 0,
        };
        assert!(v1 < v2);
        assert!(v2 > v1);
    }

    #[test]
    fn test_semver_patch_comparison() {
        let v1 = Semver {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let v2 = Semver {
            major: 1,
            minor: 2,
            patch: 4,
        };
        assert!(v1 < v2);
        assert!(v2 > v1);
    }

    #[test]
    fn test_semver_complex_comparison() {
        let v1 = Semver {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let v2 = Semver {
            major: 1,
            minor: 2,
            patch: 4,
        };
        let v3 = Semver {
            major: 1,
            minor: 3,
            patch: 0,
        };
        let v4 = Semver {
            major: 2,
            minor: 0,
            patch: 0,
        };

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
        assert!(v1 < v4);
    }

    #[test]
    fn test_from_str_valid() {
        let result: Result<Semver, _> = "1.2.3".parse();
        assert!(result.is_ok());
        let semver = result.unwrap();
        assert_eq!(semver.major, 1);
        assert_eq!(semver.minor, 2);
        assert_eq!(semver.patch, 3);
    }

    #[test]
    fn test_from_str_invalid_prerelease() {
        let result: Result<Semver, _> = "1.2.3-beta.1".parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SemverParsingError::Prerelease);
    }

    #[test]
    fn test_from_str_invalid_format_too_few_parts() {
        let result: Result<Semver, _> = "1.2".parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SemverParsingError::InvalidFormat);
    }

    #[test]
    fn test_from_str_invalid_format_too_many_parts() {
        let result: Result<Semver, _> = "1.2.3.4".parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SemverParsingError::InvalidFormat);
    }

    #[test]
    fn test_from_str_invalid_number_major() {
        let result: Result<Semver, _> = "abc.2.3".parse();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            SemverParsingError::InvalidNumber("major")
        );
    }

    #[test]
    fn test_from_str_invalid_number_minor() {
        let result: Result<Semver, _> = "1.def.3".parse();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            SemverParsingError::InvalidNumber("minor")
        );
    }

    #[test]
    fn test_from_str_invalid_number_patch() {
        let result: Result<Semver, _> = "1.2.ghi".parse();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            SemverParsingError::InvalidNumber("patch")
        );
    }
}
