# CLAUDE.md - hardware-compat

## What is hardware-compat?

Hardware compatibility verification for LevitateOS. Checks CPU features, firmware, drivers.

## What Belongs Here

- CPU feature detection (x86-64-v3, etc.)
- Firmware checks (UEFI, Secure Boot)
- Driver availability verification

## What Does NOT Belong Here

| Don't put here | Put it in |
|----------------|-----------|
| Installation tests | `testing/install-tests/` |
| Rootfs tests | `testing/rootfs-tests/` |

## Commands

```bash
cargo build
cargo run
```

## Target Hardware

LevitateOS requires x86-64-v3 (Haswell 2013+). This crate verifies compatibility.
