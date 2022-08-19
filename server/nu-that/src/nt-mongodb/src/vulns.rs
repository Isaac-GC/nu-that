use serde::{Deserialize, Serialize};


#[non_exhaustive]
pub enum PackageType {
    Go,

    #[serde(rename="npm")]
    Npm,

    #[serde(rename="OSS-Fuzz")]
    OssFuzz,
    PyPI,
    RubyGems,

    #[serde(rename="crates.io")]
    CratesIO,
    Packagist,
    Maven,
    NuGet,
    Linux,
    Debian,
    Hex,
    Android,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct VulnerabilityRecord {
    // The repo/package type of the Vulnerability
    pub package_type: PackageType,
    // The CVE of the vulnerability (used as the id)
    pub vuln: String,
    // A list of vulnerable versions to check against
    pub vulnerable_versions: Vec<String>,
    // The date the vulnerability was published
    pub published_date: String,
    // The date the vulnerability data was updated/modified
    pub modified_date: String,
}

