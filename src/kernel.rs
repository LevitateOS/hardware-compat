use crate::report::CompatResult;

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    Y,
    M,
    Value(String),
}

#[derive(Debug, Clone)]
pub struct KernelRequirement {
    pub config: &'static str,
    pub expected: ConfigValue,
    pub reason: &'static str,
    pub severity: crate::report::Severity,
}

impl KernelRequirement {
    pub fn verify(&self, actual: Option<&str>) -> CompatResult {
        match (actual, &self.expected) {
            // Exact matches
            (Some("y"), ConfigValue::Y) => CompatResult::Pass("y".to_string()),
            (Some("m"), ConfigValue::M) => CompatResult::Pass("m".to_string()),
            // Built-in (y) satisfies module (m) requirement - built-in is strictly better
            (Some("y"), ConfigValue::M) => CompatResult::Pass("y (built-in)".to_string()),
            // String value match
            (Some(v), ConfigValue::Value(expected)) if v == expected => {
                CompatResult::Pass(v.to_string())
            }
            // Module when built-in required is a warning, not failure
            (Some("m"), ConfigValue::Y) => {
                CompatResult::Warning("m (module, expected built-in)".to_string())
            }
            // Other mismatches
            (Some(v), _) => {
                CompatResult::Fail(format!("Found {}, expected {:?}", v, self.expected))
            }
            (None, _) => CompatResult::Fail("Not found".to_string()),
        }
    }
}
