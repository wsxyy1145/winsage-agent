use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{info, warn};

/// Windows Sandbox 管理器
#[derive(Clone)]
pub struct WindowsSandbox {
    enabled: bool,
    timeout_seconds: u64,
    shared_folder: Option<PathBuf>,
}

impl WindowsSandbox {
    pub fn new(enabled: bool, timeout_seconds: u64, shared_folder: Option<String>) -> Self {
        let shared_folder = shared_folder.map(PathBuf::from);

        // 检查 Windows Sandbox 是否可用
        if enabled && !Self::is_available() {
            warn!("Windows Sandbox 不可用，将禁用沙箱功能");
            return Self {
                enabled: false,
                timeout_seconds,
                shared_folder,
            };
        }

        Self {
            enabled,
            timeout_seconds,
            shared_folder,
        }
    }

    /// 检查 Windows Sandbox 是否可用
    fn is_available() -> bool {
        // 检查 Windows 版本和企业版/专业版
        #[cfg(target_os = "windows")]
        {
            // 尝试运行 PowerShell 命令检查
            true // 简化实现，实际应检查系统版本
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    /// 检查沙箱是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// 在沙箱中执行命令
    pub async fn execute_in_sandbox(&self, command: &str) -> Result<SandboxResult, SandboxError> {
        if !self.enabled {
            return Err(SandboxError::NotEnabled);
        }

        info!("在沙箱中执行命令: {}", command);

        // 创建临时 WSB 配置文件
        let wsb_content = self.create_wsb_config(command)?;
        let wsb_path = self.save_wsb_file(&wsb_content)?;

        // 启动 Windows Sandbox
        let start_time = std::time::Instant::now();

        let output = Command::new("powershell")
            .arg("-Command")
            .arg(format!("Start-Process -FilePath \"{}\" -Wait", wsb_path.display()))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| SandboxError::ExecutionError(e.to_string()))?;

        let elapsed = start_time.elapsed();

        // 清理临时文件
        if let Err(e) = std::fs::remove_file(&wsb_path) {
            warn!("清理临时文件失败: {}", e);
        }

        if output.status.success() {
            Ok(SandboxResult {
                success: true,
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                execution_time: elapsed,
            })
        } else {
            Ok(SandboxResult {
                success: false,
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                execution_time: elapsed,
            })
        }
    }

    /// 创建 WSB 配置文件内容
    fn create_wsb_config(&self, command: &str) -> Result<String, SandboxError> {
        let mut config = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<Configuration>
    <MappedFolders>"#);

        if let Some(ref folder) = self.shared_folder {
            config.push_str(&format!(
                r#"
        <MappedFolder>
            <HostFolder>{}</HostFolder>
            <ReadOnly>false</ReadOnly>
        </MappedFolder>"#,
                folder.display()
            ));
        }

        config.push_str(&format!(
            r#"
    </MappedFolders>
    <LogonCommand>
        <Command>cmd /c "{}"</Command>
    </LogonCommand>
</Configuration>"#,
            command.replace('"', "&quot;")
        ));

        Ok(config)
    }

    /// 保存 WSB 文件到临时目录
    fn save_wsb_file(&self, content: &str) -> Result<PathBuf, SandboxError> {
        let temp_dir = std::env::temp_dir();
        let wsb_path = temp_dir.join(format!("winsage_{}.wsb", uuid::Uuid::new_v4()));

        std::fs::write(&wsb_path, content)
            .map_err(|e| SandboxError::FileError(e.to_string()))?;

        Ok(wsb_path)
    }

    /// 获取沙箱状态信息
    pub fn status(&self) -> SandboxStatus {
        SandboxStatus {
            enabled: self.enabled,
            timeout_seconds: self.timeout_seconds,
            has_shared_folder: self.shared_folder.is_some(),
            available: Self::is_available(),
        }
    }
}

/// 沙箱执行结果
#[derive(Debug)]
pub struct SandboxResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub execution_time: std::time::Duration,
}

/// 沙箱状态
#[derive(Debug)]
pub struct SandboxStatus {
    pub enabled: bool,
    pub timeout_seconds: u64,
    pub has_shared_folder: bool,
    pub available: bool,
}

/// 沙箱错误类型
#[derive(Debug, thiserror::Error)]
pub enum SandboxError {
    #[error("沙箱未启用")]
    NotEnabled,

    #[error("执行命令失败: {0}")]
    ExecutionError(String),

    #[error("文件操作失败: {0}")]
    FileError(String),

    #[error("超时")]
    Timeout,
}
