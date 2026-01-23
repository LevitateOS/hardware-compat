use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

pub struct DesktopProfile;

impl HardwareProfile for DesktopProfile {
    fn name(&self) -> &str { "Generic Desktop" }
    fn description(&self) -> &str { "Custom built or pre-built desktop systems" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            KernelRequirement { config: "CONFIG_DRM_AMDGPU", expected: ConfigValue::M, reason: "AMD GPU support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_NOUVEAU", expected: ConfigValue::M, reason: "NVIDIA (nouveau) support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SATA_AHCI", expected: ConfigValue::Y, reason: "SATA storage", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_XHCI_HCD", expected: ConfigValue::Y, reason: "USB 3.0", severity: Severity::Critical },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            FirmwareRequirement { pattern: "amdgpu/*", min_files: 100, reason: "AMD GPU firmware", severity: Severity::Critical },
            FirmwareRequirement { pattern: "nvidia/*", min_files: 10, reason: "NVIDIA firmware", severity: Severity::High },
        ]
    }
}
