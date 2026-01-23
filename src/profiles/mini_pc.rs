use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

/// AMD Mini PC Profile (2020-2026)
///
/// Covers: Beelink, Minisforum, GMKtec, ASUS PN series, Geekom
/// CPU: AMD Ryzen 5000/6000/7000/8000 series APUs
/// GPU: AMD Radeon Vega/RDNA2/RDNA3 integrated
/// Common uses: Desktop replacement, home server, HTPC
pub struct AmdMiniPcProfile;

impl HardwareProfile for AmdMiniPcProfile {
    fn name(&self) -> &str { "AMD Mini PC (Beelink, Minisforum, etc.)" }
    fn description(&self) -> &str { "Compact AMD Ryzen systems - Ryzen 5000-8000 APUs" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU (AMD integrated) ===
            KernelRequirement { config: "CONFIG_DRM_AMDGPU", expected: ConfigValue::M, reason: "AMD Radeon (Vega/RDNA2/RDNA3 integrated)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_AMD_DC", expected: ConfigValue::Y, reason: "AMD Display Core", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_AMD_DC_FP", expected: ConfigValue::Y, reason: "AMD DC floating point", severity: Severity::High },
            KernelRequirement { config: "CONFIG_DRM_KMS_HELPER", expected: ConfigValue::Y, reason: "Kernel mode setting", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_FB", expected: ConfigValue::Y, reason: "Framebuffer support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_FB_EFI", expected: ConfigValue::Y, reason: "EFI framebuffer", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BACKLIGHT_CLASS_DEVICE", expected: ConfigValue::Y, reason: "Backlight control (HDMI CEC)", severity: Severity::Medium },

            // === Storage ===
            KernelRequirement { config: "CONFIG_NVME_CORE", expected: ConfigValue::Y, reason: "NVMe SSD (M.2 slot)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BLK_DEV_NVME", expected: ConfigValue::Y, reason: "NVMe block device", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SATA_AHCI", expected: ConfigValue::Y, reason: "SATA SSD/HDD (2.5\" bay)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_ATA", expected: ConfigValue::Y, reason: "ATA subsystem", severity: Severity::High },

            // === Ethernet ===
            // Realtek (most common in mini PCs)
            KernelRequirement { config: "CONFIG_R8169", expected: ConfigValue::Y, reason: "Realtek RTL8111/8125 (1/2.5 GbE)", severity: Severity::Critical },
            // Intel (some models)
            KernelRequirement { config: "CONFIG_E1000E", expected: ConfigValue::Y, reason: "Intel I219 GbE (some models)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IGC", expected: ConfigValue::Y, reason: "Intel I225/I226 2.5GbE (some models)", severity: Severity::High },
            // USB Ethernet (docking stations)
            KernelRequirement { config: "CONFIG_USB_RTL8152", expected: ConfigValue::M, reason: "Realtek USB Ethernet", severity: Severity::High },

            // === WiFi ===
            KernelRequirement { config: "CONFIG_MAC80211", expected: ConfigValue::M, reason: "802.11 wireless stack", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_CFG80211", expected: ConfigValue::M, reason: "Wireless configuration API", severity: Severity::Critical },
            // Intel (common option)
            KernelRequirement { config: "CONFIG_IWLWIFI", expected: ConfigValue::M, reason: "Intel WiFi 6/6E", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IWLMVM", expected: ConfigValue::M, reason: "Intel WiFi MVM", severity: Severity::High },
            // MediaTek (common in AMD mini PCs)
            KernelRequirement { config: "CONFIG_MT7921E", expected: ConfigValue::M, reason: "MediaTek MT7921/7922 WiFi 6E", severity: Severity::Critical },
            // Realtek (common)
            KernelRequirement { config: "CONFIG_RTW88", expected: ConfigValue::M, reason: "Realtek RTL8822/8821 WiFi", severity: Severity::High },
            KernelRequirement { config: "CONFIG_RTW89", expected: ConfigValue::M, reason: "Realtek RTL8852 WiFi 6", severity: Severity::High },

            // === Bluetooth ===
            KernelRequirement { config: "CONFIG_BT", expected: ConfigValue::M, reason: "Bluetooth subsystem", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_HCIBTUSB", expected: ConfigValue::M, reason: "Bluetooth HCI USB", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_INTEL", expected: ConfigValue::M, reason: "Intel Bluetooth", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_MTK", expected: ConfigValue::M, reason: "MediaTek Bluetooth", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_RTL", expected: ConfigValue::M, reason: "Realtek Bluetooth", severity: Severity::High },

            // === Audio ===
            // AMD HD Audio
            KernelRequirement { config: "CONFIG_SND_HDA_INTEL", expected: ConfigValue::Y, reason: "HD Audio controller", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_REALTEK", expected: ConfigValue::Y, reason: "Realtek codec", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_HDMI", expected: ConfigValue::Y, reason: "HDMI/DP audio", severity: Severity::High },
            // AMD SoC audio (newer APUs)
            KernelRequirement { config: "CONFIG_SND_SOC", expected: ConfigValue::M, reason: "SoC audio framework", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SND_SOC_AMD_ACP", expected: ConfigValue::M, reason: "AMD Audio CoProcessor", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SND_SOC_AMD_ACP3X", expected: ConfigValue::M, reason: "AMD ACP3x (Renoir/Cezanne)", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SND_SOC_AMD_ACP5X", expected: ConfigValue::M, reason: "AMD ACP5x", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SND_SOC_AMD_ACP6X", expected: ConfigValue::M, reason: "AMD ACP6x (Rembrandt)", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SND_SOC_AMD_PS", expected: ConfigValue::M, reason: "AMD Pink Sardine (Phoenix)", severity: Severity::Medium },

            // === USB ===
            KernelRequirement { config: "CONFIG_USB_XHCI_HCD", expected: ConfigValue::Y, reason: "USB 3.x (xHCI)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_XHCI_PCI", expected: ConfigValue::Y, reason: "xHCI PCI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_EHCI_HCD", expected: ConfigValue::Y, reason: "USB 2.0 (EHCI)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_STORAGE", expected: ConfigValue::M, reason: "USB mass storage", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_UAS", expected: ConfigValue::M, reason: "USB Attached SCSI", severity: Severity::High },

            // === USB-C / USB4 (some newer models) ===
            KernelRequirement { config: "CONFIG_TYPEC", expected: ConfigValue::M, reason: "USB Type-C support", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_TYPEC_UCSI", expected: ConfigValue::M, reason: "USB Type-C UCSI", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_USB4", expected: ConfigValue::M, reason: "USB4 (some models)", severity: Severity::Low },

            // === Input ===
            KernelRequirement { config: "CONFIG_HID", expected: ConfigValue::Y, reason: "HID core", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_HID_GENERIC", expected: ConfigValue::Y, reason: "Generic HID driver", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_HID", expected: ConfigValue::Y, reason: "USB HID support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_INPUT_EVDEV", expected: ConfigValue::Y, reason: "Event device interface", severity: Severity::Critical },

            // === Power Management ===
            KernelRequirement { config: "CONFIG_ACPI", expected: ConfigValue::Y, reason: "ACPI support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_CPU_FREQ", expected: ConfigValue::Y, reason: "CPU frequency scaling", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_AMD_PSTATE", expected: ConfigValue::Y, reason: "AMD P-state driver", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_AMD_PMC", expected: ConfigValue::M, reason: "AMD Power Management Controller", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SUSPEND", expected: ConfigValue::Y, reason: "Suspend to RAM", severity: Severity::High },

            // === Sensors/Monitoring ===
            KernelRequirement { config: "CONFIG_HWMON", expected: ConfigValue::Y, reason: "Hardware monitoring", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_K10TEMP", expected: ConfigValue::M, reason: "AMD CPU temperature (includes Zen support)", severity: Severity::High },
            // Note: zenpower is out-of-tree; k10temp in mainline now supports Zen CPUs
            KernelRequirement { config: "CONFIG_SENSORS_NCT6775", expected: ConfigValue::M, reason: "Nuvoton Super I/O sensors", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SENSORS_IT87", expected: ConfigValue::M, reason: "ITE Super I/O sensors", severity: Severity::Medium },

            // === TPM ===
            KernelRequirement { config: "CONFIG_TCG_TPM", expected: ConfigValue::Y, reason: "TPM 2.0 support", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_TCG_CRB", expected: ConfigValue::Y, reason: "TPM CRB interface", severity: Severity::Medium },

            // === SD Card Reader (some models) ===
            KernelRequirement { config: "CONFIG_MMC", expected: ConfigValue::Y, reason: "MMC/SD card support", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_MMC_SDHCI", expected: ConfigValue::Y, reason: "SDHCI controller", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_MMC_SDHCI_PCI", expected: ConfigValue::Y, reason: "SDHCI PCI", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_MMC_REALTEK_PCI", expected: ConfigValue::M, reason: "Realtek SD reader", severity: Severity::Low },

            // === Virtualization ===
            KernelRequirement { config: "CONFIG_KVM", expected: ConfigValue::M, reason: "KVM virtualization", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_KVM_AMD", expected: ConfigValue::M, reason: "KVM for AMD (SVM)", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_AMD_MEM_ENCRYPT", expected: ConfigValue::Y, reason: "AMD SME/SEV memory encryption", severity: Severity::Low },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // AMD GPU (comprehensive - Vega, RDNA1/2/3)
            FirmwareRequirement { pattern: "amdgpu/*", min_files: 600, reason: "AMD GPU firmware (Vega/RDNA)", severity: Severity::Critical },

            // AMD PSP (Platform Security Processor)
            FirmwareRequirement { pattern: "amdgpu/psp_*", min_files: 20, reason: "AMD PSP firmware", severity: Severity::Critical },

            // CPU Microcode
            FirmwareRequirement { pattern: "amd-ucode/*", min_files: 1, reason: "AMD CPU microcode", severity: Severity::Critical },

            // Intel WiFi (common option)
            FirmwareRequirement { pattern: "iwlwifi-*", min_files: 100, reason: "Intel WiFi firmware", severity: Severity::High },

            // MediaTek WiFi (common in AMD mini PCs)
            FirmwareRequirement { pattern: "mediatek/WIFI_*", min_files: 5, reason: "MediaTek WiFi firmware", severity: Severity::Critical },

            // Realtek WiFi
            FirmwareRequirement { pattern: "rtw88/*", min_files: 10, reason: "Realtek WiFi 5 firmware", severity: Severity::High },
            FirmwareRequirement { pattern: "rtw89/*", min_files: 10, reason: "Realtek WiFi 6 firmware", severity: Severity::High },

            // Intel Bluetooth (if Intel WiFi used)
            FirmwareRequirement { pattern: "intel/ibt-*", min_files: 50, reason: "Intel Bluetooth firmware", severity: Severity::High },

            // MediaTek Bluetooth
            FirmwareRequirement { pattern: "mediatek/BT_*", min_files: 1, reason: "MediaTek Bluetooth firmware", severity: Severity::High },

            // Realtek Bluetooth
            FirmwareRequirement { pattern: "rtl_bt/*", min_files: 20, reason: "Realtek Bluetooth firmware", severity: Severity::High },
        ]
    }
}
