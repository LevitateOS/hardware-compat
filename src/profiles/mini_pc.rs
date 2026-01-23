use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

pub struct AmdMiniPcProfile;

impl HardwareProfile for AmdMiniPcProfile {
    fn name(&self) -> &str { "AMD Mini PC" }
    fn description(&self) -> &str { "Compact systems using AMD Ryzen APUs (Beelink, Minisforum)" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            KernelRequirement { config: "CONFIG_DRM_AMDGPU", expected: ConfigValue::M, reason: "AMD integrated graphics", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_R8169", expected: ConfigValue::Y, reason: "Realtek Ethernet", severity: Severity::High },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            FirmwareRequirement { pattern: "amdgpu/*", min_files: 100, reason: "AMD GPU firmware", severity: Severity::Critical },
        ]
    }
}
