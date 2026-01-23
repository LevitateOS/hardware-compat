# hardware-compat

Static verification test for LevitateOS hardware compatibility. Checks kernel drivers and firmware for modern hardware support.

**This is a BUILD verification test, not a runtime hardware test.** It verifies the ISO will work on real hardware before users try it.

## Status

**Not yet implemented.** See `PLAN.md` for implementation details.

## What It Does

Verifies that the LevitateOS build includes:
1. Kernel drivers (CONFIG_* options) for target hardware
2. Firmware blobs for WiFi, GPU, Bluetooth, etc.

## Target Hardware

| Category | Examples |
|----------|----------|
| Intel NUCs | 7th-13th gen NUCs |
| Mini PCs | Beelink, Minisforum, GMKtec |
| Laptops | ThinkPad, Dell XPS, HP, Framework, ASUS |
| Desktops | Custom builds, pre-builts |

## Hardware Profiles

| Profile | Description |
|---------|-------------|
| `nuc` | Intel NUC (7th-13th gen) |
| `thinkpad` | Lenovo ThinkPad laptops |
| `framework` | Framework laptops |
| `desktop` | Generic desktop (most permissive) |
| `mini_pc` | AMD-based mini PCs |

## Usage (planned)

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

## What It Checks

### Kernel Configuration
- GPU drivers (i915, amdgpu, nouveau)
- Storage (NVMe, AHCI, USB storage, eMMC)
- Network (e1000e, igc, r8169, iwlwifi, ath*, rtw88/89)
- Audio (snd_hda_intel, realtek codec)
- USB (xhci, ehci, Thunderbolt)

### Firmware
- Intel WiFi (iwlwifi-*)
- AMD GPU (amdgpu/*)
- Intel GPU (i915/*)
- Bluetooth (intel/*, rtl_bt/*, brcm/*)
- CPU microcode (intel-ucode/*, amd-ucode/*)

## Known Gaps (current build)

| Missing | Impact |
|---------|--------|
| CONFIG_USB_RTL8152 | USB Ethernet adapters/docks won't work |
| CONFIG_USB4 | Thunderbolt/USB4 devices won't work |

## Requirements

- Built LevitateOS ISO with kernel config at `leviso/output/kernel-build/.config`
- Firmware directory at `leviso/output/staging/lib/firmware`

## License

MIT
