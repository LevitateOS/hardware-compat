use std::path::PathBuf;
use anyhow::{Result, Context};
use std::collections::HashMap;
use glob::glob;

pub mod kernel;
pub mod firmware;
pub mod profiles;
pub mod report;

pub use kernel::{KernelRequirement, ConfigValue};
pub use firmware::FirmwareRequirement;
pub use profiles::HardwareProfile;
pub use report::{CompatibilityReport, Severity, CompatResult};

pub struct HardwareCompatChecker {
    kernel_config_path: PathBuf,
    firmware_dir: PathBuf,
    staging_dir: PathBuf,
}

impl HardwareCompatChecker {
    pub fn new(kernel_config: PathBuf, firmware_dir: PathBuf, staging_dir: PathBuf) -> Self {
        Self {
            kernel_config_path: kernel_config,
            firmware_dir,
            staging_dir,
        }
    }

    pub fn load_kernel_config(&self) -> Result<HashMap<String, String>> {
        let content = std::fs::read_to_string(&self.kernel_config_path)
            .with_context(|| format!("Failed to read kernel config at {:?}", self.kernel_config_path))?;
        
        let mut configs = HashMap::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                configs.insert(key.to_string(), value.to_string());
            }
        }
        Ok(configs)
    }

    pub fn verify_firmware_requirement(&self, req: &FirmwareRequirement) -> CompatResult {
        let pattern = self.firmware_dir.join(req.pattern);
        let pattern_str = pattern.to_string_lossy();
        
        match glob(&pattern_str) {
            Ok(paths) => {
                let count = paths.filter_map(Result::ok).count();
                if count >= req.min_files {
                    CompatResult::Pass(format!("Found {} files", count))
                } else if count > 0 {
                    CompatResult::Warning(format!("Found only {} files, expected {}", count, req.min_files))
                } else {
                    CompatResult::Fail(format!("No files found matching {}", req.pattern))
                }
            }
            Err(e) => CompatResult::Fail(format!("Invalid glob pattern: {}", e)),
        }
    }

    pub fn verify_profile(&self, profile: &dyn HardwareProfile) -> Result<CompatibilityReport> {
        let configs = self.load_kernel_config()?;
        let mut report = CompatibilityReport::new(profile.name().to_string(), profile.description().to_string());

        // Verify kernel configs
        for req in profile.required_kernel_configs() {
            let actual = configs.get(req.config).cloned();
            let result = req.verify(actual.as_deref());
            report.add_kernel_result(req.clone(), result);
        }

        // Verify firmware
        for req in profile.required_firmware() {
            let result = self.verify_firmware_requirement(req);
            report.add_firmware_result(req.clone(), result);
        }

        Ok(report)
    }
}
