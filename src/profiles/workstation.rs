use crate::firmware::FirmwareRequirement;
use crate::kernel::{ConfigValue, KernelRequirement};
use crate::profiles::HardwareProfile;
use crate::report::Severity;

/// Workstation Profile (2018-2026)
///
/// Covers: Dell Precision, HP ZBook, Lenovo ThinkStation/ThinkPad P-series
///         AMD Threadripper, Intel Xeon W workstations
/// Features: ECC RAM, multiple GPUs, 10GbE, IPMI, NVMe RAID
pub struct WorkstationProfile;

impl HardwareProfile for WorkstationProfile {
    fn name(&self) -> &str {
        "Workstation (Precision, ZBook, Threadripper)"
    }
    fn description(&self) -> &str {
        "Professional workstations - Xeon, Threadripper, Quadro/Pro GPUs"
    }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU - Professional Graphics ===
            // NVIDIA Quadro/RTX Pro
            KernelRequirement {
                config: "CONFIG_DRM_NOUVEAU",
                expected: ConfigValue::M,
                reason: "NVIDIA Quadro/RTX (nouveau)",
                severity: Severity::Critical,
            },
            // AMD Radeon Pro
            KernelRequirement {
                config: "CONFIG_DRM_AMDGPU",
                expected: ConfigValue::M,
                reason: "AMD Radeon Pro",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_DRM_AMD_DC",
                expected: ConfigValue::Y,
                reason: "AMD Display Core",
                severity: Severity::Critical,
            },
            // Intel integrated
            KernelRequirement {
                config: "CONFIG_DRM_I915",
                expected: ConfigValue::M,
                reason: "Intel integrated",
                severity: Severity::High,
            },
            // Multi-GPU support
            KernelRequirement {
                config: "CONFIG_DRM_KMS_HELPER",
                expected: ConfigValue::Y,
                reason: "KMS helper",
                severity: Severity::Critical,
            },
            KernelRequirement {
                config: "CONFIG_DRM_FBDEV_EMULATION",
                expected: ConfigValue::Y,
                reason: "Framebuffer emulation",
                severity: Severity::High,
            },
            // === Storage - Enterprise NVMe, RAID ===
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
                config: "CONFIG_NVME_MULTIPATH",
                expected: ConfigValue::Y,
                reason: "NVMe multipath",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_SATA_AHCI",
                expected: ConfigValue::Y,
                reason: "SATA AHCI",
                severity: Severity::High,
            },
            // Hardware RAID
            KernelRequirement {
                config: "CONFIG_MEGARAID_SAS",
                expected: ConfigValue::M,
                reason: "LSI MegaRAID SAS",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_SCSI_HPSA",
                expected: ConfigValue::M,
                reason: "HP Smart Array",
                severity: Severity::Medium,
            },
            // Software RAID
            KernelRequirement {
                config: "CONFIG_MD",
                expected: ConfigValue::Y,
                reason: "Software RAID (md)",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_BLK_DEV_MD",
                expected: ConfigValue::Y,
                reason: "MD block device",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_MD_RAID0",
                expected: ConfigValue::M,
                reason: "RAID-0",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_MD_RAID1",
                expected: ConfigValue::M,
                reason: "RAID-1",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_MD_RAID456",
                expected: ConfigValue::M,
                reason: "RAID-5/6",
                severity: Severity::High,
            },
            // === Ethernet - 10GbE ===
            KernelRequirement {
                config: "CONFIG_R8169",
                expected: ConfigValue::Y,
                reason: "Realtek GbE",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_E1000E",
                expected: ConfigValue::Y,
                reason: "Intel GbE",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_IGB",
                expected: ConfigValue::Y,
                reason: "Intel I350/I210",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_IGC",
                expected: ConfigValue::Y,
                reason: "Intel I225/I226 2.5GbE",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_IXGBE",
                expected: ConfigValue::M,
                reason: "Intel 10GbE",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_I40E",
                expected: ConfigValue::M,
                reason: "Intel XL710 10/40GbE",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_MLX4_EN",
                expected: ConfigValue::M,
                reason: "Mellanox ConnectX-3",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_MLX5_CORE",
                expected: ConfigValue::M,
                reason: "Mellanox ConnectX-4/5/6",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_AQTION",
                expected: ConfigValue::M,
                reason: "Aquantia 10GbE",
                severity: Severity::Medium,
            },
            // === WiFi (optional) ===
            KernelRequirement {
                config: "CONFIG_MAC80211",
                expected: ConfigValue::M,
                reason: "802.11 stack",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_CFG80211",
                expected: ConfigValue::M,
                reason: "Wireless config",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_IWLWIFI",
                expected: ConfigValue::M,
                reason: "Intel WiFi",
                severity: Severity::Medium,
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
                config: "CONFIG_USB_STORAGE",
                expected: ConfigValue::M,
                reason: "USB storage",
                severity: Severity::High,
            },
            // === Thunderbolt ===
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
            // === Audio ===
            KernelRequirement {
                config: "CONFIG_SND_HDA_INTEL",
                expected: ConfigValue::Y,
                reason: "HD Audio",
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
                config: "CONFIG_INPUT_EVDEV",
                expected: ConfigValue::Y,
                reason: "Event device",
                severity: Severity::Critical,
            },
            // === IPMI/BMC ===
            KernelRequirement {
                config: "CONFIG_IPMI_HANDLER",
                expected: ConfigValue::M,
                reason: "IPMI message handler",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_IPMI_DEVICE_INTERFACE",
                expected: ConfigValue::M,
                reason: "IPMI device interface",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_IPMI_SI",
                expected: ConfigValue::M,
                reason: "IPMI system interface",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_IPMI_SSIF",
                expected: ConfigValue::M,
                reason: "IPMI SSIF interface",
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
            // === ECC Memory ===
            KernelRequirement {
                config: "CONFIG_EDAC",
                expected: ConfigValue::Y,
                reason: "Error Detection And Correction",
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
            KernelRequirement {
                config: "CONFIG_SENSORS_NCT6775",
                expected: ConfigValue::M,
                reason: "Nuvoton sensors",
                severity: Severity::Medium,
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
            // === Virtualization ===
            KernelRequirement {
                config: "CONFIG_KVM",
                expected: ConfigValue::M,
                reason: "KVM",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_KVM_INTEL",
                expected: ConfigValue::M,
                reason: "KVM Intel",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_KVM_AMD",
                expected: ConfigValue::M,
                reason: "KVM AMD",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_VFIO",
                expected: ConfigValue::M,
                reason: "VFIO (GPU passthrough)",
                severity: Severity::Medium,
            },
            KernelRequirement {
                config: "CONFIG_VFIO_PCI",
                expected: ConfigValue::M,
                reason: "VFIO PCI",
                severity: Severity::Medium,
            },
            // === NUMA ===
            KernelRequirement {
                config: "CONFIG_NUMA",
                expected: ConfigValue::Y,
                reason: "NUMA support",
                severity: Severity::High,
            },
            KernelRequirement {
                config: "CONFIG_X86_64_ACPI_NUMA",
                expected: ConfigValue::Y,
                reason: "ACPI NUMA",
                severity: Severity::High,
            },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
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
            // Intel GPU
            FirmwareRequirement {
                pattern: "i915/*",
                min_files: 100,
                reason: "Intel GPU firmware",
                severity: Severity::High,
            },
            // Network
            FirmwareRequirement {
                pattern: "intel/**/*",
                min_files: 200,
                reason: "Intel NIC/WiFi firmware",
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
