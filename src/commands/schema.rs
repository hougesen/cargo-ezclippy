use crate::{error::EzClippyError, generated::Config};

#[inline]
pub fn run() -> Result<(), EzClippyError> {
    let mut path = std::env::current_dir()?;

    let package_version = env!("CARGO_PKG_VERSION");

    path.push(format!("schemas/v{package_version}"));

    std::fs::create_dir_all(&path)?;

    let schema = serde_json::to_string_pretty(&schemars::schema_for!(Config))?;

    std::fs::write(path.join("ezclippy.schema.json"), schema)?;

    Ok(())
}
