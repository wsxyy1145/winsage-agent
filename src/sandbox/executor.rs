use crate::sandbox::windows_sandbox::WindowsSandbox;
use crate::sandbox::security::{SecurityChecker, SafetyLevel};
use tracing::{info, warn};

/// 命令执行器
pub struct CommandExecutor {
    sandbox: WindowsSandbox,
    security_checker: SecurityChecker,
}

impl CommandExecutor {
    pub fn new(sandbox: WindowsSandbox) -> Self {
        let security_checker = SecurityChecker::new();
        Self {
            sandbox,
            security_checker,
        }
    }

    /// 执行命令（自动判断是否使用沙箱）
    pub async fn execute(&self, command: &str) -> Result<ExecutionResult, ExecutorError> {
        info!("执行命令: {}", command);

        // 安全检查
        let safety_level = self.security_checker.check_command(command);

        match safety_level {
            SafetyLevel::Safe => {
                // 安全命令，直接执行
                self.execute_local(command).await
            }
            SafetyLevel::Caution => {
                warn!("命令需要谨慎执行: {}", command);
                // 中等风险命令，询问用户或使用沙箱
                if self.sandbox.is_enabled() {
                    self.execute_sandboxed(command).await
                } else {
                    self.execute_local(command).await
                }
            }
            SafetyLevel::Dangerous => {
                // 危险命令，必须使用沙箱
                if self.sandbox.is_enabled() {
                    info!("危险命令，使用沙箱执行: {}", command);
                    self.execute_sandboxed(command).await
                } else {
                    Err(ExecutorError::SecurityError(
                        "危险命令被阻止，且沙箱不可用".to_string(),
                    ))
                }
            }
        }
    }

    /// 在本地执行命令
    async fn execute_local(&self, command: &str) -> Result<ExecutionResult, ExecutorError> {
        let output = tokio::process::Command::new("cmd")
            .arg("/C")
            .arg(command)
            .output()
            .await
            .map_err(|e| ExecutorError::ExecutionError(e.to_string()))?;

        Ok(ExecutionResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            sandboxed: false,
        })
    }

    /// 在沙箱中执行命令
    async fn execute_sandboxed(&self, command: &str) -> Result<ExecutionResult, ExecutorError> {
        let result = self
            .sandbox
            .execute_in_sandbox(command)
            .await
            .map_err(|e| ExecutorError::SandboxError(e.to_string()))?;

        Ok(ExecutionResult {
            success: result.success,
            stdout: result.stdout,
            stderr: result.stderr,
            sandboxed: true,
        })
    }
}

/// 执行结果
#[derive(Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub sandboxed: bool,
}

/// 执行器错误类型
#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    #[error("执行失败: {0}")]
    ExecutionError(String),

    #[error("安全错误: {0}")]
    SecurityError(String),

    #[error("沙箱错误: {0}")]
    SandboxError(String),
}
