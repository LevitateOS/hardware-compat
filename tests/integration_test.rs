use hardware_compat::{
    CompatResult, ConfigValue, FirmwareRequirement, HardwareCompatChecker, HardwareProfile,
    KernelRequirement, Severity,
};
use std::io::Write;
use tempfile::TempDir;

struct MockProfile {
    kernel_reqs: Vec<KernelRequirement>,
    firmware_reqs: Vec<FirmwareRequirement>,
}

impl MockProfile {
    fn new() -> Self {
        Self {
            kernel_reqs: vec![
                KernelRequirement {
                    config: "CONFIG_MOCK_REQUIRED",
                    expected: ConfigValue::Y,
                    reason: "Required feature",
                    severity: Severity::Critical,
                },
                KernelRequirement {
                    config: "CONFIG_MOCK_MODULE",
                    expected: ConfigValue::M,
                    reason: "Module feature",
                    severity: Severity::Medium,
                },
            ],
            firmware_reqs: vec![FirmwareRequirement {
                pattern: "mock/fw_*.bin",
                min_files: 2,
                reason: "Mock firmware",
                severity: Severity::Critical,
            }],
        }
    }
}

impl HardwareProfile for MockProfile {
    fn name(&self) -> &str {
        "Mock Profile"
    }

    fn description(&self) -> &str {
        "A profile for testing check logic"
    }

    fn required_kernel_configs(&self) -> &[KernelRequirement] {
        &self.kernel_reqs
    }

    fn required_firmware(&self) -> &[FirmwareRequirement] {
        &self.firmware_reqs
    }
}

#[test]
fn test_integration_check() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".config");
    let firmware_dir = temp_dir.path().join("firmware");
    std::fs::create_dir(&firmware_dir).unwrap();
    std::fs::create_dir(firmware_dir.join("mock")).unwrap();

    // 1. Create a passing config
    let mut f = std::fs::File::create(&config_path).unwrap();
    writeln!(f, "CONFIG_MOCK_REQUIRED=y").unwrap();
    writeln!(f, "CONFIG_MOCK_MODULE=m").unwrap();

    // 2. Create firmware files
    std::fs::File::create(firmware_dir.join("mock/fw_1.bin")).unwrap();
    std::fs::File::create(firmware_dir.join("mock/fw_2.bin")).unwrap();

    // 3. Run checker
    let checker = HardwareCompatChecker::new(config_path.clone(), firmware_dir.clone());
    let profile = MockProfile::new();
    let report = checker.verify_profile(&profile).unwrap();

    assert!(
        !report.has_critical_failures(),
        "Should pass with correct config/fw"
    );
    assert_eq!(report.kernel_results.len(), 2);
    assert_eq!(report.firmware_results.len(), 1);

    // 4. Test Failure case (Missing FW) - Remove ALL files to force Fail
    std::fs::remove_file(firmware_dir.join("mock/fw_1.bin")).unwrap();
    std::fs::remove_file(firmware_dir.join("mock/fw_2.bin")).unwrap();
    let report_fail = checker.verify_profile(&profile).unwrap();

    let fw_failed = report_fail
        .firmware_results
        .iter()
        .any(|(_, res)| matches!(res, CompatResult::Fail(_)));
    assert!(fw_failed, "Should fail with missing firmware");
    assert!(
        report_fail.has_critical_failures(),
        "Missing firmware is critical"
    );

    // 5. Test Failure case (Missing Config)
    let mut f = std::fs::File::create(&config_path).unwrap(); // Overwrite
    writeln!(f, "CONFIG_MOCK_MODULE=m").unwrap(); // Missing REQUIRED
                                                  // Must run verify again
                                                  // Re-create checker because config is read on each verify? No, verify_profile calls load_kernel_config()
    let report_fail_config = checker.verify_profile(&profile).unwrap();

    let config_failed = report_fail_config.kernel_results.iter().any(|(req, res)| {
        req.config == "CONFIG_MOCK_REQUIRED" && matches!(res, CompatResult::Fail(_))
    });
    assert!(config_failed, "Should fail with missing critical config");
    assert!(report_fail_config.has_critical_failures());
}
