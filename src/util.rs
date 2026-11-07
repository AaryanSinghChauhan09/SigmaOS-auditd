//! Auditd model — system call auditing.

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub timestamp: String,
    pub type_: String,
    pub syscall: String,
    pub pid: u32,
    pub uid: u32,
    pub comm: String,
    pub exe: String,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub action: String,
    pub syscall: String,
    pub uid: Option<u32>,
    pub path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Auditd {
    pub enabled: bool,
    pub backlog: u32,
}

impl Default for Auditd {
    fn default() -> Self { Self { enabled: false, backlog: 8192 } }
}

impl Auditd {
    pub fn new() -> Self { Self::default() }

    pub fn add_rule(&self, rule: Rule) -> bool {
        !rule.syscall.is_empty()
    }

    pub fn list_events(&self) -> Vec<AuditEvent> {
        vec![AuditEvent { timestamp: "2024-01-01T00:00:00Z".into(), type_: "SYSCALL".into(), syscall: "execve".into(), pid: 1, uid: 0, comm: "init".into(), exe: "/sbin/init".into(), success: true }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rules_events() {
        let a = Auditd::new();
        assert!(a.add_rule(Rule { action: "always".into(), syscall: "execve".into(), uid: None, path: None }));
        assert_eq!(a.list_events().len(), 1);
    }
}
