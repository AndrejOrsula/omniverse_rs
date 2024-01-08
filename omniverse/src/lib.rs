// Re-export FFI bindings
pub use omniverse_sys::*;

pub mod utils;

pub use utils::{
    error::{self, OmniverseError},
    result::{self, OmniverseResult},
};

#[ctor::ctor]
fn verify_requirements() {
    utils::requirements::verify_carb_app_requirements().unwrap();
}
