use crate::{config::build_config, lints::get_available_lints};

pub fn run() {
    let lints = get_available_lints().unwrap();

    let config = build_config(lints);

    let schema = schemars::schema_for_value!(config);

    std::fs::write(
        "schema.json",
        serde_json::to_string_pretty(&schema).unwrap(),
    )
    .unwrap();
}
