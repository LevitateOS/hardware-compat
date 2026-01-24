use crate::firmware::FirmwareRequirement;
use crate::kernel::{ConfigValue, KernelRequirement};
use crate::profiles::HardwareProfile;
use crate::report::Severity;

/// Intel NUC (7th-13th gen) - Compact Intel systems
///
/// Covers: NUC7, NUC8, NUC10, NUC11, NUC12, NUC13
/// Common uses: HTPCs, home servers, desktop replacements
pub struct IntelNucProfile;

impl HardwareProfile for IntelNucProfile {
    fn name(&self) -> &str {
        "Intel NUC (7th-13th gen)"
    }
    fn description(&self) -> &str {
        "Compact Intel systems - NUC7 through NUC13"
    }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU ===
            KernelRequirement {
                config: "CONFIG_DRM_I915",
                expected: ConfigValue::M,
                reason: "Intel integrated graphics (UHD 620/630, Iris Xe)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_DRM_KMS_HELPER",
                expected: ConfigValue::Y,
                reason: "Kernel mode setting support",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_FB",
                expected: ConfigValue::Y,
                reason: "Framebuffer support",
                severity: Severity::Critical,
            },
            // === Storage ===
            KernelRequirement {
                config: "CONFIG_NVME_CORE",
                expected: ConfigValue::Y,
                reason: "NVMe SSD (M.2 slot)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_BLK_DEV_NVME",
                expected: ConfigValue::Y,
                reason: "NVMe block device",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_SATA_AHCI",
                expected: ConfigValue::Y,
                reason: "SATA SSD/HDD (2.5\" bay on some models)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_ATA",
                expected: ConfigValue::Y,
                reason: "ATA subsystem",
                severity: Severity::High,
            },
            // === Ethernet ===
            KernelRequirement {
                config: "CONFIG_E1000E",
                expected: ConfigValue::Y,
                reason: "Intel I219-V/LM Gigabit Ethernet",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_IGC",
                expected: ConfigValue::Y,
                reason: "Intel I225-V/I226-V 2.5GbE (NUC11+)",
                severity: Severity::High,
            },
            // === WiFi ===
            KernelRequirement {
                config: "CONFIG_IWLWIFI",
                expected: ConfigValue::M,
                reason: "Intel WiFi 6/6E/7 (AX200/AX201/AX210/AX211/BE200)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_IWLMVM",
                expected: ConfigValue::M,
                reason: "Intel WiFi MVM firmware support",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_MAC80211",
                expected: ConfigValue::M,
                reason: "802.11 wireless stack",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_CFG80211",
                expected: ConfigValue::M,
                reason: "Wireless configuration API",
                severity: Severity::Critical,
            },
            // === Bluetooth ===
            KernelRequirement {
                config: "CONFIG_BT",
                expected: ConfigValue::M,
                reason: "Bluetooth subsystem",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_BT_HCIBTUSB",
                expected: ConfigValue::M,
                reason: "Bluetooth HCI USB driver",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_BT_INTEL",
                expected: ConfigValue::M,
                reason: "Intel Bluetooth firmware loader",
                severity: Severity::High,
            },
            // === Audio ===
            KernelRequirement {
                config: "CONFIG_SND_HDA_INTEL",
                expected: ConfigValue::Y,
                reason: "Intel HD Audio controller",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_HDA_CODEC_REALTEK",
                expected: ConfigValue::Y,
                reason: "Realtek audio codec (speakers/headphone)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_HDA_CODEC_HDMI",
                expected: ConfigValue::Y,
                reason: "HDMI/DisplayPort audio output",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_HDA_CORE",
                expected: ConfigValue::Y,
                reason: "HD Audio core",
                severity: Severity::High,
            },
            // === USB ===
            KernelRequirement {
                config: "CONFIG_USB_XHCI_HCD",
                expected: ConfigValue::Y,
                reason: "USB 3.0/3.1/3.2 (xHCI)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB_XHCI_PCI",
                expected: ConfigValue::Y,
                reason: "xHCI PCI support",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB_EHCI_HCD",
                expected: ConfigValue::Y,
                reason: "USB 2.0 (EHCI)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_USB_STORAGE",
                expected: ConfigValue::M,
                reason: "USB mass storage",
                severity: Severity::High,
            },
            // === Thunderbolt (NUC8+) ===
            KernelRequirement {
                config: "CONFIG_USB4",
                expected: ConfigValue::M,
                reason: "Thunderbolt 3/4 / USB4 (NUC8+)",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_USB4",
                expected: ConfigValue::M,
                reason: "Thunderbolt bus driver",
                severity: Severity::Medium,
            },
            // === Input ===
            KernelRequirement {
                config: "CONFIG_HID",
                expected: ConfigValue::Y,
                reason: "HID core (USB keyboards/mice)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_HID_GENERIC",
                expected: ConfigValue::Y,
                reason: "Generic HID driver",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB_HID",
                expected: ConfigValue::Y,
                reason: "USB HID support",
                severity: Severity::Critical,
            },
            // === Power Management ===
            KernelRequirement {
                config: "CONFIG_ACPI",
                expected: ConfigValue::Y,
                reason: "ACPI support",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_ACPI_VIDEO",
                expected: ConfigValue::M,
                reason: "ACPI video driver",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_CPU_FREQ",
                expected: ConfigValue::Y,
                reason: "CPU frequency scaling",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_X86_INTEL_PSTATE",
                expected: ConfigValue::Y,
                reason: "Intel P-state driver",
                severity: Severity::High,
            },
            // === Sensors/Monitoring ===
            KernelRequirement {
                config: "CONFIG_HWMON",
                expected: ConfigValue::Y,
                reason: "Hardware monitoring",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_SENSORS_CORETEMP",
                expected: ConfigValue::M,
                reason: "Intel CPU temperature",
                severity: Severity::Medium,
            },
            // === SD Card Reader (some models) ===
            KernelRequirement {
                config: "CONFIG_MMC",
                expected: ConfigValue::Y,
                reason: "MMC/SD card support",
                severity: Severity::Low,
            },
            KernelRequirement {
                config: "CONFIG_MMC_SDHCI",
                expected: ConfigValue::Y,
                reason: "SDHCI controller",
                severity: Severity::Low,
            },
            KernelRequirement {
                config: "CONFIG_MMC_SDHCI_PCI",
                expected: ConfigValue::Y,
                reason: "SDHCI PCI support",
                severity: Severity::Low,
            },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // GPU
            FirmwareRequirement {
                pattern: "i915/*",
                min_files: 100,
                reason: "Intel GPU firmware (GuC, HuC, DMC)",
                severity: Severity::Critical,
            },
            // WiFi
            FirmwareRequirement {
                pattern: "iwlwifi-*",
                min_files: 100,
                reason: "Intel WiFi firmware (AX200/AX201/AX210/AX211/BE200)",
                severity: Severity::Critical,
            },
            // Bluetooth
            FirmwareRequirement {
                pattern: "intel/ibt-*",
                min_files: 50,
                reason: "Intel Bluetooth firmware",
                severity: Severity::High,
            },
            // CPU Microcode
            FirmwareRequirement {
                pattern: "intel-ucode/*",
                min_files: 100,
                reason: "Intel CPU microcode updates",
                severity: Severity::Critical,
            },
        ]
    }
}
