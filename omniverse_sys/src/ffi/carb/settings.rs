use crate::{carb, result::Result, OmniverseSysError};
use cpp::cpp;
use std::pin::Pin;

impl carb::settings::ISettings {
    pub fn tryAcquireInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut carb::settings::ISettings as "carb::settings::ISettings *" {
                return carb::getFramework()->tryAcquireInterface<carb::settings::ISettings>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire ISettings interface.".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }

    pub fn tryAcquireExistingInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut carb::settings::ISettings as "carb::settings::ISettings *" {
                return carb::getFramework()->tryAcquireExistingInterface<carb::settings::ISettings>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire an existing ISettings interface. Remember to call `carb::settings::ISettings::acquire_interface()` first.".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }
}
