/// Result wrapper for `OmniverseError`.
pub type OmniverseResult<T> = std::result::Result<T, crate::OmniverseError>;

/// Crate-local alias for `OmniverseResult`.
pub(crate) type Result<T> = OmniverseResult<T>;
