use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

/// Gaming Desktop Profile (2020-2026)
///
/// Covers: Custom gaming builds, pre-built gaming PCs (NZXT, CyberPower, iBuyPower)
/// CPU: Intel 10th-14th gen, AMD Ryzen 5000-9000
/// GPU: NVIDIA RTX 30/40 series, AMD RX 6000/7000 series
/// Features: High-refresh displays, RGB peripherals, fast NVMe, liquid cooling
pub struct GamingDesktopProfile;

impl HardwareProfile for GamingDesktopProfile {
    fn name(&self) -> &str { "Gaming Desktop" }
    fn description(&self) -> &str { "High-performance gaming PCs - RTX 40/RX 7000, fast storage" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU - Gaming Graphics ===
            // NVIDIA (RTX 20/30/40 series) - nouveau has limited support, but proprietary driver needs these
            KernelRequirement { config: "CONFIG_DRM_NOUVEAU", expected: ConfigValue::M, reason: "NVIDIA nouveau (basic support, use proprietary for gaming)", severity: Severity::Critical },
            // AMD (RX 5000/6000/7000 series) - fully supported
            KernelRequirement { config: "CONFIG_DRM_AMDGPU", expected: ConfigValue::M, reason: "AMD Radeon RX 5000/6000/7000", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_AMD_DC", expected: ConfigValue::Y, reason: "AMD Display Core (FreeSync)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_AMD_DC_FP", expected: ConfigValue::Y, reason: "AMD DC floating point", severity: Severity::High },
            // Intel Arc
            KernelRequirement { config: "CONFIG_DRM_I915", expected: ConfigValue::M, reason: "Intel Arc A-series", severity: Severity::High },
            KernelRequirement { config: "CONFIG_DRM_XE", expected: ConfigValue::M, reason: "Intel Xe (Arc discrete)", severity: Severity::Medium },
            // Core DRM
            KernelRequirement { config: "CONFIG_DRM_KMS_HELPER", expected: ConfigValue::Y, reason: "Kernel mode setting", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_FBDEV_EMULATION", expected: ConfigValue::Y, reason: "Framebuffer emulation", severity: Severity::High },
            KernelRequirement { config: "CONFIG_FB_EFI", expected: ConfigValue::Y, reason: "EFI framebuffer (boot)", severity: Severity::High },

            // === Storage - Fast NVMe ===
            KernelRequirement { config: "CONFIG_NVME_CORE", expected: ConfigValue::Y, reason: "NVMe SSD (Gen4/Gen5)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BLK_DEV_NVME", expected: ConfigValue::Y, reason: "NVMe block device", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_NVME_HWMON", expected: ConfigValue::Y, reason: "NVMe temperature monitoring", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SATA_AHCI", expected: ConfigValue::Y, reason: "SATA (game storage HDDs)", severity: Severity::High },

            // === Ethernet - Gaming grade ===
            KernelRequirement { config: "CONFIG_R8169", expected: ConfigValue::Y, reason: "Realtek 2.5GbE (most gaming boards)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_IGC", expected: ConfigValue::Y, reason: "Intel I225/I226 2.5GbE", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IGB", expected: ConfigValue::Y, reason: "Intel I210/I211 GbE", severity: Severity::High },
            KernelRequirement { config: "CONFIG_AQTION", expected: ConfigValue::M, reason: "Aquantia 10GbE (high-end boards)", severity: Severity::Medium },

            // === WiFi (PCIe cards) ===
            KernelRequirement { config: "CONFIG_MAC80211", expected: ConfigValue::M, reason: "802.11 wireless stack", severity: Severity::High },
            KernelRequirement { config: "CONFIG_CFG80211", expected: ConfigValue::M, reason: "Wireless configuration", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IWLWIFI", expected: ConfigValue::M, reason: "Intel WiFi 6E/7", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MT7921E", expected: ConfigValue::M, reason: "MediaTek WiFi 6E", severity: Severity::High },
            KernelRequirement { config: "CONFIG_RTW89", expected: ConfigValue::M, reason: "Realtek WiFi 6", severity: Severity::High },

            // === Bluetooth (for controllers) ===
            KernelRequirement { config: "CONFIG_BT", expected: ConfigValue::M, reason: "Bluetooth (wireless controllers)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_HCIBTUSB", expected: ConfigValue::M, reason: "Bluetooth USB", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_HIDP", expected: ConfigValue::M, reason: "Bluetooth HID (controllers)", severity: Severity::High },

            // === Audio - Gaming headsets, DACs ===
            KernelRequirement { config: "CONFIG_SND_HDA_INTEL", expected: ConfigValue::Y, reason: "HD Audio", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_REALTEK", expected: ConfigValue::Y, reason: "Realtek codec", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_HDMI", expected: ConfigValue::Y, reason: "HDMI audio", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_USB_AUDIO", expected: ConfigValue::M, reason: "USB DACs/headsets", severity: Severity::High },

            // === USB - Peripherals ===
            KernelRequirement { config: "CONFIG_USB_XHCI_HCD", expected: ConfigValue::Y, reason: "USB 3.x", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_XHCI_PCI", expected: ConfigValue::Y, reason: "xHCI PCI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_STORAGE", expected: ConfigValue::M, reason: "USB storage", severity: Severity::High },

            // === Input - Gaming peripherals ===
            KernelRequirement { config: "CONFIG_HID", expected: ConfigValue::Y, reason: "HID core", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_HID_GENERIC", expected: ConfigValue::Y, reason: "Generic HID", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_HID", expected: ConfigValue::Y, reason: "USB HID", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_HID_LOGITECH", expected: ConfigValue::M, reason: "Logitech gaming peripherals", severity: Severity::High },
            KernelRequirement { config: "CONFIG_HID_LOGITECH_DJ", expected: ConfigValue::M, reason: "Logitech Unifying receiver", severity: Severity::High },
            KernelRequirement { config: "CONFIG_HID_LOGITECH_HIDPP", expected: ConfigValue::M, reason: "Logitech HID++ (G-series)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_HID_RAZER", expected: ConfigValue::M, reason: "Razer peripherals", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_HID_CORSAIR", expected: ConfigValue::M, reason: "Corsair peripherals", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_HID_STEELSERIES", expected: ConfigValue::M, reason: "SteelSeries peripherals", severity: Severity::Medium },

            // === Game Controllers ===
            KernelRequirement { config: "CONFIG_INPUT_JOYDEV", expected: ConfigValue::M, reason: "Joystick interface", severity: Severity::High },
            KernelRequirement { config: "CONFIG_INPUT_EVDEV", expected: ConfigValue::Y, reason: "Event device", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_HID_SONY", expected: ConfigValue::M, reason: "PlayStation controllers", severity: Severity::High },
            KernelRequirement { config: "CONFIG_HID_MICROSOFT", expected: ConfigValue::M, reason: "Xbox controllers", severity: Severity::High },
            KernelRequirement { config: "CONFIG_HID_NINTENDO", expected: ConfigValue::M, reason: "Nintendo controllers", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_JOYSTICK_XPAD", expected: ConfigValue::M, reason: "Xbox gamepad driver", severity: Severity::High },

            // === Power/Cooling ===
            KernelRequirement { config: "CONFIG_ACPI", expected: ConfigValue::Y, reason: "ACPI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_CPU_FREQ", expected: ConfigValue::Y, reason: "CPU frequency scaling", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_INTEL_PSTATE", expected: ConfigValue::Y, reason: "Intel P-state", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_AMD_PSTATE", expected: ConfigValue::Y, reason: "AMD P-state", severity: Severity::High },

            // === Sensors - Temps, fan control ===
            KernelRequirement { config: "CONFIG_HWMON", expected: ConfigValue::Y, reason: "Hardware monitoring", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_CORETEMP", expected: ConfigValue::M, reason: "Intel CPU temp", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_K10TEMP", expected: ConfigValue::M, reason: "AMD CPU temp", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_NCT6775", expected: ConfigValue::M, reason: "Nuvoton Super I/O", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_IT87", expected: ConfigValue::M, reason: "ITE Super I/O", severity: Severity::High },

            // === RGB Control (basic kernel support) ===
            KernelRequirement { config: "CONFIG_LEDS_CLASS", expected: ConfigValue::Y, reason: "LED class (RGB)", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_LEDS_CLASS_MULTICOLOR", expected: ConfigValue::M, reason: "Multi-color LED", severity: Severity::Medium },

            // === Virtualization (for game streaming, VMs) ===
            KernelRequirement { config: "CONFIG_KVM", expected: ConfigValue::M, reason: "KVM virtualization", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_KVM_INTEL", expected: ConfigValue::M, reason: "KVM Intel", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_KVM_AMD", expected: ConfigValue::M, reason: "KVM AMD", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_VFIO", expected: ConfigValue::M, reason: "VFIO (GPU passthrough)", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_VFIO_PCI", expected: ConfigValue::M, reason: "VFIO PCI", severity: Severity::Medium },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // AMD GPU
            FirmwareRequirement { pattern: "amdgpu/*", min_files: 600, reason: "AMD GPU firmware", severity: Severity::Critical },

            // NVIDIA (nouveau)
            FirmwareRequirement { pattern: "nvidia/*", min_files: 30, reason: "NVIDIA GPU firmware", severity: Severity::Critical },

            // Intel Arc
            FirmwareRequirement { pattern: "i915/*", min_files: 100, reason: "Intel GPU firmware", severity: Severity::High },

            // WiFi
            FirmwareRequirement { pattern: "iwlwifi-*", min_files: 100, reason: "Intel WiFi", severity: Severity::High },
            FirmwareRequirement { pattern: "mediatek/*", min_files: 10, reason: "MediaTek WiFi", severity: Severity::High },
            FirmwareRequirement { pattern: "rtw89/*", min_files: 10, reason: "Realtek WiFi 6", severity: Severity::High },

            // Bluetooth
            FirmwareRequirement { pattern: "intel/ibt-*", min_files: 50, reason: "Intel Bluetooth", severity: Severity::High },
            FirmwareRequirement { pattern: "rtl_bt/*", min_files: 20, reason: "Realtek Bluetooth", severity: Severity::High },

            // CPU Microcode
            FirmwareRequirement { pattern: "intel-ucode/*", min_files: 100, reason: "Intel microcode", severity: Severity::Critical },
            FirmwareRequirement { pattern: "amd-ucode/*", min_files: 1, reason: "AMD microcode", severity: Severity::Critical },
        ]
    }
}
