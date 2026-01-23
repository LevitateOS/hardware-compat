use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

/// Generic Desktop Profile (2018-2026)
///
/// Covers: Custom builds, pre-built desktops, workstations
/// CPU: Intel 8th-14th gen, AMD Ryzen 1000-7000
/// GPU: AMD Radeon, NVIDIA GeForce, Intel Arc
/// Most permissive profile - covers widest hardware range
pub struct DesktopProfile;

impl HardwareProfile for DesktopProfile {
    fn name(&self) -> &str { "Desktop (Custom/Pre-built)" }
    fn description(&self) -> &str { "Desktops and workstations - Intel/AMD CPUs, all GPU vendors" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU - All Vendors ===
            // Intel (Arc, integrated)
            KernelRequirement { config: "CONFIG_DRM_I915", expected: ConfigValue::M, reason: "Intel graphics (Arc, UHD, Iris)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_XE", expected: ConfigValue::M, reason: "Intel Xe graphics (Arc discrete)", severity: Severity::Medium },
            // AMD
            KernelRequirement { config: "CONFIG_DRM_AMDGPU", expected: ConfigValue::M, reason: "AMD Radeon (RX 5000/6000/7000, Vega)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_AMDGPU_SI", expected: ConfigValue::Y, reason: "AMD Southern Islands (legacy)", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_DRM_AMDGPU_CIK", expected: ConfigValue::Y, reason: "AMD Sea Islands (legacy)", severity: Severity::Low },
            // NVIDIA (nouveau - open source)
            KernelRequirement { config: "CONFIG_DRM_NOUVEAU", expected: ConfigValue::M, reason: "NVIDIA nouveau (GTX 10xx/16xx, RTX 20xx basic)", severity: Severity::Critical },
            // Core DRM
            KernelRequirement { config: "CONFIG_DRM_KMS_HELPER", expected: ConfigValue::Y, reason: "Kernel mode setting", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_DRM_FBDEV_EMULATION", expected: ConfigValue::Y, reason: "Framebuffer emulation", severity: Severity::High },
            KernelRequirement { config: "CONFIG_FB", expected: ConfigValue::Y, reason: "Framebuffer support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_FB_EFI", expected: ConfigValue::Y, reason: "EFI framebuffer", severity: Severity::High },

            // === Storage ===
            KernelRequirement { config: "CONFIG_NVME_CORE", expected: ConfigValue::Y, reason: "NVMe SSD", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BLK_DEV_NVME", expected: ConfigValue::Y, reason: "NVMe block device", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SATA_AHCI", expected: ConfigValue::Y, reason: "SATA AHCI (HDD/SSD)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ATA", expected: ConfigValue::Y, reason: "ATA subsystem", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ATA_PIIX", expected: ConfigValue::Y, reason: "Intel PIIX/ICH SATA", severity: Severity::High },
            KernelRequirement { config: "CONFIG_PATA_AMD", expected: ConfigValue::M, reason: "AMD PATA (legacy)", severity: Severity::Low },

            // === Ethernet ===
            // Intel
            KernelRequirement { config: "CONFIG_E1000E", expected: ConfigValue::Y, reason: "Intel I217/I218/I219 GbE", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IGC", expected: ConfigValue::Y, reason: "Intel I225/I226 2.5GbE", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IGB", expected: ConfigValue::Y, reason: "Intel I350/I210/I211 GbE", severity: Severity::High },
            // Realtek (very common on motherboards)
            KernelRequirement { config: "CONFIG_R8169", expected: ConfigValue::Y, reason: "Realtek RTL8111/8168/8125 (1/2.5 GbE)", severity: Severity::Critical },
            // Aquantia/Marvell (10GbE)
            KernelRequirement { config: "CONFIG_AQTION", expected: ConfigValue::M, reason: "Aquantia AQC107/113 10GbE", severity: Severity::Medium },
            // USB Ethernet
            KernelRequirement { config: "CONFIG_USB_RTL8152", expected: ConfigValue::M, reason: "Realtek USB Ethernet", severity: Severity::High },

            // === WiFi (PCIe cards, USB dongles) ===
            KernelRequirement { config: "CONFIG_MAC80211", expected: ConfigValue::M, reason: "802.11 wireless stack", severity: Severity::High },
            KernelRequirement { config: "CONFIG_CFG80211", expected: ConfigValue::M, reason: "Wireless configuration API", severity: Severity::High },
            // Intel
            KernelRequirement { config: "CONFIG_IWLWIFI", expected: ConfigValue::M, reason: "Intel WiFi (AX200/AX210)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IWLMVM", expected: ConfigValue::M, reason: "Intel WiFi MVM", severity: Severity::High },
            // Realtek
            KernelRequirement { config: "CONFIG_RTW88", expected: ConfigValue::M, reason: "Realtek RTL8822/8821 WiFi", severity: Severity::High },
            KernelRequirement { config: "CONFIG_RTW89", expected: ConfigValue::M, reason: "Realtek RTL8852 WiFi 6", severity: Severity::High },
            // MediaTek
            KernelRequirement { config: "CONFIG_MT7921E", expected: ConfigValue::M, reason: "MediaTek MT7921/7922 WiFi 6E", severity: Severity::High },
            // Atheros/Qualcomm
            KernelRequirement { config: "CONFIG_ATH10K", expected: ConfigValue::M, reason: "Qualcomm Atheros WiFi 5", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_ATH11K", expected: ConfigValue::M, reason: "Qualcomm WiFi 6", severity: Severity::Medium },

            // === Bluetooth ===
            KernelRequirement { config: "CONFIG_BT", expected: ConfigValue::M, reason: "Bluetooth subsystem", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_HCIBTUSB", expected: ConfigValue::M, reason: "Bluetooth HCI USB", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_INTEL", expected: ConfigValue::M, reason: "Intel Bluetooth", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_RTL", expected: ConfigValue::M, reason: "Realtek Bluetooth", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BT_MTK", expected: ConfigValue::M, reason: "MediaTek Bluetooth", severity: Severity::Medium },

            // === Audio ===
            KernelRequirement { config: "CONFIG_SND_HDA_INTEL", expected: ConfigValue::Y, reason: "Intel HD Audio", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_REALTEK", expected: ConfigValue::Y, reason: "Realtek codec (most common)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_HDMI", expected: ConfigValue::Y, reason: "HDMI/DP audio", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_ANALOG", expected: ConfigValue::Y, reason: "Analog Devices codec", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SND_HDA_CODEC_CONEXANT", expected: ConfigValue::Y, reason: "Conexant codec", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_SND_USB_AUDIO", expected: ConfigValue::M, reason: "USB audio (DACs, headsets)", severity: Severity::High },

            // === USB ===
            KernelRequirement { config: "CONFIG_USB_XHCI_HCD", expected: ConfigValue::Y, reason: "USB 3.x (xHCI)", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_XHCI_PCI", expected: ConfigValue::Y, reason: "xHCI PCI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_EHCI_HCD", expected: ConfigValue::Y, reason: "USB 2.0 (EHCI)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_OHCI_HCD", expected: ConfigValue::Y, reason: "USB 1.1 (OHCI)", severity: Severity::Low },
            KernelRequirement { config: "CONFIG_USB_STORAGE", expected: ConfigValue::M, reason: "USB mass storage", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_UAS", expected: ConfigValue::M, reason: "USB Attached SCSI", severity: Severity::High },

            // === Input ===
            KernelRequirement { config: "CONFIG_HID", expected: ConfigValue::Y, reason: "HID core", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_HID_GENERIC", expected: ConfigValue::Y, reason: "Generic HID driver", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_USB_HID", expected: ConfigValue::Y, reason: "USB HID", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_INPUT_EVDEV", expected: ConfigValue::Y, reason: "Event device interface", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_INPUT_KEYBOARD", expected: ConfigValue::Y, reason: "Keyboard support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_INPUT_MOUSE", expected: ConfigValue::Y, reason: "Mouse support", severity: Severity::Critical },

            // === Power Management ===
            KernelRequirement { config: "CONFIG_ACPI", expected: ConfigValue::Y, reason: "ACPI support", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_CPU_FREQ", expected: ConfigValue::Y, reason: "CPU frequency scaling", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_INTEL_PSTATE", expected: ConfigValue::Y, reason: "Intel P-state", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_AMD_PSTATE", expected: ConfigValue::Y, reason: "AMD P-state", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SUSPEND", expected: ConfigValue::Y, reason: "Suspend to RAM", severity: Severity::Medium },

            // === Sensors/Monitoring ===
            KernelRequirement { config: "CONFIG_HWMON", expected: ConfigValue::Y, reason: "Hardware monitoring", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_CORETEMP", expected: ConfigValue::M, reason: "Intel CPU temperature", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_K10TEMP", expected: ConfigValue::M, reason: "AMD CPU temperature", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_IT87", expected: ConfigValue::M, reason: "ITE Super I/O sensors", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SENSORS_NCT6775", expected: ConfigValue::M, reason: "Nuvoton NCT6775/NCT6776 sensors", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SENSORS_ASUS_EC", expected: ConfigValue::M, reason: "ASUS embedded controller sensors", severity: Severity::Low },

            // === TPM ===
            KernelRequirement { config: "CONFIG_TCG_TPM", expected: ConfigValue::Y, reason: "TPM 2.0 support", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_TCG_TIS", expected: ConfigValue::Y, reason: "TPM TIS interface", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_TCG_CRB", expected: ConfigValue::Y, reason: "TPM CRB interface", severity: Severity::Medium },

            // === SD Card Reader ===
            KernelRequirement { config: "CONFIG_MMC", expected: ConfigValue::Y, reason: "MMC/SD card support", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_MMC_SDHCI", expected: ConfigValue::Y, reason: "SDHCI controller", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_MMC_SDHCI_PCI", expected: ConfigValue::Y, reason: "SDHCI PCI", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_MMC_REALTEK_PCI", expected: ConfigValue::M, reason: "Realtek PCI-E SD reader", severity: Severity::Medium },

            // === Virtualization (KVM) ===
            KernelRequirement { config: "CONFIG_KVM", expected: ConfigValue::M, reason: "KVM virtualization", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_KVM_INTEL", expected: ConfigValue::M, reason: "KVM for Intel", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_KVM_AMD", expected: ConfigValue::M, reason: "KVM for AMD", severity: Severity::Medium },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // Intel GPU
            FirmwareRequirement { pattern: "i915/*", min_files: 100, reason: "Intel GPU firmware", severity: Severity::Critical },

            // AMD GPU (comprehensive - RDNA1/2/3, Vega, Polaris)
            FirmwareRequirement { pattern: "amdgpu/*", min_files: 600, reason: "AMD GPU firmware", severity: Severity::Critical },

            // NVIDIA (nouveau)
            FirmwareRequirement { pattern: "nvidia/*", min_files: 30, reason: "NVIDIA GPU firmware (nouveau)", severity: Severity::Critical },

            // Intel WiFi
            FirmwareRequirement { pattern: "iwlwifi-*", min_files: 100, reason: "Intel WiFi firmware", severity: Severity::High },

            // Realtek WiFi
            FirmwareRequirement { pattern: "rtw88/*", min_files: 10, reason: "Realtek WiFi 5 firmware", severity: Severity::High },
            FirmwareRequirement { pattern: "rtw89/*", min_files: 10, reason: "Realtek WiFi 6 firmware", severity: Severity::High },

            // MediaTek WiFi
            FirmwareRequirement { pattern: "mediatek/*", min_files: 10, reason: "MediaTek WiFi firmware", severity: Severity::High },

            // Atheros/Qualcomm
            FirmwareRequirement { pattern: "ath10k/*", min_files: 20, reason: "Qualcomm WiFi 5 firmware", severity: Severity::Medium },
            FirmwareRequirement { pattern: "ath11k/*", min_files: 10, reason: "Qualcomm WiFi 6 firmware", severity: Severity::Medium },

            // Bluetooth
            FirmwareRequirement { pattern: "intel/ibt-*", min_files: 50, reason: "Intel Bluetooth firmware", severity: Severity::High },
            FirmwareRequirement { pattern: "rtl_bt/*", min_files: 20, reason: "Realtek Bluetooth firmware", severity: Severity::High },

            // CPU Microcode
            FirmwareRequirement { pattern: "intel-ucode/*", min_files: 100, reason: "Intel CPU microcode", severity: Severity::Critical },
            FirmwareRequirement { pattern: "amd-ucode/*", min_files: 1, reason: "AMD CPU microcode", severity: Severity::Critical },
        ]
    }
}
