use thiserror::Error;

#[derive(Error, Debug)]
pub enum OmniverseError {
    #[error(transparent)]
    SysError(#[from] omniverse_sys::error::OmniverseSysError),

    #[error("Dependency error: {0}")]
    DependencyError(String),
}

impl From<semver::Error> for OmniverseError {
    fn from(e: semver::Error) -> Self {
        OmniverseError::DependencyError(e.to_string())
    }
}
