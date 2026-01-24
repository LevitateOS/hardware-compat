use crate::firmware::FirmwareRequirement;
use crate::kernel::{ConfigValue, KernelRequirement};
use crate::profiles::HardwareProfile;
use crate::report::Severity;

/// Dell XPS Profile (2018-2026)
///
/// Covers: XPS 13, XPS 15, XPS 17 (all generations)
/// CPU: Intel 8th-14th gen, AMD Ryzen (XPS 13 Plus)
/// Known for: Premium build, 4K/OLED displays, Developer Edition available
pub struct DellXpsProfile;

impl HardwareProfile for DellXpsProfile {
    fn name(&self) -> &str {
        "Dell XPS (13/15/17)"
    }
    fn description(&self) -> &str {
        "Premium Dell ultrabooks - excellent Linux support"
    }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU ===
            KernelRequirement {
                config: "CONFIG_DRM_I915",
                expected: ConfigValue::M,
                reason: "Intel Iris Xe/UHD",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_DRM_NOUVEAU",
                expected: ConfigValue::M,
                reason: "NVIDIA GTX/RTX (XPS 15/17)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_DRM_KMS_HELPER",
                expected: ConfigValue::Y,
                reason: "KMS helper",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_BACKLIGHT_CLASS_DEVICE",
                expected: ConfigValue::Y,
                reason: "Backlight control",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_DRM_DISPLAY_DP_HELPER",
                expected: ConfigValue::Y,
                reason: "DisplayPort (4K/OLED)",
                severity: Severity::High,
            },
            // === Storage ===
            KernelRequirement {
                config: "CONFIG_NVME_CORE",
                expected: ConfigValue::Y,
                reason: "NVMe SSD",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_BLK_DEV_NVME",
                expected: ConfigValue::Y,
                reason: "NVMe block device",
                severity: Severity::Critical,
            },
            // === Thunderbolt (critical for XPS) ===
            KernelRequirement {
                config: "CONFIG_USB4",
                expected: ConfigValue::M,
                reason: "USB4/Thunderbolt 4",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB4",
                expected: ConfigValue::M,
                reason: "Thunderbolt bus",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_TYPEC",
                expected: ConfigValue::M,
                reason: "USB Type-C",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_TYPEC_UCSI",
                expected: ConfigValue::M,
                reason: "USB Type-C UCSI",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_TYPEC_DP_ALTMODE",
                expected: ConfigValue::M,
                reason: "USB-C DisplayPort",
                severity: Severity::High,
            },
            // === USB ===
            KernelRequirement {
                config: "CONFIG_USB_XHCI_HCD",
                expected: ConfigValue::Y,
                reason: "USB 3.x",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB_XHCI_PCI",
                expected: ConfigValue::Y,
                reason: "xHCI PCI",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB_RTL8152",
                expected: ConfigValue::M,
                reason: "USB Ethernet (Dell docks)",
                severity: Severity::High,
            },
            // === WiFi ===
            KernelRequirement {
                config: "CONFIG_MAC80211",
                expected: ConfigValue::M,
                reason: "802.11 stack",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_CFG80211",
                expected: ConfigValue::M,
                reason: "Wireless config",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_IWLWIFI",
                expected: ConfigValue::M,
                reason: "Intel WiFi 6E (Killer)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_IWLMVM",
                expected: ConfigValue::M,
                reason: "Intel WiFi MVM",
                severity: Severity::Critical,
            },
            // === Bluetooth ===
            KernelRequirement {
                config: "CONFIG_BT",
                expected: ConfigValue::M,
                reason: "Bluetooth",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_BT_HCIBTUSB",
                expected: ConfigValue::M,
                reason: "Bluetooth USB",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_BT_INTEL",
                expected: ConfigValue::M,
                reason: "Intel Bluetooth",
                severity: Severity::High,
            },
            // === Audio ===
            KernelRequirement {
                config: "CONFIG_SND_HDA_INTEL",
                expected: ConfigValue::Y,
                reason: "Intel HD Audio",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_HDA_CODEC_REALTEK",
                expected: ConfigValue::Y,
                reason: "Realtek codec",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_HDA_CODEC_HDMI",
                expected: ConfigValue::Y,
                reason: "HDMI/DP audio",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_SOC",
                expected: ConfigValue::M,
                reason: "SoC audio (newer XPS)",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_SND_SOC_INTEL_AVS",
                expected: ConfigValue::M,
                reason: "Intel AVS audio",
                severity: Severity::Medium,
            },
            // === Input ===
            KernelRequirement {
                config: "CONFIG_HID",
                expected: ConfigValue::Y,
                reason: "HID core",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_HID_GENERIC",
                expected: ConfigValue::Y,
                reason: "Generic HID",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB_HID",
                expected: ConfigValue::Y,
                reason: "USB HID",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_I2C_HID_CORE",
                expected: ConfigValue::M,
                reason: "I2C HID (precision touchpad)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_I2C_HID_ACPI",
                expected: ConfigValue::M,
                reason: "I2C HID ACPI",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_INPUT_EVDEV",
                expected: ConfigValue::Y,
                reason: "Event device",
                severity: Severity::Critical,
            },
            // === Dell Platform ===
            KernelRequirement {
                config: "CONFIG_DELL_LAPTOP",
                expected: ConfigValue::M,
                reason: "Dell laptop support",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_DELL_WMI",
                expected: ConfigValue::M,
                reason: "Dell WMI",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_DELL_SMBIOS",
                expected: ConfigValue::M,
                reason: "Dell SMBIOS",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_DELL_RBTN",
                expected: ConfigValue::M,
                reason: "Dell radio button",
                severity: Severity::Medium,
            },
            // === Power Management ===
            KernelRequirement {
                config: "CONFIG_ACPI",
                expected: ConfigValue::Y,
                reason: "ACPI",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_ACPI_BATTERY",
                expected: ConfigValue::M,
                reason: "Battery",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_ACPI_AC",
                expected: ConfigValue::M,
                reason: "AC adapter",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_CPU_FREQ",
                expected: ConfigValue::Y,
                reason: "CPU frequency",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_X86_INTEL_PSTATE",
                expected: ConfigValue::Y,
                reason: "Intel P-state",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_INTEL_HFI_THERMAL",
                expected: ConfigValue::M,
                reason: "Intel thermal (12th gen+)",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_SUSPEND",
                expected: ConfigValue::Y,
                reason: "Suspend",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_HIBERNATION",
                expected: ConfigValue::Y,
                reason: "Hibernate",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_PM_SLEEP",
                expected: ConfigValue::Y,
                reason: "PM sleep",
                severity: Severity::High,
            },
            // === Webcam ===
            KernelRequirement {
                config: "CONFIG_MEDIA_USB_SUPPORT",
                expected: ConfigValue::Y,
                reason: "USB camera",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_USB_VIDEO_CLASS",
                expected: ConfigValue::M,
                reason: "UVC webcam",
                severity: Severity::High,
            },
            // IPU6 camera on newer XPS models
            KernelRequirement {
                config: "CONFIG_INTEL_VSC",
                expected: ConfigValue::M,
                reason: "Intel Visual Sensing",
                severity: Severity::Medium,
            },
            // === Fingerprint ===
            KernelRequirement {
                config: "CONFIG_USB_HIDDEV",
                expected: ConfigValue::Y,
                reason: "USB HID device",
                severity: Severity::Low,
            },
            // === Sensors ===
            KernelRequirement {
                config: "CONFIG_HWMON",
                expected: ConfigValue::Y,
                reason: "Hardware monitoring",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_SENSORS_CORETEMP",
                expected: ConfigValue::M,
                reason: "Intel CPU temp",
                severity: Severity::Medium,
            },
            // === SD Card (XPS 15/17) ===
            KernelRequirement {
                config: "CONFIG_MMC",
                expected: ConfigValue::Y,
                reason: "SD card",
                severity: Severity::Low,
            },
            KernelRequirement {
                config: "CONFIG_MMC_SDHCI",
                expected: ConfigValue::Y,
                reason: "SDHCI",
                severity: Severity::Low,
            },
            KernelRequirement {
                config: "CONFIG_MMC_SDHCI_PCI",
                expected: ConfigValue::Y,
                reason: "SDHCI PCI",
                severity: Severity::Low,
            },
            KernelRequirement {
                config: "CONFIG_MMC_REALTEK_PCI",
                expected: ConfigValue::M,
                reason: "Realtek SD reader",
                severity: Severity::Low,
            },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // Intel GPU
            FirmwareRequirement {
                pattern: "i915/*",
                min_files: 100,
                reason: "Intel GPU firmware",
                severity: Severity::Critical,
            },
            // NVIDIA (XPS 15/17)
            FirmwareRequirement {
                pattern: "nvidia/*",
                min_files: 30,
                reason: "NVIDIA GPU firmware",
                severity: Severity::High,
            },
            // Intel WiFi
            FirmwareRequirement {
                pattern: "iwlwifi-*",
                min_files: 100,
                reason: "Intel WiFi firmware",
                severity: Severity::Critical,
            },
            // Intel Bluetooth
            FirmwareRequirement {
                pattern: "intel/ibt-*",
                min_files: 50,
                reason: "Intel Bluetooth firmware",
                severity: Severity::High,
            },
            // Intel microcode
            FirmwareRequirement {
                pattern: "intel-ucode/*",
                min_files: 100,
                reason: "Intel CPU microcode",
                severity: Severity::Critical,
            },
        ]
    }
}
