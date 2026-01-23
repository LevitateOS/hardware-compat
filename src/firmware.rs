use crate::report::Severity;

#[derive(Debug, Clone)]
pub struct FirmwareRequirement {
    pub pattern: &'static str,
    pub min_files: usize,
    pub reason: &'static str,
    pub severity: Severity,
}
