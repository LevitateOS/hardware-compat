# Hardware Compatibility Verification - Implementation Plan

## Summary

Static verification test that checks if the LevitateOS build has all required kernel drivers and firmware for modern hardware (2018-2026 desktops, laptops, NUCs, mini PCs).

**Key insight:** This is NOT a runtime hardware test. It's a BUILD verification test that runs in CI to ensure the ISO will work on real hardware before users try it.

---

## Target Hardware Matrix

### Form Factors
- Intel NUCs (7th-13th gen)
- Mini PCs (Beelink, Minisforum, GMKtec)
- Laptops (ThinkPad, Dell XPS, HP, Framework, ASUS)
- Desktops (custom builds, pre-builts)
- Workstations

### CPUs (x86_64-v3+, 2015 onwards)

| Vendor | Families | Microcode Required |
|--------|----------|-------------------|
| Intel | 8th-14th gen (Coffee Lake → Raptor Lake) | intel-ucode/* |
| AMD | Ryzen 1000-7000 (Zen → Zen4) | amd-ucode/* |

### GPUs

| Type | Driver | Firmware Dir |
|------|--------|--------------|
| Intel integrated (UHD 620/630, Iris Xe, Arc) | i915 | i915/* |
| AMD integrated (Vega, RDNA2/3) | amdgpu | amdgpu/* |
| AMD discrete (RX 5xx, 5000, 6000, 7000) | amdgpu | amdgpu/* |
| NVIDIA discrete (GTX 10xx, RTX 20xx/30xx/40xx) | nouveau | nvidia/* |

### Storage

| Type | Driver | Config |
|------|--------|--------|
| NVMe SSD | nvme | CONFIG_NVME_CORE, CONFIG_BLK_DEV_NVME |
| SATA SSD/HDD | ahci | CONFIG_SATA_AHCI |
| USB storage | usb-storage | CONFIG_USB_STORAGE |
| eMMC | mmc | CONFIG_MMC |

### Network - Ethernet

| Chip | Driver | Config |
|------|--------|--------|
| Intel I219 (NUC, laptops) | e1000e | CONFIG_E1000E |
| Intel I225/I226 (2.5GbE) | igc | CONFIG_IGC |
| Realtek RTL8111/8168 | r8169 | CONFIG_R8169 |
| Realtek RTL8125 (2.5GbE) | r8169 | CONFIG_R8169 |
| Realtek USB (RTL8152) | r8152 | CONFIG_USB_RTL8152 |

### Network - WiFi

| Chip | Driver | Firmware |
|------|--------|----------|
| Intel AX200/201/210/211/BE200 | iwlwifi | iwlwifi-* |
| Realtek RTL8822/8852 | rtw88/rtw89 | rtw88/*, rtw89/* |
| MediaTek MT7921/7922 | mt76 | mediatek/* |
| Atheros/Qualcomm | ath10k/11k/12k | ath10k/*, ath11k/* |
| Broadcom (Mac, some laptops) | brcmfmac | brcm/* |

### Audio

| Type | Driver | Config |
|------|--------|--------|
| Intel HDA | snd_hda_intel | CONFIG_SND_HDA_INTEL |
| Realtek codecs | snd_hda_codec_realtek | CONFIG_SND_HDA_CODEC_REALTEK |
| HDMI audio | snd_hda_codec_hdmi | CONFIG_SND_HDA_CODEC_HDMI |

### USB/Thunderbolt

| Type | Driver | Config |
|------|--------|--------|
| USB 3.x | xhci_hcd | CONFIG_USB_XHCI_HCD |
| USB 2.0 | ehci_hcd | CONFIG_USB_EHCI_HCD |
| Thunderbolt/USB4 | thunderbolt | CONFIG_USB4 |

### Bluetooth

| Chip | Driver | Firmware |
|------|--------|----------|
| Intel | btintel | intel/* |
| Realtek | btrtl | rtl_bt/* |
| Broadcom | btbcm | brcm/* |

---

## Current Status (from exploration)

### Kernel Config (mostly complete)
- i915, amdgpu, nouveau (modules)
- NVMe (built-in), AHCI (built-in)
- e1000e, igc, r8169 (built-in)
- iwlwifi, ath*, rtw88, rtw89, mt76 (modules)
- snd_hda_intel, realtek codec (built-in)
- xhci, ehci (built-in)
- **r8152** (USB Ethernet) - NOT ENABLED
- **Thunderbolt** - NOT ENABLED

### Firmware (complete)
- Intel WiFi: 157 files
- AMD GPU: 674 files
- Intel GPU: 124 files
- Realtek WiFi: 24 files
- Bluetooth: 329 files
- Microcode: Intel (152), AMD (6)

---

## Implementation

### Directory Structure

```
testing/hardware-compat/
├── Cargo.toml
├── README.md
├── PLAN.md                # This file
├── src/
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # HardwareCompatChecker
│   ├── kernel.rs          # Kernel config verification
│   ├── firmware.rs        # Firmware blob verification
│   ├── profiles/
│   │   ├── mod.rs         # Profile trait + registry
│   │   ├── nuc.rs         # Intel NUC profile
│   │   ├── thinkpad.rs    # ThinkPad profile
│   │   ├── desktop.rs     # Generic desktop profile
│   │   ├── framework.rs   # Framework laptop profile
│   │   └── mini_pc.rs     # AMD mini PC profile
│   └── report.rs          # Report generation
```

### Core API

```rust
// src/lib.rs
pub struct HardwareCompatChecker {
    kernel_config: PathBuf,  // Path to .config
    firmware_dir: PathBuf,   // Path to firmware directory
    staging_dir: PathBuf,    // Path to stage rootfs source staging
}

impl HardwareCompatChecker {
    pub fn verify_kernel_config(&self) -> CompatResult;
    pub fn verify_firmware(&self) -> CompatResult;
    pub fn verify_profile(&self, profile: &dyn HardwareProfile) -> CompatResult;
    pub fn generate_report(&self) -> CompatibilityReport;
}
```

### Profile Trait

```rust
// src/profiles/mod.rs
pub trait HardwareProfile {
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    // Required kernel configs
    fn required_kernel_configs(&self) -> &[KernelRequirement];

    // Required firmware
    fn required_firmware(&self) -> &[FirmwareRequirement];

    // Optional but recommended
    fn recommended_kernel_configs(&self) -> &[KernelRequirement] { &[] }
    fn recommended_firmware(&self) -> &[FirmwareRequirement] { &[] }
}

pub struct KernelRequirement {
    pub config: &'static str,      // e.g., "CONFIG_DRM_I915"
    pub value: ConfigValue,         // Y, M, or specific value
    pub reason: &'static str,       // "Intel integrated graphics"
    pub severity: Severity,         // Critical, High, Medium, Low
}

pub struct FirmwareRequirement {
    pub pattern: &'static str,      // e.g., "i915/*" or "iwlwifi-cc-*"
    pub min_files: usize,           // Minimum files expected
    pub reason: &'static str,
    pub severity: Severity,
}
```

### Hardware Profiles

#### 1. Intel NUC Profile
- Intel integrated graphics (i915)
- NVMe + SATA storage
- Intel I219 Ethernet
- Intel WiFi (iwlwifi)
- Intel HDA audio
- Thunderbolt (NUC 8+)

#### 2. ThinkPad Profile
- Intel or AMD variants
- Intel/AMD integrated graphics
- NVMe storage
- Intel/Realtek Ethernet
- Intel WiFi
- TrackPoint/touchpad support

#### 3. Framework Laptop Profile
- Intel or AMD variants
- Thunderbolt critical
- USB-C display support
- Intel/AMD integrated graphics

#### 4. Desktop Profile
Most permissive - all GPU options, all network options, discrete GPU support

#### 5. Mini PC Profile (AMD)
- AMD Ryzen (Beelink, Minisforum)
- AMD GPU (Vega/RDNA)
- Realtek Ethernet common
- Various WiFi chips

### CLI Interface

```bash
# Verify against all profiles
cargo run -- verify

# Verify against specific profile
cargo run -- verify --profile nuc

# List available profiles
cargo run -- profiles

# Generate detailed report
cargo run -- report --output hardware-compat.md

# Check specific hardware category
cargo run -- check gpu
cargo run -- check wifi
cargo run -- check storage
```

### Output Format

```
Hardware Compatibility Verification
===================================

Profile: Intel NUC (7th-13th gen)
─────────────────────────────────

Kernel Configuration:
  ✓ CONFIG_DRM_I915=m          Intel integrated graphics
  ✓ CONFIG_NVME_CORE=y         NVMe SSD support
  ✓ CONFIG_E1000E=y            Intel I219 Ethernet
  ✓ CONFIG_IWLWIFI=m           Intel WiFi
  ✗ CONFIG_USB4 NOT FOUND      Thunderbolt/USB4 [MEDIUM]

Firmware:
  ✓ i915/*                     124 files (need 50+)
  ✓ iwlwifi-*                  157 files (need 100+)
  ✓ intel/*                    297 files (need 100+)
  ✓ intel-ucode/*              152 files (need 100+)

Result: PASS with warnings
  - Missing: Thunderbolt support (NUC 8+ feature)

───────────────────────────────────
Overall: 4/5 profiles PASS
  ✓ Intel NUC        (PASS with warnings)
  ✓ ThinkPad         (PASS)
  ✓ Desktop          (PASS)
  ✓ Mini PC (AMD)    (PASS)
  ✗ Framework        (FAIL - Thunderbolt required)
```

---

## Immediate Fixes Required

Before the test can pass for all profiles:

1. **Enable CONFIG_USB_RTL8152** - USB Ethernet (many docks use this)
2. **Enable CONFIG_USB4** - Thunderbolt/USB4 (Framework, NUC 8+, modern laptops)

These require kernel config changes in `leviso`.

---

## Success Criteria

1. **Critical**: All profiles pass critical checks (GPU, storage, basic network)
2. **High**: Most profiles pass high-priority checks (audio, WiFi)
3. **Medium**: Known gaps documented (Thunderbolt currently missing)
4. **Report**: Clear output showing what hardware IS and ISN'T supported

---

## Integration Points

### Build Integration (leviso)

After ISO build completes, run hardware-compat verification:

```rust
fn verify_hardware_compat(output_dir: &Path) -> Result<()> {
    let checker = HardwareCompatChecker::new(
        output_dir.join("kernel-build/.config"),
        output_dir.join("staging/lib/firmware"),
        output_dir.join("staging"),
    );

    let report = checker.generate_report();

    if report.has_critical_failures() {
        bail!("Hardware compatibility check failed:\n{}", report);
    }

    if report.has_warnings() {
        println!("Warning: {}", report.warnings_summary());
    }

    Ok(())
}
```

### CI Integration

```yaml
# .github/workflows/build.yml
- name: Verify hardware compatibility
  run: |
    cd testing/hardware-compat
    cargo run -- verify
```
