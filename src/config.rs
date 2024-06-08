use crate::lints::Lint;

fn default_schema_location() -> String {
    "./schema.json".to_owned()
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    #[serde(rename = "$schema", default = "default_schema_location")]
    pub schema: String,

    pub rules: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
}

pub fn build_config(lints: Vec<Lint>) -> Config {
    let mut rules =
        std::collections::HashMap::<String, std::collections::HashMap<String, String>>::new();

    for lint in lints {
        rules
            .entry(lint.group)
            .or_default()
            .insert(lint.id, lint.level);
    }

    Config {
        schema: default_schema_location(),
        rules,
    }
}
