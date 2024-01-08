use crate::carb;
use std::{mem::ManuallyDrop, pin::Pin};

impl carb::Framework {
    #[must_use]
    pub fn get() -> ManuallyDrop<Pin<Box<Self>>> {
        let framework_ptr = carb::getFramework();
        debug_assert!(!framework_ptr.is_null());
        ManuallyDrop::new(Box::into_pin(unsafe { Box::from_raw(framework_ptr) }))
    }
}
