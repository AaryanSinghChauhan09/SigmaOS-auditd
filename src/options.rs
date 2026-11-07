//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Start,
    Stop,
    Status,
    ListRules,
    ListEvents,
    AddRule,
    DeleteRule,
}

#[derive(Debug, Clone)]
pub struct AuditdOptions {
    pub mode: Mode,
    pub rule: Option<String>,
}

impl Default for AuditdOptions {
    fn default() -> Self { Self { mode: Mode::Default, rule: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(AuditdOptions::default().mode, Mode::Default); }
}
