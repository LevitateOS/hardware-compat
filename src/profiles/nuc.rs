use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

pub struct IntelNucProfile;

impl HardwareProfile for IntelNucProfile {
    fn name(&self) -> &str { "Intel NUC (7th-13th gen)" }
    fn description(&self) -> &str { "Compact Intel systems commonly used for servers and HTPCs" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            KernelRequirement { config: "CONFIG_DRM_I915", expected: ConfigValue::M, reason: "Intel integrated graphics", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_NVME_CORE", expected: ConfigValue::Y, reason: "NVMe SSD support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_E1000E", expected: ConfigValue::Y, reason: "Intel I219 Ethernet", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IGC", expected: ConfigValue::Y, reason: "Intel I225/I226 2.5GbE", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IWLWIFI", expected: ConfigValue::M, reason: "Intel WiFi", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_INTEL", expected: ConfigValue::Y, reason: "Intel HD Audio", severity: Severity::Medium },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            FirmwareRequirement { pattern: "i915/*", min_files: 50, reason: "Intel GPU firmware", severity: Severity::Critical },
            FirmwareRequirement { pattern: "iwlwifi-*", min_files: 50, reason: "Intel WiFi firmware", severity: Severity::High },
            FirmwareRequirement { pattern: "intel-ucode/*", min_files: 50, reason: "Intel CPU microcode", severity: Severity::Critical },
        ]
    }
}
