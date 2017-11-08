#[derive(Debug)]
pub enum EngineFeatures {
    Robustness,
}

#[derive(Debug)]
pub enum ContextError {
    OsError(String),
    VersionNotSupported,
    FeatureNotSupported(EngineFeatures),
    NoAvailableFormat,
    IoError(::std::io::Error),
    ContextLost,
    Unknown,
}
