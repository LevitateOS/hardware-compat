use colored::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub enum CompatResult {
    Pass(String),
    Warning(String),
    Fail(String),
}

pub struct CompatibilityReport {
    pub name: String,
    pub description: String,
    pub kernel_results: Vec<(crate::kernel::KernelRequirement, CompatResult)>,
    pub firmware_results: Vec<(crate::firmware::FirmwareRequirement, CompatResult)>,
}

impl CompatibilityReport {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            kernel_results: Vec::new(),
            firmware_results: Vec::new(),
        }
    }

    pub fn add_kernel_result(
        &mut self,
        req: crate::kernel::KernelRequirement,
        result: CompatResult,
    ) {
        self.kernel_results.push((req, result));
    }

    pub fn add_firmware_result(
        &mut self,
        req: crate::firmware::FirmwareRequirement,
        result: CompatResult,
    ) {
        self.firmware_results.push((req, result));
    }

    pub fn has_critical_failures(&self) -> bool {
        self.kernel_results.iter().any(|(req, res)| {
            matches!(res, CompatResult::Fail(_)) && req.severity == Severity::Critical
        }) || self.firmware_results.iter().any(|(req, res)| {
            matches!(res, CompatResult::Fail(_)) && req.severity == Severity::Critical
        })
    }

    pub fn status_string(&self) -> String {
        if self.has_critical_failures() {
            "FAIL".red().bold().to_string()
        } else if self
            .kernel_results
            .iter()
            .any(|(_, res)| matches!(res, CompatResult::Fail(_) | CompatResult::Warning(_)))
            || self
                .firmware_results
                .iter()
                .any(|(_, res)| matches!(res, CompatResult::Fail(_) | CompatResult::Warning(_)))
        {
            "PASS WITH WARNINGS".yellow().bold().to_string()
        } else {
            "PASS".green().bold().to_string()
        }
    }

    pub fn print_summary(&self) {
        println!("\nProfile: {}", self.name.cyan().bold());
        println!("Description: {}", self.description.dimmed());
        println!("--------------------------------------------------");

        println!("\nKernel Configuration:");
        for (req, res) in &self.kernel_results {
            match res {
                CompatResult::Pass(v) => println!(
                    "  {} {:<30}= {:<10} {}",
                    "✓".green(),
                    req.config,
                    v,
                    req.reason.dimmed()
                ),
                CompatResult::Warning(m) => println!(
                    "  {} {:<30} {:<10} {}",
                    "!".yellow(),
                    req.config,
                    m.yellow(),
                    req.reason.dimmed()
                ),
                CompatResult::Fail(m) => println!(
                    "  {} {:<30} {:<10} {}",
                    "✗".red(),
                    req.config,
                    m.red(),
                    req.reason.dimmed()
                ),
            }
        }

        println!("\nFirmware:");
        for (req, res) in &self.firmware_results {
            match res {
                CompatResult::Pass(m) => println!(
                    "  {} {:<30} {:<10} {}",
                    "✓".green(),
                    req.pattern,
                    m,
                    req.reason.dimmed()
                ),
                CompatResult::Warning(m) => println!(
                    "  {} {:<30} {:<10} {}",
                    "!".yellow(),
                    req.pattern,
                    m.yellow(),
                    req.reason.dimmed()
                ),
                CompatResult::Fail(m) => println!(
                    "  {} {:<30} {:<10} {}",
                    "✗".red(),
                    req.pattern,
                    m.red(),
                    req.reason.dimmed()
                ),
            }
        }

        println!("\nResult: {}", self.status_string());
        println!("==================================================");
    }
}
