use crate::firmware::FirmwareRequirement;
use crate::kernel::{ConfigValue, KernelRequirement};
use crate::profiles::HardwareProfile;
use crate::report::Severity;

/// Gaming Laptop Profile (2020-2026)
///
/// Covers: ASUS ROG, Razer Blade, MSI, Alienware, Lenovo Legion, HP Omen, Acer Predator
/// CPU: Intel 10th-14th gen H-series, AMD Ryzen 6000-8000 HS/HX
/// GPU: NVIDIA RTX 30/40 Mobile, AMD RX 6000M/7000M
/// Features: High-refresh displays, hybrid graphics, advanced cooling
pub struct GamingLaptopProfile;

impl HardwareProfile for GamingLaptopProfile {
    fn name(&self) -> &str {
        "Gaming Laptop (ROG, Razer, Legion)"
    }
    fn description(&self) -> &str {
        "High-performance gaming laptops - RTX Mobile, Ryzen HX"
    }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU - Hybrid Graphics ===
            // Integrated (always present)
            KernelRequirement {
                config: "CONFIG_DRM_I915",
                expected: ConfigValue::M,
                reason: "Intel integrated (hybrid)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_DRM_AMDGPU",
                expected: ConfigValue::M,
                reason: "AMD integrated (hybrid)",
                severity: Severity::Critical,
            },
            // Discrete
            KernelRequirement {
                config: "CONFIG_DRM_NOUVEAU",
                expected: ConfigValue::M,
                reason: "NVIDIA discrete (nouveau)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_DRM_AMD_DC",
                expected: ConfigValue::Y,
                reason: "AMD Display Core",
                severity: Severity::Critical,
            },
            // Core DRM
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
            // MUX switch support
            KernelRequirement {
                config: "CONFIG_VGA_SWITCHEROO",
                expected: ConfigValue::Y,
                reason: "GPU switching (Optimus/MUX)",
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
            KernelRequirement {
                config: "CONFIG_NVME_HWMON",
                expected: ConfigValue::Y,
                reason: "NVMe temperature",
                severity: Severity::Medium,
            },
            // === Ethernet ===
            KernelRequirement {
                config: "CONFIG_R8169",
                expected: ConfigValue::Y,
                reason: "Realtek Ethernet",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_E1000E",
                expected: ConfigValue::Y,
                reason: "Intel Ethernet",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_USB_RTL8152",
                expected: ConfigValue::M,
                reason: "USB Ethernet (docks)",
                severity: Severity::High,
            },
            // === WiFi (critical for laptops) ===
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
                reason: "Intel WiFi 6E/7 (Killer)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_IWLMVM",
                expected: ConfigValue::M,
                reason: "Intel WiFi MVM",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_MT7921E",
                expected: ConfigValue::M,
                reason: "MediaTek WiFi 6E",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_RTW89",
                expected: ConfigValue::M,
                reason: "Realtek WiFi 6",
                severity: Severity::High,
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
            KernelRequirement {
                config: "CONFIG_BT_MTK",
                expected: ConfigValue::M,
                reason: "MediaTek Bluetooth",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_BT_HIDP",
                expected: ConfigValue::M,
                reason: "Bluetooth HID",
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
                reason: "HDMI audio",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_SOC",
                expected: ConfigValue::M,
                reason: "SoC audio",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_SND_USB_AUDIO",
                expected: ConfigValue::M,
                reason: "USB audio",
                severity: Severity::High,
            },
            // === USB / Thunderbolt ===
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
                config: "CONFIG_USB4",
                expected: ConfigValue::M,
                reason: "USB4/Thunderbolt",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_USB4",
                expected: ConfigValue::M,
                reason: "Thunderbolt bus",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_TYPEC",
                expected: ConfigValue::M,
                reason: "USB Type-C",
                severity: Severity::High,
            },
            // === Input - Keyboard, touchpad ===
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
                reason: "I2C HID (touchpad)",
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
            // === Vendor-specific platform drivers ===
            KernelRequirement {
                config: "CONFIG_ASUS_WMI",
                expected: ConfigValue::M,
                reason: "ASUS ROG platform",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_ASUS_NB_WMI",
                expected: ConfigValue::M,
                reason: "ASUS notebook WMI",
                severity: Severity::High,
            },
            // Note: Razer devices require OpenRazer (out-of-tree DKMS driver, not in mainline kernel)

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
                config: "CONFIG_X86_AMD_PSTATE",
                expected: ConfigValue::Y,
                reason: "AMD P-state",
                severity: Severity::High,
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
            // === Sensors ===
            KernelRequirement {
                config: "CONFIG_HWMON",
                expected: ConfigValue::Y,
                reason: "Hardware monitoring",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SENSORS_CORETEMP",
                expected: ConfigValue::M,
                reason: "Intel CPU temp",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SENSORS_K10TEMP",
                expected: ConfigValue::M,
                reason: "AMD CPU temp",
                severity: Severity::High,
            },
            // === Game Controllers ===
            KernelRequirement {
                config: "CONFIG_INPUT_JOYDEV",
                expected: ConfigValue::M,
                reason: "Joystick",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_HID_SONY",
                expected: ConfigValue::M,
                reason: "PlayStation controller",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_HID_MICROSOFT",
                expected: ConfigValue::M,
                reason: "Xbox controller",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_JOYSTICK_XPAD",
                expected: ConfigValue::M,
                reason: "Xbox gamepad",
                severity: Severity::High,
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
            // AMD GPU
            FirmwareRequirement {
                pattern: "amdgpu/*",
                min_files: 600,
                reason: "AMD GPU firmware",
                severity: Severity::Critical,
            },
            // NVIDIA
            FirmwareRequirement {
                pattern: "nvidia/*",
                min_files: 30,
                reason: "NVIDIA GPU firmware",
                severity: Severity::Critical,
            },
            // WiFi
            FirmwareRequirement {
                pattern: "iwlwifi-*",
                min_files: 100,
                reason: "Intel WiFi (Killer)",
                severity: Severity::Critical,
            },
            FirmwareRequirement {
                pattern: "mediatek/*",
                min_files: 10,
                reason: "MediaTek WiFi",
                severity: Severity::High,
            },
            FirmwareRequirement {
                pattern: "rtw89/*",
                min_files: 10,
                reason: "Realtek WiFi 6",
                severity: Severity::High,
            },
            // Bluetooth
            FirmwareRequirement {
                pattern: "intel/ibt-*",
                min_files: 50,
                reason: "Intel Bluetooth",
                severity: Severity::High,
            },
            FirmwareRequirement {
                pattern: "mediatek/BT_*",
                min_files: 1,
                reason: "MediaTek Bluetooth",
                severity: Severity::High,
            },
            // CPU Microcode
            FirmwareRequirement {
                pattern: "intel-ucode/*",
                min_files: 100,
                reason: "Intel microcode",
                severity: Severity::Critical,
            },
            FirmwareRequirement {
                pattern: "amd-ucode/*",
                min_files: 1,
                reason: "AMD microcode",
                severity: Severity::Critical,
            },
        ]
    }
}
