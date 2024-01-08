use crate::{
    error::OmniverseError,
    result::Result,
    utils::version::{carb_app_version, carb_app_version_runtime},
};
use semver::VersionReq;

/// Verifies that the current environment meets the requirements of Isaac Sim.
pub(crate) fn verify_carb_app_requirements() -> Result<()> {
    let compiletime_version = carb_app_version()?;
    let runtime_version = carb_app_version_runtime()?;

    // Check if the compile time and runtime versions are compatible
    let req = VersionReq::parse(&format!(
        ">={}.{}",
        compiletime_version.major, compiletime_version.minor
    ))
    .unwrap();
    if !req.matches(&runtime_version) {
        return Err(OmniverseError::DependencyError(format!(
            "The compile time version of Carbonite is \"{compiletime_version}\" but the runtime version is \"{runtime_version}\""
        )));
    }

    Ok(())
}
