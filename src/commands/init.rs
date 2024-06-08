use crate::{config::build_config, lints::get_available_lints};

pub fn run() {
    let lints = get_available_lints().unwrap();

    let config = build_config(lints);

    let json = serde_json::to_string_pretty(&config).unwrap();

    std::fs::write(".ezclipy.json", json).unwrap();
}
