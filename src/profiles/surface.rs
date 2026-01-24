use crate::firmware::FirmwareRequirement;
use crate::kernel::{ConfigValue, KernelRequirement};
use crate::profiles::HardwareProfile;
use crate::report::Severity;

/// Microsoft Surface Profile (2018-2026)
///
/// Covers: Surface Pro 6-9, Surface Laptop 3-5, Surface Book 2-3, Surface Go 2-3
/// Known for: Requires linux-surface kernel patches for full support
/// Features: Touchscreen, Surface Pen, detachable keyboard, cameras
pub struct SurfaceProfile;

impl HardwareProfile for SurfaceProfile {
    fn name(&self) -> &str {
        "Microsoft Surface (Pro/Laptop/Go)"
    }
    fn description(&self) -> &str {
        "Surface devices - requires linux-surface patches for full support"
    }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU ===
            KernelRequirement {
                config: "CONFIG_DRM_I915",
                expected: ConfigValue::M,
                reason: "Intel integrated graphics",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_DRM_NOUVEAU",
                expected: ConfigValue::M,
                reason: "NVIDIA dGPU (Surface Book)",
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
            // eMMC (Surface Go)
            KernelRequirement {
                config: "CONFIG_MMC",
                expected: ConfigValue::Y,
                reason: "eMMC (Surface Go)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_MMC_SDHCI",
                expected: ConfigValue::Y,
                reason: "SDHCI controller",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_MMC_SDHCI_ACPI",
                expected: ConfigValue::M,
                reason: "SDHCI ACPI",
                severity: Severity::High,
            },
            // === WiFi - Marvell/Qualcomm ===
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
                config: "CONFIG_MWIFIEX",
                expected: ConfigValue::M,
                reason: "Marvell WiFi (older Surface)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_MWIFIEX_PCIE",
                expected: ConfigValue::M,
                reason: "Marvell WiFi PCIe",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_ATH10K",
                expected: ConfigValue::M,
                reason: "Qualcomm WiFi (newer Surface)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_ATH10K_PCI",
                expected: ConfigValue::M,
                reason: "Qualcomm WiFi PCI",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_IWLWIFI",
                expected: ConfigValue::M,
                reason: "Intel WiFi (some models)",
                severity: Severity::Medium,
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
                config: "CONFIG_BT_MRVL",
                expected: ConfigValue::M,
                reason: "Marvell Bluetooth",
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
                config: "CONFIG_SND_SOC",
                expected: ConfigValue::M,
                reason: "SoC audio",
                severity: Severity::Medium,
            },
            // === USB / Type-C ===
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
                config: "CONFIG_TYPEC",
                expected: ConfigValue::M,
                reason: "USB Type-C",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_TYPEC_DP_ALTMODE",
                expected: ConfigValue::M,
                reason: "USB-C DisplayPort",
                severity: Severity::High,
            },
            // === Input - Touchscreen, Pen, Type Cover ===
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
                reason: "I2C HID (touchscreen)",
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
            // Touchscreen
            KernelRequirement {
                config: "CONFIG_INPUT_TOUCHSCREEN",
                expected: ConfigValue::Y,
                reason: "Touchscreen input",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_HID_MULTITOUCH",
                expected: ConfigValue::M,
                reason: "Multitouch",
                severity: Severity::Critical,
            },
            // Surface Pen (linux-surface specific, but base HID needed)
            KernelRequirement {
                config: "CONFIG_HID_BATTERY_STRENGTH",
                expected: ConfigValue::Y,
                reason: "HID battery (Pen)",
                severity: Severity::High,
            },
            // === Surface-specific (linux-surface patches) ===
            // These may not exist in mainline kernel
            KernelRequirement {
                config: "CONFIG_SURFACE_AGGREGATOR",
                expected: ConfigValue::M,
                reason: "Surface Aggregator Module",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_SURFACE_AGGREGATOR_HUB",
                expected: ConfigValue::M,
                reason: "SAM hub",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_SURFACE_AGGREGATOR_REGISTRY",
                expected: ConfigValue::M,
                reason: "SAM registry",
                severity: Severity::Medium,
            },
            // === Cameras (IPU3/IPU6) ===
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
            // IPU3/IPU6 cameras need additional out-of-tree support

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
                config: "CONFIG_SUSPEND",
                expected: ConfigValue::Y,
                reason: "Suspend",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_PM_SLEEP",
                expected: ConfigValue::Y,
                reason: "PM sleep",
                severity: Severity::High,
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
            // Accelerometer/Gyro for screen rotation
            KernelRequirement {
                config: "CONFIG_IIO",
                expected: ConfigValue::M,
                reason: "IIO subsystem",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_HID_SENSOR_HUB",
                expected: ConfigValue::M,
                reason: "HID sensor hub",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_HID_SENSOR_ACCEL_3D",
                expected: ConfigValue::M,
                reason: "HID accelerometer",
                severity: Severity::High,
            },
            // === TPM ===
            KernelRequirement {
                config: "CONFIG_TCG_TPM",
                expected: ConfigValue::Y,
                reason: "TPM 2.0",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_TCG_TIS",
                expected: ConfigValue::Y,
                reason: "TPM TIS",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_TCG_CRB",
                expected: ConfigValue::Y,
                reason: "TPM CRB",
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
            // NVIDIA (Surface Book)
            FirmwareRequirement {
                pattern: "nvidia/*",
                min_files: 30,
                reason: "NVIDIA GPU firmware",
                severity: Severity::High,
            },
            // Marvell WiFi
            FirmwareRequirement {
                pattern: "mrvl/*",
                min_files: 10,
                reason: "Marvell WiFi firmware",
                severity: Severity::High,
            },
            // Qualcomm WiFi
            FirmwareRequirement {
                pattern: "ath10k/*",
                min_files: 10,
                reason: "Qualcomm WiFi firmware",
                severity: Severity::High,
            },
            // Intel WiFi
            FirmwareRequirement {
                pattern: "iwlwifi-*",
                min_files: 100,
                reason: "Intel WiFi firmware",
                severity: Severity::Medium,
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
