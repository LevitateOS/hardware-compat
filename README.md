# hardware-compat

Static verification test for LevitateOS hardware compatibility. Checks kernel drivers and firmware for modern hardware support.

**This is a BUILD verification test, not a runtime hardware test.** It verifies the ISO will work on real hardware before users try it.

## Status
**Active.** Used in CI to verify build artifacts.

## Testing

Run the test suite (unit + integration):
```bash
cargo test
```

## CI / Release

This tool uses a strict continuous delivery pipeline (`.github/workflows/ci.yml`).
1.  **Strict Verification**: Runs `cargo test` (Unit & Integration) on every push. **Releases are blocked if tests fail.**
2.  **Auto-Release**: If tests pass on `main`, it automatically bumps the version, creates a GitHub Release, and publishes binary artifacts.

## Requirements

- Built LevitateOS ISO with kernel config at `.artifacts/kernel/levitate/current/kernel-build/.config`
- Firmware directory at `.artifacts/work/levitate/stage01-rootfs-provider/rocky/rootfs-minimal/usr/lib/firmware`

## License

MIT
