// 集成测试示例
// 运行命令: cargo test --test integration_test

#[cfg(test)]
mod tests {
    use winsage::config::AppConfig;
    use winsage::models::provider::{Message, ModelProvider};
    use winsage::sandbox::security::SecurityChecker;

    #[tokio::test]
    async fn test_config_load() {
        // 测试配置加载
        let config = AppConfig::load();
        assert!(config.is_ok(), "配置应该能够加载");
    }

    #[test]
    fn test_security_checker_safe_command() {
        let checker = SecurityChecker::new();

        // 测试安全命令
        let result = checker.check_command("echo hello");
        assert_eq!(result, winsage::sandbox::security::SafetyLevel::Safe);
    }

    #[test]
    fn test_security_checker_dangerous_command() {
        let checker = SecurityChecker::new();

        // 测试危险命令
        let result = checker.check_command("format C:");
        assert_eq!(result, winsage::sandbox::security::SafetyLevel::Dangerous);
    }

    #[test]
    fn test_message_creation() {
        // 测试消息创建
        let user_msg = Message::user("Hello");
        assert_eq!(user_msg.content, "Hello");

        let system_msg = Message::system("You are helpful");
        assert_eq!(system_msg.role, winsage::models::provider::MessageRole::System);
    }
}
