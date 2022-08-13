use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Severity {
    r#type: String,
    score: String,
}

#[derive(Serialize, Deserialize)]
struct Package {
    ecosystem: String,
    name: String,
    purl: String,
}

#[derive(Serialize, Deserialize)]
struct Ranges {
    r#type: String,
    repo: String, 
    events: Vec<Events>, 
    database_specific: Vec<DatabaseSpecific>,
}

#[derive(Serialize, Deserialize)]
struct Events {
    introduced: String,
    fixed: String,
    last_affected: String,
    limit: String,
}

#[derive(Serialize, Deserialize)]
struct EcosystemSpecific {
    source: String,
}

#[derive(Serialize, Deserialize)]
struct DatabaseSpecific {
    source: String,
}

#[derive(Serialize, Deserialize)]
struct Affected {
    package: Vec<Package>,
    ranges: Vec<Ranges>,
    versions: Vec<String>,
    ecosystem_specific: Vec<EcosystemSpecific>,
}

#[derive(Serialize, Deserialize)]
struct References {
    r#type: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Credits {
    name: String,
    contact: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct OvfFormat {
    schema_version: String,
    id: String,
    modified: String,
    published: String,
    withdrawn: String,
    aliases: Vec<String>,
    related: Vec<String>,
    summary: String,
    details: String,
    severity: Vec<Severity>,
    affected: Vec<Affected>,
    references: References,
    credits: Credits,

}