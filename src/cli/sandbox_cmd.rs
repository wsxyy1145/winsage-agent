use crate::sandbox::windows_sandbox::WindowsSandbox;
use console::style;

/// 沙箱状态命令
pub fn sandbox_command(sandbox: &WindowsSandbox) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("=== 沙箱状态 ===").green().bold());

    let status = sandbox.status();

    println!("\n可用状态: {}", if status.available { "是" } else { "否" });
    println!(
        "启用状态: {}",
        if status.enabled { "是" } else { "否" }
    );
    println!("超时时间: {} 秒", status.timeout_seconds);
    println!(
        "共享文件夹: {}",
        if status.has_shared_folder {
            "已配置"
        } else {
            "未配置"
        }
    );

    if !status.available && status.enabled {
        println!(
            "\n{}",
            style("警告: 沙箱已启用但不可用，请检查 Windows 版本").yellow()
        );
    }

    Ok(())
}
