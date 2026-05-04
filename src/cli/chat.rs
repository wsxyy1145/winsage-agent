use crate::agent::Agent;
use console::style;
use dialoguer::Input;
use tracing::info;

/// 交互式聊天命令
pub async fn chat_command(agent: &mut Agent) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", style("=== WinSage AI Agent ===").green().bold());
    println!("{}", style("输入 'quit' 或 'exit' 退出").dim());
    println!("{}", style("输入 'help' 查看帮助\n").dim());

    // 如果没有当前对话，创建一个新对话
    if agent.current_conversation_id().is_none() {
        match agent.start_conversation(Some("新对话".to_string())).await {
            Ok(id) => info!("创建对话: {}", id),
            Err(e) => eprintln!("创建对话失败: {}", e),
        }
    }

    loop {
        // 获取用户输入
        let input: String = Input::new()
            .with_prompt(style("你").cyan().to_string())
            .interact_text()?;

        let input = input.trim();

        // 检查退出命令
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("\n{}", style("再见！").green());
            break;
        }

        // 检查帮助命令
        if input.eq_ignore_ascii_case("help") {
            print_help();
            continue;
        }

        if input.is_empty() {
            continue;
        }

        // 显示"思考中"提示
        print!("{}", style("助手思考中...").dim());

        // 发送消息并获取回复
        match agent.send_message(input).await {
            Ok(response) => {
                println!("\r{}\r", " ".repeat(20)); // 清除"思考中"提示
                println!("\n{}", style("助手").yellow().bold());
                println!("{}\n", response.content);
            }
            Err(e) => {
                println!("\r{}\r", " ".repeat(20));
                eprintln!("\n{}", style(format!("错误: {}", e)).red());
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("\n{}", style("可用命令:").cyan().bold());
    println!("  quit/exit - 退出聊天");
    println!("  help      - 显示此帮助信息");
    println!("  clear     - 清屏");
    println!("\n直接输入消息与AI对话\n");
}
