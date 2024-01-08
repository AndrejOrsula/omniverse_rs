/// Result wrapper for `OmniverseSysError`.
pub type OmniverseSysResult<T> = std::result::Result<T, crate::OmniverseSysError>;

/// Crate-local alias for `OmniverseSysResult`.
pub(crate) type Result<T> = OmniverseSysResult<T>;
