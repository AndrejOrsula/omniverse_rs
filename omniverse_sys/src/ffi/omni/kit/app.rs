use crate::{omni, result::Result, OmniverseSysError};
use cpp::cpp;
use std::pin::Pin;

impl omni::kit::IApp {
    pub fn tryAcquireInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut omni::kit::IApp as "omni::kit::IApp *" {
                return carb::getFramework()->tryAcquireInterface<omni::kit::IApp>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire IApp interface. Is `omni.kit.app.plugin` loaded?".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }

    pub fn tryAcquireExistingInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut omni::kit::IApp as "omni::kit::IApp *" {
                return carb::getFramework()->tryAcquireExistingInterface<omni::kit::IApp>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire an existing IApp interface. Remember to call `omni::kit::IApp::acquire_interface()` first.".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }
}
