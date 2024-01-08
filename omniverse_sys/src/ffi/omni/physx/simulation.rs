use crate::{omni, result::Result, OmniverseSysError};
use cpp::cpp;
use std::pin::Pin;

impl omni::physx::IPhysx {
    pub fn tryAcquireInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut omni::physx::IPhysx as "omni::physx::IPhysx *" {
                return carb::getFramework()->tryAcquireInterface<omni::physx::IPhysx>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire IPhysx interface.".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }

    pub fn tryAcquireExistingInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut omni::physx::IPhysx as "omni::physx::IPhysx *" {
                return carb::getFramework()->tryAcquireExistingInterface<omni::physx::IPhysx>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire an existing IApp interface. Remember to call `omni::physx::IPhysx::acquire_interface()` first.".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }

    pub fn updateSimulationScene(
        self: Pin<&mut Self>,
        scene_path: u64,
        elapsed_step: f32,
        current_time: f32,
    ) {
        unsafe {
            cpp!([self as "omni::physx::IPhysx *", scene_path as "unsigned long", elapsed_step as "float", current_time as "float"] {
                self->updateSimulationScene(scene_path, elapsed_step, current_time);
            });
        }
    }

    pub fn updateTransformationsScene(
        self: Pin<&mut Self>,
        scene_path: u64,
        update_to_usd: bool,
        update_velocities_to_usd: bool,
    ) {
        unsafe {
            cpp!([self as "omni::physx::IPhysx *", scene_path as "unsigned long", update_to_usd as "bool", update_velocities_to_usd as "bool"] {
                self->updateTransformationsScene(scene_path, update_to_usd, update_velocities_to_usd);
            });
        }
    }

    pub fn forceLoadPhysicsFromUsd(self: Pin<&mut Self>) {
        unsafe {
            cpp!([self as "omni::physx::IPhysx *"] {
                self->forceLoadPhysicsFromUsd();
            });
        }
    }
    pub fn startSimulation(self: Pin<&mut Self>) {
        unsafe {
            cpp!([self as "omni::physx::IPhysx *"] {
                self->startSimulation();
            });
        }
    }

    pub fn updateSimulation(self: Pin<&mut Self>, elapsed_step: f32, current_time: f32) {
        unsafe {
            cpp!([self as "omni::physx::IPhysx *", elapsed_step as "float", current_time as "float"] {
                self->updateSimulation(elapsed_step, current_time);
            });
        }
    }

    pub fn updateTransformations(
        self: Pin<&mut Self>,
        update_to_usd: bool,
        update_velocities_to_usd: bool,
    ) {
        unsafe {
            cpp!([self as "omni::physx::IPhysx *", update_to_usd as "bool", update_velocities_to_usd as "bool"] {
                self->updateTransformations(update_to_usd, update_velocities_to_usd);
            });
        }
    }
}

impl omni::physx::IPhysxSimulation {
    pub fn tryAcquireInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut omni::physx::IPhysxSimulation as "omni::physx::IPhysxSimulation *" {
                return carb::getFramework()->tryAcquireInterface<omni::physx::IPhysxSimulation>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire IPhysxSimulation interface.".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }

    pub fn tryAcquireExistingInterface() -> Result<Pin<Box<Self>>> {
        let app_ptr = unsafe {
            cpp!([] -> *mut omni::physx::IPhysxSimulation as "omni::physx::IPhysxSimulation *" {
                return carb::getFramework()->tryAcquireExistingInterface<omni::physx::IPhysxSimulation>();
            })
        };
        if app_ptr.is_null() {
            return Err(OmniverseSysError::InitializationError(
                "Failed to acquire an existing IApp interface. Remember to call `omni::physx::IPhysxSimulation::acquire_interface()` first.".to_string(),
            ));
        }
        Ok(Box::into_pin(unsafe { Box::from_raw(app_ptr) }))
    }

    #[must_use]
    pub fn attachStage(self: Pin<&mut Self>, stage_id: i64) -> bool {
        unsafe {
            cpp!([self as "omni::physx::IPhysxSimulation *", stage_id as "long"] -> bool as "bool" {
                return self->attachStage(stage_id);
            })
        }
    }

    #[must_use]
    pub fn checkResultsScene(self: Pin<&mut Self>, scene_path: u64) -> bool {
        unsafe {
            cpp!([self as "omni::physx::IPhysxSimulation *", scene_path as "unsigned long"] -> bool as "bool" {
                return self->checkResultsScene(scene_path);
            })
        }
    }

    #[must_use]
    pub fn checkResults(self: Pin<&mut Self>) -> bool {
        unsafe {
            cpp!([self as "omni::physx::IPhysxSimulation *"] -> bool as "bool" {
                return self->checkResults();
            })
        }
    }

    pub fn pauseChangeTracking(self: Pin<&mut Self>, pause: bool) {
        unsafe {
            cpp!([self as "omni::physx::IPhysxSimulation *", pause as "bool"] {
                self->pauseChangeTracking(pause);
            });
        }
    }

    pub fn simulate(self: Pin<&mut Self>, elapsed_step: f32, current_time: f32) {
        unsafe {
            cpp!([self as "omni::physx::IPhysxSimulation *", elapsed_step as "float", current_time as "float"] {
                self->simulate(elapsed_step, current_time);
            });
        }
    }

    pub fn simulateScene(
        self: Pin<&mut Self>,
        scene_path: u64,
        elapsed_step: f32,
        current_time: f32,
    ) {
        unsafe {
            cpp!([self as "omni::physx::IPhysxSimulation *", scene_path as "unsigned long", elapsed_step as "float", current_time as "float"] {
                self->simulateScene(scene_path, elapsed_step, current_time);
            });
        }
    }

    pub fn fetchResultsScene(self: Pin<&mut Self>, scene_path: u64) {
        unsafe {
            cpp!([self as "omni::physx::IPhysxSimulation *", scene_path as "unsigned long"] {
                self->fetchResultsScene(scene_path);
            });
        }
    }

    pub fn detachStage(self: Pin<&mut Self>) {
        unsafe {
            cpp!([self as "omni::physx::IPhysxSimulation *"] {
                self->detachStage();
            });
        }
    }

    pub fn fetchResults(self: Pin<&mut Self>) {
        unsafe {
            cpp!([self as "omni::physx::IPhysxSimulation *"] {
                self->fetchResults();
            });
        }
    }
}
