use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

/// Linux-First Laptop Profile (System76, TUXEDO, Purism, Star Labs)
///
/// Covers: System76 (Lemur Pro, Galago Pro, Oryx Pro, Darter Pro)
///         TUXEDO (Pulse, InfinityBook, Stellaris)
///         Purism Librem, Star Labs StarBook/Byte
/// Known for: Excellent Linux support out of box, open firmware, Coreboot
pub struct LinuxLaptopProfile;

impl HardwareProfile for LinuxLaptopProfile {
    fn name(&self) -> &str { "Linux-First Laptop (System76, TUXEDO)" }
    fn description(&self) -> &str { "Laptops designed for Linux - excellent out-of-box support" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU ===
            KernelRequirement { config: "CONFIG_DRM_I915", expected: ConfigValue::M, reason: "Intel integrated", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_AMDGPU", expected: ConfigValue::M, reason: "AMD integrated/discrete", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_NOUVEAU", expected: ConfigValue::M, reason: "NVIDIA discrete (nouveau)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_DRM_KMS_HELPER", expected: ConfigValue::Y, reason: "KMS helper", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BACKLIGHT_CLASS_DEVICE", expected: ConfigValue::Y, reason: "Backlight control", severity: Severity::Critical },

            // === Storage ===
            KernelRequirement { config: "CONFIG_NVME_CORE", expected: ConfigValue::Y, reason: "NVMe SSD", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BLK_DEV_NVME", expected: ConfigValue::Y, reason: "NVMe block device", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SATA_AHCI", expected: ConfigValue::Y, reason: "SATA (some models)", severity: Severity::High },

            // === Ethernet ===
            KernelRequirement { config: "CONFIG_R8169", expected: ConfigValue::Y, reason: "Realtek Ethernet", severity: Severity::High },
            KernelRequirement { config: "CONFIG_E1000E", expected: ConfigValue::Y, reason: "Intel Ethernet", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_RTL8152", expected: ConfigValue::M, reason: "USB Ethernet", severity: Severity::High },

            // === WiFi ===
            KernelRequirement { config: "CONFIG_MAC80211", expected: ConfigValue::M, reason: "802.11 stack", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_CFG80211", expected: ConfigValue::M, reason: "Wireless config", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_IWLWIFI", expected: ConfigValue::M, reason: "Intel WiFi 6E/7", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_IWLMVM", expected: ConfigValue::M, reason: "Intel WiFi MVM", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ATH11K", expected: ConfigValue::M, reason: "Qualcomm WiFi 6", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MT7921E", expected: ConfigValue::M, reason: "MediaTek WiFi 6E", severity: Severity::High },

            // === Bluetooth ===
            KernelRequirement { config: "CONFIG_BT", expected: ConfigValue::M, reason: "Bluetooth", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_HCIBTUSB", expected: ConfigValue::M, reason: "Bluetooth USB", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_INTEL", expected: ConfigValue::M, reason: "Intel Bluetooth", severity: Severity::High },

            // === Audio ===
            KernelRequirement { config: "CONFIG_SND_HDA_INTEL", expected: ConfigValue::Y, reason: "Intel HD Audio", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_REALTEK", expected: ConfigValue::Y, reason: "Realtek codec", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_HDMI", expected: ConfigValue::Y, reason: "HDMI audio", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_SOC", expected: ConfigValue::M, reason: "SoC audio", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SND_USB_AUDIO", expected: ConfigValue::M, reason: "USB audio", severity: Severity::High },

            // === USB / Thunderbolt ===
            KernelRequirement { config: "CONFIG_USB_XHCI_HCD", expected: ConfigValue::Y, reason: "USB 3.x", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_XHCI_PCI", expected: ConfigValue::Y, reason: "xHCI PCI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB4", expected: ConfigValue::M, reason: "USB4/Thunderbolt", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB4", expected: ConfigValue::M, reason: "Thunderbolt bus", severity: Severity::High },
            KernelRequirement { config: "CONFIG_TYPEC", expected: ConfigValue::M, reason: "USB Type-C", severity: Severity::High },

            // === Input ===
            KernelRequirement { config: "CONFIG_HID", expected: ConfigValue::Y, reason: "HID core", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_HID_GENERIC", expected: ConfigValue::Y, reason: "Generic HID", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_HID", expected: ConfigValue::Y, reason: "USB HID", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_I2C_HID_CORE", expected: ConfigValue::M, reason: "I2C HID (touchpad)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_I2C_HID_ACPI", expected: ConfigValue::M, reason: "I2C HID ACPI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_INPUT_EVDEV", expected: ConfigValue::Y, reason: "Event device", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_MOUSE_PS2_ELANTECH", expected: ConfigValue::Y, reason: "Elantech touchpad", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MOUSE_PS2_SYNAPTICS", expected: ConfigValue::Y, reason: "Synaptics touchpad", severity: Severity::High },

            // === Vendor Platform Drivers ===
            // System76 (in mainline kernel since 5.16)
            KernelRequirement { config: "CONFIG_SYSTEM76_ACPI", expected: ConfigValue::M, reason: "System76 ACPI (Fn keys, backlight, airplane LED)", severity: Severity::High },
            // Note: TUXEDO keyboard requires out-of-tree tuxedo-drivers (DKMS)
            // Note: Clevo WMI requires out-of-tree clevo-wmi driver (DKMS)

            // === Power Management ===
            KernelRequirement { config: "CONFIG_ACPI", expected: ConfigValue::Y, reason: "ACPI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ACPI_BATTERY", expected: ConfigValue::M, reason: "Battery", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ACPI_AC", expected: ConfigValue::M, reason: "AC adapter", severity: Severity::High },
            KernelRequirement { config: "CONFIG_CPU_FREQ", expected: ConfigValue::Y, reason: "CPU frequency", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_INTEL_PSTATE", expected: ConfigValue::Y, reason: "Intel P-state", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_AMD_PSTATE", expected: ConfigValue::Y, reason: "AMD P-state", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SUSPEND", expected: ConfigValue::Y, reason: "Suspend", severity: Severity::High },
            KernelRequirement { config: "CONFIG_HIBERNATION", expected: ConfigValue::Y, reason: "Hibernate", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_PM_SLEEP", expected: ConfigValue::Y, reason: "PM sleep", severity: Severity::High },

            // === Webcam ===
            KernelRequirement { config: "CONFIG_MEDIA_USB_SUPPORT", expected: ConfigValue::Y, reason: "USB camera", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_VIDEO_CLASS", expected: ConfigValue::M, reason: "UVC webcam", severity: Severity::High },

            // === Sensors ===
            KernelRequirement { config: "CONFIG_HWMON", expected: ConfigValue::Y, reason: "Hardware monitoring", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_CORETEMP", expected: ConfigValue::M, reason: "Intel CPU temp", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_K10TEMP", expected: ConfigValue::M, reason: "AMD CPU temp", severity: Severity::High },

            // === Keyboard Backlight ===
            KernelRequirement { config: "CONFIG_LEDS_CLASS", expected: ConfigValue::Y, reason: "LED class", severity: Severity::High },
            KernelRequirement { config: "CONFIG_LEDS_CLASS_MULTICOLOR", expected: ConfigValue::M, reason: "Multicolor LED (RGB)", severity: Severity::Medium },

            // === SD Card ===
            KernelRequirement { config: "CONFIG_MMC", expected: ConfigValue::Y, reason: "SD card", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_MMC_SDHCI", expected: ConfigValue::Y, reason: "SDHCI", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_MMC_SDHCI_PCI", expected: ConfigValue::Y, reason: "SDHCI PCI", severity: Severity::Medium },

            // === Coreboot (open firmware support) ===
            KernelRequirement { config: "CONFIG_GOOGLE_COREBOOT_TABLE", expected: ConfigValue::M, reason: "Coreboot tables", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_GOOGLE_MEMCONSOLE_COREBOOT", expected: ConfigValue::M, reason: "Coreboot console", severity: Severity::Low },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // Intel GPU
            FirmwareRequirement { pattern: "i915/*", min_files: 100, reason: "Intel GPU firmware", severity: Severity::Critical },

            // AMD GPU
            FirmwareRequirement { pattern: "amdgpu/*", min_files: 600, reason: "AMD GPU firmware", severity: Severity::Critical },

            // NVIDIA
            FirmwareRequirement { pattern: "nvidia/*", min_files: 30, reason: "NVIDIA GPU firmware", severity: Severity::High },

            // Intel WiFi
            FirmwareRequirement { pattern: "iwlwifi-*", min_files: 100, reason: "Intel WiFi firmware", severity: Severity::Critical },

            // Qualcomm WiFi
            FirmwareRequirement { pattern: "ath11k/*", min_files: 10, reason: "Qualcomm WiFi firmware", severity: Severity::High },

            // MediaTek WiFi
            FirmwareRequirement { pattern: "mediatek/*", min_files: 10, reason: "MediaTek WiFi firmware", severity: Severity::High },

            // Intel Bluetooth
            FirmwareRequirement { pattern: "intel/ibt-*", min_files: 50, reason: "Intel Bluetooth firmware", severity: Severity::High },

            // CPU Microcode
            FirmwareRequirement { pattern: "intel-ucode/*", min_files: 100, reason: "Intel microcode", severity: Severity::Critical },
            FirmwareRequirement { pattern: "amd-ucode/*", min_files: 1, reason: "AMD microcode", severity: Severity::Critical },
        ]
    }
}
