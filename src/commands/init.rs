use crate::{error::EzClippyError, generated::Config};

#[inline]
pub fn run() -> Result<(), EzClippyError> {
    let config = Config::new();

    let json = serde_json::to_string_pretty(&config)?;

    std::fs::write("ezclippy.json", json)?;

    Ok(())
}
