use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

pub struct ThinkPadProfile;

impl HardwareProfile for ThinkPadProfile {
    fn name(&self) -> &str { "ThinkPad" }
    fn description(&self) -> &str { "Enterprise laptops (Intel/AMD variants)" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            KernelRequirement { config: "CONFIG_NVME_CORE", expected: ConfigValue::Y, reason: "NVMe SSD support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_THINKPAD_ACPI", expected: ConfigValue::M, reason: "ThinkPad extras", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_IWLWIFI", expected: ConfigValue::M, reason: "Intel WiFi", severity: Severity::High },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            FirmwareRequirement { pattern: "i915/*", min_files: 50, reason: "Intel GPU firmware", severity: Severity::Critical },
            FirmwareRequirement { pattern: "iwlwifi-*", min_files: 50, reason: "Intel WiFi firmware", severity: Severity::High },
        ]
    }
}
