use crate::result::Result;
use omniverse_sys::utils::version::{
    carb_app_version_compiletime_raw, carb_app_version_runtime_raw,
};
use semver::Version;

/// Returns the version of Carbonite that this crate was compiled against.
///
/// # Errors
///
/// Returns an error if the version string is not valid semver.
pub fn carb_app_version() -> Result<Version> {
    let mut raw_version = carb_app_version_compiletime_raw()?;
    maybe_append_patch(&mut raw_version);
    Ok(Version::parse(&raw_version)?)
}

/// Returns the version of Carbonite available at runtime.
///
/// # Errors
///
/// Returns an error if the version string is not valid semver.
pub fn carb_app_version_runtime() -> Result<Version> {
    let mut raw_version = carb_app_version_runtime_raw()?;
    maybe_append_patch(&mut raw_version);
    Ok(Version::parse(&raw_version)?)
}

fn maybe_append_patch(version: &mut String) {
    // Append patch as 0 if not present (check number of dots)
    if version.matches('.').count() == 1 {
        version.push_str(".0");
    }
}
