#[derive(Debug)]
pub enum EzClippyError {
    IoError(std::io::Error),
    SerdeJsonError(serde_json::Error),
}

impl core::fmt::Display for EzClippyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EzClippyError::IoError(error) => error.fmt(f),
            EzClippyError::SerdeJsonError(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for EzClippyError {}

impl From<std::io::Error> for EzClippyError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_json::Error> for EzClippyError {
    #[inline]
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJsonError(value)
    }
}
