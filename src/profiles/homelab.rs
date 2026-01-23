use crate::profiles::HardwareProfile;
use crate::kernel::{KernelRequirement, ConfigValue};
use crate::firmware::FirmwareRequirement;
use crate::report::Severity;

/// Homelab/Server Profile (2018-2026)
///
/// Covers: Dell PowerEdge, HP ProLiant, Supermicro, custom servers
///         Intel Xeon E/W, AMD EPYC, Ryzen Server builds
/// Features: IPMI/iLO/iDRAC, RAID, 10GbE, ECC, multiple NICs
pub struct HomelabProfile;

impl HardwareProfile for HomelabProfile {
    fn name(&self) -> &str { "Homelab/Server" }
    fn description(&self) -> &str { "Servers and homelab - PowerEdge, ProLiant, EPYC, Xeon" }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &[
            // === GPU (minimal, headless) ===
            KernelRequirement { config: "CONFIG_DRM", expected: ConfigValue::Y, reason: "DRM core", severity: Severity::High },
            KernelRequirement { config: "CONFIG_DRM_SIMPLEDRM", expected: ConfigValue::M, reason: "Simple DRM (UEFI framebuffer)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_FB_EFI", expected: ConfigValue::Y, reason: "EFI framebuffer", severity: Severity::High },
            KernelRequirement { config: "CONFIG_VGA_CONSOLE", expected: ConfigValue::Y, reason: "VGA console", severity: Severity::Medium },
            // Aspeed AST (common BMC VGA)
            KernelRequirement { config: "CONFIG_DRM_AST", expected: ConfigValue::M, reason: "Aspeed AST (BMC VGA)", severity: Severity::High },
            // Matrox (older servers)
            KernelRequirement { config: "CONFIG_DRM_MGAG200", expected: ConfigValue::M, reason: "Matrox G200 (BMC)", severity: Severity::Medium },

            // === Storage - Enterprise ===
            KernelRequirement { config: "CONFIG_NVME_CORE", expected: ConfigValue::Y, reason: "NVMe SSD", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_BLK_DEV_NVME", expected: ConfigValue::Y, reason: "NVMe block device", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_NVME_MULTIPATH", expected: ConfigValue::Y, reason: "NVMe multipath", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SATA_AHCI", expected: ConfigValue::Y, reason: "SATA AHCI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_ATA", expected: ConfigValue::Y, reason: "ATA subsystem", severity: Severity::Critical },
            // Hardware RAID controllers
            KernelRequirement { config: "CONFIG_MEGARAID_SAS", expected: ConfigValue::M, reason: "Broadcom MegaRAID SAS", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MEGARAID_MM", expected: ConfigValue::M, reason: "MegaRAID management", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_SCSI_HPSA", expected: ConfigValue::M, reason: "HP Smart Array", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SCSI_MPT3SAS", expected: ConfigValue::M, reason: "Broadcom SAS 3.0", severity: Severity::High },
            // Software RAID
            KernelRequirement { config: "CONFIG_MD", expected: ConfigValue::Y, reason: "MD RAID", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BLK_DEV_MD", expected: ConfigValue::Y, reason: "MD block device", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MD_RAID0", expected: ConfigValue::M, reason: "RAID-0", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MD_RAID1", expected: ConfigValue::M, reason: "RAID-1", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MD_RAID456", expected: ConfigValue::M, reason: "RAID-5/6", severity: Severity::High },
            // SCSI for SAS drives
            KernelRequirement { config: "CONFIG_SCSI", expected: ConfigValue::Y, reason: "SCSI subsystem", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BLK_DEV_SD", expected: ConfigValue::Y, reason: "SCSI disk", severity: Severity::High },

            // === Ethernet - Server Grade ===
            // Intel GbE
            KernelRequirement { config: "CONFIG_E1000E", expected: ConfigValue::Y, reason: "Intel I217/I218/I219", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IGB", expected: ConfigValue::Y, reason: "Intel I350/I210/I211", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_IGC", expected: ConfigValue::Y, reason: "Intel I225/I226 2.5GbE", severity: Severity::High },
            // Intel 10/25/40GbE
            KernelRequirement { config: "CONFIG_IXGBE", expected: ConfigValue::M, reason: "Intel 10GbE (X520/X540/X550)", severity: Severity::High },
            KernelRequirement { config: "CONFIG_I40E", expected: ConfigValue::M, reason: "Intel XL710 10/25/40GbE", severity: Severity::High },
            KernelRequirement { config: "CONFIG_ICE", expected: ConfigValue::M, reason: "Intel E810 25/100GbE", severity: Severity::Medium },
            // Mellanox ConnectX
            KernelRequirement { config: "CONFIG_MLX4_CORE", expected: ConfigValue::M, reason: "Mellanox ConnectX-3", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MLX4_EN", expected: ConfigValue::M, reason: "Mellanox ConnectX-3 Ethernet", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MLX5_CORE", expected: ConfigValue::M, reason: "Mellanox ConnectX-4/5/6/7", severity: Severity::High },
            // Broadcom
            KernelRequirement { config: "CONFIG_BNXT", expected: ConfigValue::M, reason: "Broadcom NetXtreme-E", severity: Severity::Medium },
            KernelRequirement { config: "CONFIG_BNX2X", expected: ConfigValue::M, reason: "Broadcom NetXtreme II 10GbE", severity: Severity::Medium },
            // Chelsio
            KernelRequirement { config: "CONFIG_CHELSIO_T4", expected: ConfigValue::M, reason: "Chelsio T4/T5/T6", severity: Severity::Medium },

            // === USB (minimal for servers) ===
            KernelRequirement { config: "CONFIG_USB_XHCI_HCD", expected: ConfigValue::Y, reason: "USB 3.x", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_XHCI_PCI", expected: ConfigValue::Y, reason: "xHCI PCI", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_EHCI_HCD", expected: ConfigValue::Y, reason: "USB 2.0", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_STORAGE", expected: ConfigValue::M, reason: "USB storage", severity: Severity::High },

            // === Input (minimal) ===
            KernelRequirement { config: "CONFIG_HID", expected: ConfigValue::Y, reason: "HID core", severity: Severity::High },
            KernelRequirement { config: "CONFIG_USB_HID", expected: ConfigValue::Y, reason: "USB HID", severity: Severity::High },
            KernelRequirement { config: "CONFIG_INPUT_EVDEV", expected: ConfigValue::Y, reason: "Event device", severity: Severity::High },

            // === IPMI/BMC ===
            KernelRequirement { config: "CONFIG_IPMI_HANDLER", expected: ConfigValue::M, reason: "IPMI message handler", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_IPMI_DEVICE_INTERFACE", expected: ConfigValue::M, reason: "IPMI device interface", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_IPMI_SI", expected: ConfigValue::M, reason: "IPMI system interface", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_IPMI_SSIF", expected: ConfigValue::M, reason: "IPMI SSIF", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IPMI_WATCHDOG", expected: ConfigValue::M, reason: "IPMI watchdog", severity: Severity::High },
            KernelRequirement { config: "CONFIG_IPMI_POWEROFF", expected: ConfigValue::M, reason: "IPMI poweroff", severity: Severity::Medium },

            // === Serial Console (critical for headless) ===
            KernelRequirement { config: "CONFIG_SERIAL_8250", expected: ConfigValue::Y, reason: "8250/16550 serial", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SERIAL_8250_CONSOLE", expected: ConfigValue::Y, reason: "Serial console", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_SERIAL_8250_PCI", expected: ConfigValue::Y, reason: "Serial PCI", severity: Severity::High },

            // === Power Management ===
            KernelRequirement { config: "CONFIG_ACPI", expected: ConfigValue::Y, reason: "ACPI", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_CPU_FREQ", expected: ConfigValue::Y, reason: "CPU frequency", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_INTEL_PSTATE", expected: ConfigValue::Y, reason: "Intel P-state", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_AMD_PSTATE", expected: ConfigValue::Y, reason: "AMD P-state", severity: Severity::High },
            KernelRequirement { config: "CONFIG_ACPI_CPPC_LIB", expected: ConfigValue::Y, reason: "CPPC (EPYC)", severity: Severity::Medium },

            // === ECC Memory ===
            KernelRequirement { config: "CONFIG_EDAC", expected: ConfigValue::Y, reason: "ECC error detection", severity: Severity::Critical },
            KernelRequirement { config: "CONFIG_EDAC_DECODE_MCE", expected: ConfigValue::M, reason: "MCE decoding", severity: Severity::High },

            // === Sensors ===
            KernelRequirement { config: "CONFIG_HWMON", expected: ConfigValue::Y, reason: "Hardware monitoring", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_CORETEMP", expected: ConfigValue::M, reason: "Intel CPU temp", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_K10TEMP", expected: ConfigValue::M, reason: "AMD CPU temp", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SENSORS_NCT6775", expected: ConfigValue::M, reason: "Nuvoton sensors", severity: Severity::Medium },

            // === Watchdog ===
            KernelRequirement { config: "CONFIG_WATCHDOG", expected: ConfigValue::Y, reason: "Watchdog support", severity: Severity::High },
            KernelRequirement { config: "CONFIG_WATCHDOG_CORE", expected: ConfigValue::Y, reason: "Watchdog core", severity: Severity::High },
            KernelRequirement { config: "CONFIG_ITCO_WDT", expected: ConfigValue::M, reason: "Intel TCO watchdog", severity: Severity::High },
            KernelRequirement { config: "CONFIG_SP5100_TCO", expected: ConfigValue::M, reason: "AMD SP5100 watchdog", severity: Severity::High },

            // === TPM ===
            KernelRequirement { config: "CONFIG_TCG_TPM", expected: ConfigValue::Y, reason: "TPM 2.0", severity: Severity::High },
            KernelRequirement { config: "CONFIG_TCG_TIS", expected: ConfigValue::Y, reason: "TPM TIS", severity: Severity::High },
            KernelRequirement { config: "CONFIG_TCG_CRB", expected: ConfigValue::Y, reason: "TPM CRB", severity: Severity::High },

            // === Virtualization ===
            KernelRequirement { config: "CONFIG_KVM", expected: ConfigValue::M, reason: "KVM", severity: Severity::High },
            KernelRequirement { config: "CONFIG_KVM_INTEL", expected: ConfigValue::M, reason: "KVM Intel", severity: Severity::High },
            KernelRequirement { config: "CONFIG_KVM_AMD", expected: ConfigValue::M, reason: "KVM AMD", severity: Severity::High },
            KernelRequirement { config: "CONFIG_VFIO", expected: ConfigValue::M, reason: "VFIO", severity: Severity::High },
            KernelRequirement { config: "CONFIG_VFIO_PCI", expected: ConfigValue::M, reason: "VFIO PCI", severity: Severity::High },
            KernelRequirement { config: "CONFIG_VHOST_NET", expected: ConfigValue::M, reason: "vhost-net", severity: Severity::Medium },

            // === NUMA ===
            KernelRequirement { config: "CONFIG_NUMA", expected: ConfigValue::Y, reason: "NUMA support", severity: Severity::High },
            KernelRequirement { config: "CONFIG_X86_64_ACPI_NUMA", expected: ConfigValue::Y, reason: "ACPI NUMA", severity: Severity::High },
            KernelRequirement { config: "CONFIG_NUMA_BALANCING", expected: ConfigValue::Y, reason: "NUMA balancing", severity: Severity::Medium },

            // === Cgroups (containers) ===
            KernelRequirement { config: "CONFIG_CGROUPS", expected: ConfigValue::Y, reason: "Control groups", severity: Severity::High },
            KernelRequirement { config: "CONFIG_CGROUP_SCHED", expected: ConfigValue::Y, reason: "CPU cgroup", severity: Severity::High },
            KernelRequirement { config: "CONFIG_MEMCG", expected: ConfigValue::Y, reason: "Memory cgroup", severity: Severity::High },
            KernelRequirement { config: "CONFIG_BLK_CGROUP", expected: ConfigValue::Y, reason: "Block I/O cgroup", severity: Severity::High },
        ]
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &[
            // Intel NIC firmware
            FirmwareRequirement { pattern: "intel/**/*", min_files: 200, reason: "Intel NIC firmware", severity: Severity::High },

            // CPU Microcode
            FirmwareRequirement { pattern: "intel-ucode/*", min_files: 100, reason: "Intel Xeon microcode", severity: Severity::Critical },
            FirmwareRequirement { pattern: "amd-ucode/*", min_files: 1, reason: "AMD EPYC microcode", severity: Severity::Critical },

            // Broadcom NIC
            FirmwareRequirement { pattern: "bnx2x/*", min_files: 5, reason: "Broadcom NetXtreme II", severity: Severity::Medium },
        ]
    }
}
