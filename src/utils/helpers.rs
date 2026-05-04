use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// 初始化日志系统
pub fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "winsage=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// 打印欢迎信息
pub fn print_welcome() {
    use console::style;

    println!("\n{}", style("╔══════════════════════════════════════╗").green());
    println!("{}", style("║     WinSage AI Agent v0.1.0          ║").green());
    println!("{}", style("║     Windows Terminal AI Assistant    ║").green());
    println!("{}", style("╚══════════════════════════════════════╝").green());
    println!();
}
