use crate::{omni, result::Result, OmniverseSysError};
use cpp::cpp;
use std::pin::Pin;

impl omni::kit::IViewport {
    pub fn tryAcquireInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut omni::kit::IViewport as "omni::kit::IViewport *" {
                return carb::getFramework()->tryAcquireInterface<omni::kit::IViewport>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire IViewport interface.".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }

    pub fn tryAcquireExistingInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut omni::kit::IViewport as "omni::kit::IViewport *" {
                return carb::getFramework()->tryAcquireExistingInterface<omni::kit::IViewport>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire an existing IViewport interface. Remember to call `omni::kit::IViewport::acquire_interface()` first.".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }
}
