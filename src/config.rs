#[inline]
pub fn default_schema_location() -> String {
    let package_version = env!("CARGO_PKG_VERSION");

    format!("https://raw.githubusercontent.com/hougesen/cargo-ezclippy/main/schemas/v{package_version}/ezclippy.schema.json")
}

impl crate::generated::Config {
    #[inline]
    pub fn new() -> Self {
        Self {
            schema: default_schema_location(),

            ..Default::default()
        }
    }
}
