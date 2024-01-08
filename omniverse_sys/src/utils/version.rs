use crate::result::Result;

/// Returns the raw version of Carbonite that this crate was compiled against.
///
/// # Errors
///
/// Returns an error if the version cannot be parsed from the PACKAGE-INFO.yaml file.
pub fn carb_app_version_compiletime_raw() -> Result<String> {
    Ok(parse_carb_app_version(include_str!(concat!(
        env!("CARB_APP_PATH"),
        "/PACKAGE-INFO.yaml"
    )))?
    .to_string())
}

/// Returns the raw version of Carbonite available at runtime.
///
/// # Errors
///
/// Returns an error if the version cannot be parsed from the PACKAGE-INFO.yaml file.
pub fn carb_app_version_runtime_raw() -> Result<String> {
    Ok(parse_carb_app_version(
        std::fs::read_to_string(crate::carb_app_path().join("PACKAGE-INFO.yaml"))?.as_str(),
    )?
    .to_string())
}

fn parse_carb_app_version(package_info_content: &str) -> Result<&str> {
    Ok(package_info_content
        .split('\n')
        .find(|line| line.starts_with("Version:"))
        .map(|line| {
            line.split(':').collect::<Vec<&str>>()[1]
                .split('+')
                .collect::<Vec<&str>>()[0]
                .trim()
        })
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to parse version string.",
        ))?)
}
