use crate::carb;
use cpp::cpp;

impl Default for carb::PluginLoadingDesc {
    fn default() -> Self {
        unsafe {
            cpp!([] -> carb::PluginLoadingDesc as "carb::PluginLoadingDesc" {
                return carb::PluginLoadingDesc::getDefault();
            })
        }
    }
}
