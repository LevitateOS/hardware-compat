use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

/// Steam Deck Profile
///
/// Covers: Steam Deck LCD, Steam Deck OLED
/// CPU: AMD Zen 2 APU (Van Gogh/Sephiroth)
/// GPU: AMD RDNA 2 integrated
/// Features: Handheld gaming, touchscreen, gyro, trackpads
pub struct SteamDeckProfile;

impl HardwareProfile for SteamDeckProfile {
    fn name(&self) -> &str { "Steam Deck (LCD & OLED)" }
    fn description(&self) -> &str { "Valve handheld gaming PC - AMD Van Gogh/Sephiroth APU" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU - AMD RDNA 2 ===
            KernelRequirement { config: "CONFIG_DRM_AMDGPU", expected: ConfigValue::M, reason: "AMD RDNA 2 integrated", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_AMD_DC", expected: ConfigValue::Y, reason: "AMD Display Core", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_AMD_DC_FP", expected: ConfigValue::Y, reason: "AMD DC floating point", severity: Severity::High },
            KernelRequirement { config: "CONFIG_DRM_KMS_HELPER", expected: ConfigValue::Y, reason: "KMS helper", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BACKLIGHT_CLASS_DEVICE", expected: ConfigValue::Y, reason: "Backlight control", severity: Severity::Critical },

            // === Storage ===
            KernelRequirement { config: "CONFIG_NVME_CORE", expected: ConfigValue::Y, reason: "NVMe SSD", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BLK_DEV_NVME", expected: ConfigValue::Y, reason: "NVMe block device", severity: Severity::Critical },
            // SD Card (critical for Steam Deck)
            KernelRequirement { config: "CONFIG_MMC", expected: ConfigValue::Y, reason: "SD card (game storage)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_MMC_SDHCI", expected: ConfigValue::Y, reason: "SDHCI controller", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_MMC_SDHCI_PCI", expected: ConfigValue::Y, reason: "SDHCI PCI", severity: Severity::Critical },

            // === WiFi ===
            // LCD model uses Realtek RTL8822CE (WiFi 5), OLED uses WiFi 6E
            KernelRequirement { config: "CONFIG_MAC80211", expected: ConfigValue::M, reason: "802.11 stack", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_CFG80211", expected: ConfigValue::M, reason: "Wireless config", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_RTW88", expected: ConfigValue::M, reason: "Realtek RTL8822CE WiFi 5 (LCD model)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_RTW88_8822CE", expected: ConfigValue::M, reason: "Realtek RTL8822CE PCIe", severity: Severity::Critical },

            // === Bluetooth ===
            KernelRequirement { config: "CONFIG_BT", expected: ConfigValue::M, reason: "Bluetooth", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_HCIBTUSB", expected: ConfigValue::M, reason: "Bluetooth USB", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_HIDP", expected: ConfigValue::M, reason: "Bluetooth HID (controllers)", severity: Severity::High },

            // === Audio - AMD ===
            KernelRequirement { config: "CONFIG_SND_SOC", expected: ConfigValue::M, reason: "SoC audio", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SND_SOC_AMD_ACP", expected: ConfigValue::M, reason: "AMD Audio CoProcessor", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SND_SOC_AMD_VANGOGH_MACH", expected: ConfigValue::M, reason: "AMD Van Gogh audio", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_HDMI", expected: ConfigValue::Y, reason: "HDMI audio (dock)", severity: Severity::High },

            // === USB ===
            KernelRequirement { config: "CONFIG_USB_XHCI_HCD", expected: ConfigValue::Y, reason: "USB 3.x", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_XHCI_PCI", expected: ConfigValue::Y, reason: "xHCI PCI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_STORAGE", expected: ConfigValue::M, reason: "USB storage", severity: Severity::High },
            KernelRequirement { config: "CONFIG_TYPEC", expected: ConfigValue::M, reason: "USB Type-C", severity: Severity::High },
            KernelRequirement { config: "CONFIG_TYPEC_DP_ALTMODE", expected: ConfigValue::M, reason: "USB-C DisplayPort (dock)", severity: Severity::High },

            // === Input - Steam Deck specific ===
            KernelRequirement { config: "CONFIG_HID", expected: ConfigValue::Y, reason: "HID core", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_HID_GENERIC", expected: ConfigValue::Y, reason: "Generic HID", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_HID", expected: ConfigValue::Y, reason: "USB HID", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_HID_STEAM", expected: ConfigValue::M, reason: "Steam Controller/Deck", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_INPUT_EVDEV", expected: ConfigValue::Y, reason: "Event device", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_INPUT_JOYDEV", expected: ConfigValue::M, reason: "Joystick interface", severity: Severity::High },
            // Touchscreen
            KernelRequirement { config: "CONFIG_INPUT_TOUCHSCREEN", expected: ConfigValue::Y, reason: "Touchscreen", severity: Severity::High },
            KernelRequirement { config: "CONFIG_TOUCHSCREEN_GOODIX", expected: ConfigValue::M, reason: "Goodix touch", severity: Severity::High },
            // Gyro/accelerometer
            KernelRequirement { config: "CONFIG_IIO", expected: ConfigValue::M, reason: "IIO subsystem", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IIO_BUFFER", expected: ConfigValue::Y, reason: "IIO buffer", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BMI160_I2C", expected: ConfigValue::M, reason: "BMI160 gyro/accel", severity: Severity::High },

            // === Game Controllers (external) ===
            KernelRequirement { config: "CONFIG_HID_SONY", expected: ConfigValue::M, reason: "PlayStation controllers", severity: Severity::High },
            KernelRequirement { config: "CONFIG_HID_MICROSOFT", expected: ConfigValue::M, reason: "Xbox controllers", severity: Severity::High },
            KernelRequirement { config: "CONFIG_HID_NINTENDO", expected: ConfigValue::M, reason: "Nintendo controllers", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_JOYSTICK_XPAD", expected: ConfigValue::M, reason: "Xbox gamepad", severity: Severity::High },

            // === Power Management ===
            KernelRequirement { config: "CONFIG_ACPI", expected: ConfigValue::Y, reason: "ACPI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ACPI_BATTERY", expected: ConfigValue::M, reason: "Battery", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ACPI_AC", expected: ConfigValue::M, reason: "AC adapter", severity: Severity::High },
            KernelRequirement { config: "CONFIG_CPU_FREQ", expected: ConfigValue::Y, reason: "CPU frequency", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_AMD_PSTATE", expected: ConfigValue::Y, reason: "AMD P-state", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_AMD_PMC", expected: ConfigValue::M, reason: "AMD PMC (deep sleep)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SUSPEND", expected: ConfigValue::Y, reason: "Suspend", severity: Severity::Critical },

            // === Sensors ===
            KernelRequirement { config: "CONFIG_HWMON", expected: ConfigValue::Y, reason: "Hardware monitoring", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_K10TEMP", expected: ConfigValue::M, reason: "AMD CPU temp", severity: Severity::High },

            // === Ambient Light Sensor ===
            KernelRequirement { config: "CONFIG_IIO_TRIGGERED_BUFFER", expected: ConfigValue::M, reason: "IIO triggered buffer", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_APDS9960", expected: ConfigValue::M, reason: "APDS9960 light sensor", severity: Severity::Medium },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // AMD GPU
            FirmwareRequirement { pattern: "amdgpu/*", min_files: 600, reason: "AMD RDNA 2 firmware", severity: Severity::Critical },

            // AMD PSP
            FirmwareRequirement { pattern: "amdgpu/psp_*", min_files: 10, reason: "AMD PSP firmware", severity: Severity::Critical },

            // Realtek WiFi (LCD model uses RTL8822CE)
            FirmwareRequirement { pattern: "rtw88/*", min_files: 10, reason: "Realtek RTL8822CE WiFi firmware", severity: Severity::Critical },

            // AMD microcode
            FirmwareRequirement { pattern: "amd-ucode/*", min_files: 1, reason: "AMD microcode", severity: Severity::Critical },
        ]
    }
}
