use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

/// Lenovo ThinkPad Profile (2018-2026)
///
/// Covers: T480/T490/T14, X1 Carbon (6th-12th gen), X1 Yoga, P-series workstations
/// Both Intel and AMD variants
/// Known for: Enterprise reliability, TrackPoint, excellent Linux support
pub struct ThinkPadProfile;

impl HardwareProfile for ThinkPadProfile {
    fn name(&self) -> &str { "ThinkPad (T/X/P series 2018+)" }
    fn description(&self) -> &str { "Enterprise laptops - Intel and AMD variants" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU (Intel variants) ===
            KernelRequirement { config: "CONFIG_DRM_I915", expected: ConfigValue::M, reason: "Intel integrated graphics (UHD/Iris)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_AMDGPU", expected: ConfigValue::M, reason: "AMD Radeon (AMD ThinkPads, P-series dGPU)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_KMS_HELPER", expected: ConfigValue::Y, reason: "Kernel mode setting", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_FB", expected: ConfigValue::Y, reason: "Framebuffer support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BACKLIGHT_CLASS_DEVICE", expected: ConfigValue::Y, reason: "Backlight control", severity: Severity::High },

            // === Storage ===
            KernelRequirement { config: "CONFIG_NVME_CORE", expected: ConfigValue::Y, reason: "NVMe SSD (primary storage)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BLK_DEV_NVME", expected: ConfigValue::Y, reason: "NVMe block device", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SATA_AHCI", expected: ConfigValue::Y, reason: "SATA (older models, 2.5\" bay)", severity: Severity::High },

            // === Ethernet ===
            KernelRequirement { config: "CONFIG_E1000E", expected: ConfigValue::Y, reason: "Intel I219 Gigabit Ethernet", severity: Severity::High },
            KernelRequirement { config: "CONFIG_R8169", expected: ConfigValue::Y, reason: "Realtek RTL8111/8168 (some models)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_RTL8152", expected: ConfigValue::M, reason: "Realtek USB Ethernet (docks)", severity: Severity::High },

            // === WiFi ===
            KernelRequirement { config: "CONFIG_IWLWIFI", expected: ConfigValue::M, reason: "Intel WiFi 6/6E/7", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_IWLMVM", expected: ConfigValue::M, reason: "Intel WiFi MVM firmware", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_MAC80211", expected: ConfigValue::M, reason: "802.11 wireless stack", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_CFG80211", expected: ConfigValue::M, reason: "Wireless configuration API", severity: Severity::Critical },
            // Qualcomm WiFi (some AMD models)
            KernelRequirement { config: "CONFIG_ATH11K", expected: ConfigValue::M, reason: "Qualcomm WiFi 6 (some AMD models)", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_ATH11K_PCI", expected: ConfigValue::M, reason: "Qualcomm WiFi 6 PCI", severity: Severity::Medium },

            // === Bluetooth ===
            KernelRequirement { config: "CONFIG_BT", expected: ConfigValue::M, reason: "Bluetooth subsystem", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_HCIBTUSB", expected: ConfigValue::M, reason: "Bluetooth HCI USB", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_INTEL", expected: ConfigValue::M, reason: "Intel Bluetooth", severity: Severity::High },

            // === Audio ===
            KernelRequirement { config: "CONFIG_SND_HDA_INTEL", expected: ConfigValue::Y, reason: "Intel HD Audio", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_REALTEK", expected: ConfigValue::Y, reason: "Realtek codec (speakers/mic)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_HDMI", expected: ConfigValue::Y, reason: "HDMI/DP audio", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_SOC", expected: ConfigValue::M, reason: "SoC audio (newer models)", severity: Severity::Medium },

            // === USB ===
            KernelRequirement { config: "CONFIG_USB_XHCI_HCD", expected: ConfigValue::Y, reason: "USB 3.x (xHCI)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_XHCI_PCI", expected: ConfigValue::Y, reason: "xHCI PCI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_STORAGE", expected: ConfigValue::M, reason: "USB mass storage", severity: Severity::High },

            // === Thunderbolt/USB-C ===
            KernelRequirement { config: "CONFIG_USB4", expected: ConfigValue::M, reason: "Thunderbolt 3/4 / USB4", severity: Severity::High },
            KernelRequirement { config: "CONFIG_THUNDERBOLT", expected: ConfigValue::M, reason: "Thunderbolt bus", severity: Severity::High },
            KernelRequirement { config: "CONFIG_TYPEC", expected: ConfigValue::M, reason: "USB Type-C support", severity: Severity::High },
            KernelRequirement { config: "CONFIG_TYPEC_UCSI", expected: ConfigValue::M, reason: "USB Type-C UCSI", severity: Severity::High },

            // === Input - TrackPoint & Touchpad ===
            KernelRequirement { config: "CONFIG_HID", expected: ConfigValue::Y, reason: "HID core", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_HID_GENERIC", expected: ConfigValue::Y, reason: "Generic HID", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_HID", expected: ConfigValue::Y, reason: "USB HID", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_MOUSE_PS2", expected: ConfigValue::M, reason: "PS/2 mouse (TrackPoint)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_MOUSE_PS2_TRACKPOINT", expected: ConfigValue::Y, reason: "TrackPoint support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_MOUSE_PS2_SYNAPTICS", expected: ConfigValue::Y, reason: "Synaptics touchpad", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MOUSE_PS2_ELANTECH", expected: ConfigValue::Y, reason: "Elantech touchpad", severity: Severity::High },
            KernelRequirement { config: "CONFIG_I2C_HID_CORE", expected: ConfigValue::M, reason: "I2C HID (precision touchpad)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_I2C_HID_ACPI", expected: ConfigValue::M, reason: "I2C HID ACPI", severity: Severity::High },

            // === ThinkPad-specific ===
            KernelRequirement { config: "CONFIG_THINKPAD_ACPI", expected: ConfigValue::M, reason: "ThinkPad ACPI extras (Fn keys, LEDs)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_ACPI_VIDEO", expected: ConfigValue::M, reason: "ACPI backlight control", severity: Severity::High },

            // === Power Management ===
            KernelRequirement { config: "CONFIG_ACPI", expected: ConfigValue::Y, reason: "ACPI support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ACPI_BATTERY", expected: ConfigValue::M, reason: "Battery status", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ACPI_AC", expected: ConfigValue::M, reason: "AC adapter status", severity: Severity::High },
            KernelRequirement { config: "CONFIG_CPU_FREQ", expected: ConfigValue::Y, reason: "CPU frequency scaling", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_INTEL_PSTATE", expected: ConfigValue::Y, reason: "Intel P-state", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_AMD_PSTATE", expected: ConfigValue::Y, reason: "AMD P-state", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SUSPEND", expected: ConfigValue::Y, reason: "Suspend to RAM", severity: Severity::High },
            KernelRequirement { config: "CONFIG_HIBERNATION", expected: ConfigValue::Y, reason: "Hibernate to disk", severity: Severity::Medium },

            // === Webcam ===
            KernelRequirement { config: "CONFIG_MEDIA_USB_SUPPORT", expected: ConfigValue::Y, reason: "USB camera support", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_VIDEO_CLASS", expected: ConfigValue::M, reason: "UVC webcam driver", severity: Severity::High },

            // === Sensors ===
            KernelRequirement { config: "CONFIG_HWMON", expected: ConfigValue::Y, reason: "Hardware monitoring", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SENSORS_CORETEMP", expected: ConfigValue::M, reason: "Intel CPU temperature", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SENSORS_K10TEMP", expected: ConfigValue::M, reason: "AMD CPU temperature", severity: Severity::Medium },

            // === Fingerprint Reader (common but not critical) ===
            KernelRequirement { config: "CONFIG_USB_HIDDEV", expected: ConfigValue::Y, reason: "USB HID device (fingerprint)", severity: Severity::Low },

            // === SD Card Reader ===
            KernelRequirement { config: "CONFIG_MMC", expected: ConfigValue::Y, reason: "SD card support", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_MMC_SDHCI", expected: ConfigValue::Y, reason: "SDHCI controller", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_MMC_SDHCI_PCI", expected: ConfigValue::Y, reason: "SDHCI PCI", severity: Severity::Low },

            // === WWAN (optional LTE/5G) ===
            KernelRequirement { config: "CONFIG_USB_NET_QMI_WWAN", expected: ConfigValue::M, reason: "Qualcomm WWAN (LTE modem)", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_USB_WDM", expected: ConfigValue::M, reason: "WWAN modem support", severity: Severity::Low },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // Intel GPU
            FirmwareRequirement { pattern: "i915/*", min_files: 100, reason: "Intel GPU firmware", severity: Severity::Critical },

            // AMD GPU (AMD variants, P-series dGPU)
            FirmwareRequirement { pattern: "amdgpu/*", min_files: 500, reason: "AMD GPU firmware", severity: Severity::Critical },

            // Intel WiFi
            FirmwareRequirement { pattern: "iwlwifi-*", min_files: 100, reason: "Intel WiFi firmware", severity: Severity::Critical },

            // Qualcomm WiFi (some AMD models)
            FirmwareRequirement { pattern: "ath11k/*", min_files: 10, reason: "Qualcomm WiFi 6 firmware", severity: Severity::Medium },

            // Intel Bluetooth
            FirmwareRequirement { pattern: "intel/ibt-*", min_files: 50, reason: "Intel Bluetooth firmware", severity: Severity::High },

            // CPU Microcode
            FirmwareRequirement { pattern: "intel-ucode/*", min_files: 100, reason: "Intel CPU microcode", severity: Severity::Critical },
            FirmwareRequirement { pattern: "amd-ucode/*", min_files: 1, reason: "AMD CPU microcode", severity: Severity::High },
        ]
    }
}
