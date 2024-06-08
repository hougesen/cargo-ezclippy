use schemars::JsonSchema;

#[derive(
    Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, JsonSchema,
)]
pub enum LintLevel {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "deny")]
    Deny,
}
