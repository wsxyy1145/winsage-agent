use regex::Regex;

/// 安全检查器
pub struct SecurityChecker {
    dangerous_patterns: Vec<Regex>,
    caution_patterns: Vec<Regex>,
}

impl SecurityChecker {
    pub fn new() -> Self {
        // 危险命令模式（必须在沙箱中执行）
        let dangerous_patterns = vec![
            Regex::new(r"(?i)\b(format|diskpart|fdisk)\b").unwrap(),
            Regex::new(r"(?i)\b(del|rm|rmdir)\s+/[a-z]*s\b").unwrap(),
            Regex::new(r"(?i)\b(reg\s+delete)\b").unwrap(),
            Regex::new(r"(?i)\b(taskkill)\b.*\/f").unwrap(),
            Regex::new(r"(?i)\b(shutdown)\b.*\/[rs].*\/t").unwrap(),
            Regex::new(r"(?i)\b(net\s+user)\b.*\/delete").unwrap(),
            Regex::new(r"(?i)(?:\.exe|\.bat|\.cmd|\.ps1).*-(?:force|destroy|wipe)").unwrap(),
        ];

        // 需谨慎的命令模式
        let caution_patterns = vec![
            Regex::new(r"(?i)\b(install|msiexec)\b").unwrap(),
            Regex::new(r"(?i)\b(chmod|chown)\b").unwrap(),
            Regex::new(r"(?i)\b(netsh|iptables)\b").unwrap(),
            Regex::new(r"(?i)\b(sc\s+create)\b").unwrap(),
            Regex::new(r"(?i)\b(schtasks)\b").unwrap(),
            Regex::new(r"(?i)\b(powershell)\b.*-executionpolicy").unwrap(),
        ];

        Self {
            dangerous_patterns,
            caution_patterns,
        }
    }

    /// 检查命令的安全等级
    pub fn check_command(&self, command: &str) -> SafetyLevel {
        // 检查是否为危险命令
        for pattern in &self.dangerous_patterns {
            if pattern.is_match(command) {
                return SafetyLevel::Dangerous;
            }
        }

        // 检查是否需要谨慎
        for pattern in &self.caution_patterns {
            if pattern.is_match(command) {
                return SafetyLevel::Caution;
            }
        }

        // 默认安全
        SafetyLevel::Safe
    }
}

/// 安全等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    /// 安全，可以直接执行
    Safe,
    /// 需要谨慎，建议使用沙箱
    Caution,
    /// 危险，必须在沙箱中执行
    Dangerous,
}
