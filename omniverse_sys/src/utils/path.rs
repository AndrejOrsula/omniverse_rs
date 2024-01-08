use std::{env, path::PathBuf};

#[must_use]
pub fn carb_app_path() -> PathBuf {
    PathBuf::from(if let Ok(path) = env::var("CARB_APP_PATH") {
        path
    } else {
        env!("CARB_APP_PATH").to_string()
    })
}
