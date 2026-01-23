use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

pub struct FrameworkProfile;

impl HardwareProfile for FrameworkProfile {
    fn name(&self) -> &str { "Framework Laptop" }
    fn description(&self) -> &str { "Highly modular laptops with focus on repairability" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            KernelRequirement { config: "CONFIG_USB4", expected: ConfigValue::Y, reason: "Thunderbolt/USB4", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_I915", expected: ConfigValue::M, reason: "Intel GPU", severity: Severity::Critical },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            FirmwareRequirement { pattern: "i915/*", min_files: 50, reason: "Intel GPU firmware", severity: Severity::Critical },
        ]
    }
}
