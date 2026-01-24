use crate::firmware::FirmwareRequirement;
use crate::kernel::{ConfigValue, KernelRequirement};
use crate::profiles::HardwareProfile;
use crate::report::Severity;

/// Framework Laptop Profile (13" and 16")
///
/// Covers: Framework Laptop 13 (Intel 11th-13th gen, AMD Ryzen 7040)
///         Framework Laptop 16 (AMD Ryzen 7040HS)
/// Known for: Modular design, repairability, excellent Linux support
/// Key features: Thunderbolt 4, USB4, expansion cards, upgradeable
pub struct FrameworkProfile;

impl HardwareProfile for FrameworkProfile {
    fn name(&self) -> &str {
        "Framework Laptop (13\" & 16\")"
    }
    fn description(&self) -> &str {
        "Modular laptops - Intel and AMD variants"
    }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU ===
            // Intel variants (Framework 13 Intel)
            KernelRequirement {
                config: "CONFIG_DRM_I915",
                expected: ConfigValue::M,
                reason: "Intel Iris Xe (Framework 13 Intel)",
                severity: Severity::Critical,
            },
            // AMD variants (Framework 13 AMD, Framework 16)
            KernelRequirement {
                config: "CONFIG_DRM_AMDGPU",
                expected: ConfigValue::M,
                reason: "AMD Radeon (Framework 13 AMD, Framework 16)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_DRM_KMS_HELPER",
                expected: ConfigValue::Y,
                reason: "Kernel mode setting",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_FB",
                expected: ConfigValue::Y,
                reason: "Framebuffer support",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_BACKLIGHT_CLASS_DEVICE",
                expected: ConfigValue::Y,
                reason: "Display backlight control",
                severity: Severity::High,
            },
            // === Storage (NVMe only - no SATA) ===
            KernelRequirement {
                config: "CONFIG_NVME_CORE",
                expected: ConfigValue::Y,
                reason: "NVMe SSD (only storage option)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_BLK_DEV_NVME",
                expected: ConfigValue::Y,
                reason: "NVMe block device",
                severity: Severity::Critical,
            },
            // === Thunderbolt/USB4 (CRITICAL for Framework) ===
            KernelRequirement {
                config: "CONFIG_USB4",
                expected: ConfigValue::M,
                reason: "USB4/Thunderbolt 4 (expansion cards)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB4",
                expected: ConfigValue::M,
                reason: "Thunderbolt bus driver",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_TYPEC",
                expected: ConfigValue::M,
                reason: "USB Type-C support",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_TYPEC_UCSI",
                expected: ConfigValue::M,
                reason: "USB Type-C UCSI",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_TYPEC_TPS6598X",
                expected: ConfigValue::M,
                reason: "TI TPS6598x USB PD controller",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_USB_ROLE_SWITCH",
                expected: ConfigValue::M,
                reason: "USB role switching",
                severity: Severity::High,
            },
            // === USB ===
            KernelRequirement {
                config: "CONFIG_USB_XHCI_HCD",
                expected: ConfigValue::Y,
                reason: "USB 3.x (xHCI)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB_XHCI_PCI",
                expected: ConfigValue::Y,
                reason: "xHCI PCI support",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB_STORAGE",
                expected: ConfigValue::M,
                reason: "USB mass storage (expansion cards)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_USB_UAS",
                expected: ConfigValue::M,
                reason: "USB Attached SCSI",
                severity: Severity::Medium,
            },
            // === WiFi ===
            // Intel variants
            KernelRequirement {
                config: "CONFIG_IWLWIFI",
                expected: ConfigValue::M,
                reason: "Intel WiFi 6E AX210 (Intel variant)",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_IWLMVM",
                expected: ConfigValue::M,
                reason: "Intel WiFi MVM firmware",
                severity: Severity::Critical,
            },
            // AMD variants use MediaTek
            KernelRequirement {
                config: "CONFIG_MT7921E",
                expected: ConfigValue::M,
                reason: "MediaTek MT7922 WiFi 6E (AMD variant)",
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
                reason: "Bluetooth HCI USB",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_BT_INTEL",
                expected: ConfigValue::M,
                reason: "Intel Bluetooth (Intel variant)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_BT_MTK",
                expected: ConfigValue::M,
                reason: "MediaTek Bluetooth (AMD variant)",
                severity: Severity::High,
            },
            // === Audio ===
            KernelRequirement {
                config: "CONFIG_SND_HDA_INTEL",
                expected: ConfigValue::Y,
                reason: "Intel HD Audio (Intel variant)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_SOC",
                expected: ConfigValue::M,
                reason: "SoC audio framework",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_SOC_AMD_ACP",
                expected: ConfigValue::M,
                reason: "AMD Audio CoProcessor (AMD variant)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_SOC_AMD_PS",
                expected: ConfigValue::M,
                reason: "AMD Pink Sardine audio (Ryzen 7040)",
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
                reason: "HDMI/DP audio (via USB-C)",
                severity: Severity::High,
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
                reason: "Generic HID driver",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_USB_HID",
                expected: ConfigValue::Y,
                reason: "USB HID support",
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
                reason: "Event device interface",
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
                config: "CONFIG_ACPI_BATTERY",
                expected: ConfigValue::M,
                reason: "Battery status",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_ACPI_AC",
                expected: ConfigValue::M,
                reason: "AC adapter status",
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
                reason: "Intel P-state (Intel variant)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_X86_AMD_PSTATE",
                expected: ConfigValue::Y,
                reason: "AMD P-state (AMD variant)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SUSPEND",
                expected: ConfigValue::Y,
                reason: "Suspend to RAM",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_HIBERNATION",
                expected: ConfigValue::Y,
                reason: "Hibernate to disk",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_PM_SLEEP",
                expected: ConfigValue::Y,
                reason: "Power management sleep",
                severity: Severity::High,
            },
            // === Webcam ===
            KernelRequirement {
                config: "CONFIG_MEDIA_USB_SUPPORT",
                expected: ConfigValue::Y,
                reason: "USB camera support",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_USB_VIDEO_CLASS",
                expected: ConfigValue::M,
                reason: "UVC webcam driver",
                severity: Severity::High,
            },
            // IPU6 camera (newer Intel variants) - may require out-of-tree drivers
            KernelRequirement {
                config: "CONFIG_INTEL_VSC",
                expected: ConfigValue::M,
                reason: "Intel Visual Sensing Controller (webcam privacy)",
                severity: Severity::Medium,
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
                reason: "Intel CPU temperature",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_SENSORS_K10TEMP",
                expected: ConfigValue::M,
                reason: "AMD CPU temperature",
                severity: Severity::Medium,
            },
            // === Framework EC (embedded controller) ===
            KernelRequirement {
                config: "CONFIG_CROS_EC",
                expected: ConfigValue::M,
                reason: "Chrome OS EC (Framework uses similar)",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_CROS_EC_LPC",
                expected: ConfigValue::M,
                reason: "EC LPC interface",
                severity: Severity::Medium,
            },
            // === Fingerprint Reader ===
            KernelRequirement {
                config: "CONFIG_USB_HIDDEV",
                expected: ConfigValue::Y,
                reason: "USB HID device (fingerprint module)",
                severity: Severity::Low,
            },
            // === Expansion Card Support (various) ===
            KernelRequirement {
                config: "CONFIG_USB_RTL8152",
                expected: ConfigValue::M,
                reason: "Realtek USB Ethernet (Ethernet expansion)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SND_USB_AUDIO",
                expected: ConfigValue::M,
                reason: "USB audio (Audio expansion)",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_USB_SERIAL",
                expected: ConfigValue::M,
                reason: "USB serial (various expansions)",
                severity: Severity::Medium,
            },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // Intel GPU (Intel variants)
            FirmwareRequirement {
                pattern: "i915/*",
                min_files: 100,
                reason: "Intel GPU firmware (Iris Xe)",
                severity: Severity::Critical,
            },
            // AMD GPU (AMD variants)
            FirmwareRequirement {
                pattern: "amdgpu/*",
                min_files: 500,
                reason: "AMD GPU firmware (Radeon 780M/760M)",
                severity: Severity::Critical,
            },
            // Intel WiFi (Intel variants)
            FirmwareRequirement {
                pattern: "iwlwifi-*",
                min_files: 100,
                reason: "Intel WiFi 6E AX210 firmware",
                severity: Severity::Critical,
            },
            // MediaTek WiFi (AMD variants)
            FirmwareRequirement {
                pattern: "mediatek/WIFI_*",
                min_files: 5,
                reason: "MediaTek MT7922 WiFi firmware",
                severity: Severity::Critical,
            },
            // Intel Bluetooth (Intel variants)
            FirmwareRequirement {
                pattern: "intel/ibt-*",
                min_files: 50,
                reason: "Intel Bluetooth firmware",
                severity: Severity::High,
            },
            // MediaTek Bluetooth (AMD variants)
            FirmwareRequirement {
                pattern: "mediatek/BT_*",
                min_files: 1,
                reason: "MediaTek Bluetooth firmware",
                severity: Severity::High,
            },
            // CPU Microcode
            FirmwareRequirement {
                pattern: "intel-ucode/*",
                min_files: 100,
                reason: "Intel CPU microcode",
                severity: Severity::Critical,
            },
            FirmwareRequirement {
                pattern: "amd-ucode/*",
                min_files: 1,
                reason: "AMD CPU microcode (Ryzen 7040)",
                severity: Severity::Critical,
            },
            // AMD audio firmware
            FirmwareRequirement {
                pattern: "amdgpu/psp_*",
                min_files: 10,
                reason: "AMD PSP firmware",
                severity: Severity::High,
            },
        ]
    }
}
