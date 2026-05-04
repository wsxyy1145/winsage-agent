# WinSage 项目总结

## 项目概述

WinSage 是一个功能完整的Windows终端AI Agent，采用Rust语言开发，具备以下核心特性：

### ✅ 已实现的功能

1. **多模型供应商支持**
   - OpenAI GPT系列（GPT-4, GPT-3.5）
   - Anthropic Claude系列
   - Azure OpenAI Service
   - Ollama本地模型
   - 统一的ModelProvider trait抽象

2. **安全隔离沙箱**
   - Windows Sandbox集成
   - 命令安全检查器（三级安全等级）
   - 自动危险命令检测
   - 共享文件夹支持

3. **长期记忆系统**
   - PostgreSQL数据库存储
   - 对话历史管理
   - 知识库系统
   - 向量嵌入支持（预留）

4. **交互式CLI**
   - Clap命令行框架
   - 配置管理向导
   - 聊天历史记录
   - 彩色终端输出

5. **模块化架构**
   - 清晰的代码分层
   - 易于扩展新模型
   - 完善的错误处理
   - Tracing日志系统

## 项目结构

```
WinSage-agent/
├── Cargo.toml              # Rust项目配置
├── README.md               # 项目说明文档
├── EXAMPLES.md             # 使用示例
├── docker-compose.yml      # PostgreSQL Docker配置
├── setup.ps1               # Windows安装脚本
│
├── config/
│   └── default.toml        # 默认配置文件模板
│
├── migrations/
│   └── 001_init.sql        # 数据库初始化脚本
│
├── src/
│   ├── main.rs             # 应用程序入口
│   ├── lib.rs              # 库根模块
│   │
│   ├── agent/              # AI Agent核心
│   │   ├── mod.rs
│   │   ├── core.rs         # Agent主逻辑
│   │   ├── memory.rs       # 记忆管理（预留）
│   │   └── tools.rs        # 工具调用（预留）
│   │
│   ├── cli/                # CLI命令层
│   │   ├── mod.rs
│   │   ├── chat.rs         # 聊天命令
│   │   ├── config_cmd.rs   # 配置命令
│   │   └── sandbox_cmd.rs  # 沙箱命令
│   │
│   ├── config/             # 配置管理
│   │   ├── mod.rs
│   │   └── settings.rs     # 配置结构体和加载
│   │
│   ├── memory/             # 记忆系统
│   │   ├── mod.rs
│   │   ├── storage.rs      # PostgreSQL连接
│   │   ├── conversation.rs # 对话历史
│   │   └── knowledge.rs    # 知识库
│   │
│   ├── models/             # 模型提供商
│   │   ├── mod.rs
│   │   ├── provider.rs     # Provider trait定义
│   │   ├── openai.rs       # OpenAI实现
│   │   ├── anthropic.rs    # Anthropic实现
│   │   ├── azure.rs        # Azure实现
│   │   └── ollama.rs       # Ollama实现
│   │
│   ├── sandbox/            # 沙箱系统
│   │   ├── mod.rs
│   │   ├── windows_sandbox.rs # Windows Sandbox封装
│   │   ├── executor.rs     # 命令执行器
│   │   └── security.rs     # 安全检查器
│   │
│   └── utils/              # 工具函数
│       ├── mod.rs
│       └── helpers.rs
│
└── tests/
    └── integration_test.rs # 集成测试
```

## 技术栈

### 核心依赖
- **tokio**: 异步运行时
- **clap**: CLI框架
- **reqwest**: HTTP客户端
- **sqlx**: 异步PostgreSQL驱动
- **serde**: 序列化/反序列化
- **tracing**: 结构化日志

### 关键特性
- 完全异步设计
- 类型安全的API调用
- 错误传播和处理
- 环境变量支持
- 热重载配置

## 编译和运行

### 前置要求
1. Rust MSVC工具链
2. Visual Studio Build Tools（含C++工作负载）
3. （可选）PostgreSQL 14+
4. （可选）Docker Desktop

### 编译步骤
```bash
# 切换到MSVC工具链
rustup default stable-x86_64-pc-windows-msvc

# 编译发布版本
cargo build --release

# 或使用安装脚本
.\setup.ps1
```

### 运行
```bash
# 启动聊天
.\target\release\winsage.exe chat

# 配置
.\target\release\winsage.exe config

# 查看沙箱状态
.\target\release\winsage.exe sandbox
```

## 架构设计亮点

### 1. Provider模式
通过`ModelProvider` trait实现模型无关性，新增模型只需实现该trait：

```rust
#[async_trait]
pub trait ModelProvider: Send + Sync {
    fn name(&self) -> &str;
    async fn chat(&self, messages: Vec<Message>) -> Result<ModelResponse, ModelError>;
}
```

### 2. 分层安全
- **L1**: 正则表达式匹配危险命令
- **L2**: Windows Sandbox隔离执行
- **L3**: 用户确认机制

### 3. 记忆系统设计
- 对话级别的消息组织
- 支持消息元数据（tokens、模型名称）
- JSONB字段存储灵活的用户偏好
- 预留向量搜索接口

### 4. 配置优先级
1. 环境变量（最高优先级）
2. 配置文件
3. 默认值

## 安全措施

1. **API密钥保护**
   - 支持环境变量注入
   - 不硬编码在代码中
   - .gitignore排除.env文件

2. **命令执行安全**
   - 默认启用沙箱
   - 危险命令自动拦截
   - 超时限制（可配置）

3. **数据库安全**
   - SQLx预编译语句防止SQL注入
   - 最小权限原则
   - 连接池管理

## 性能考虑

1. **异步I/O**: 所有网络和数据库操作都是异步的
2. **连接池**: PostgreSQL使用连接池复用连接
3. **流式响应**: 支持模型流式输出（预留接口）
4. **内存管理**: Rust所有权系统保证零拷贝

## 扩展方向

### 短期改进
1. 添加更多模型提供商（Google Gemini、Mistral等）
2. 实现真正的流式聊天UI
3. 添加语音输入/输出
4. 完善单元测试覆盖

### 中期目标
1. 插件系统（自定义工具）
2. 图形界面（Tauri桌面应用）
3. 团队协作功能
4. 工作流自动化

### 长期愿景
1. 多模态支持（图像、音频）
2. 本地LLM推理优化
3. 分布式记忆系统
4. 企业级部署方案

## 已知限制

1. **Windows Sandbox**: 需要Windows 10/11专业版或企业版
2. **编译依赖**: 需要Visual Studio Build Tools（约6GB）
3. **PostgreSQL**: 需要单独安装或使用Docker
4. **本地模型**: 当前仅支持通过Ollama间接使用

## 故障排除

### 常见问题

**Q: 编译时link.exe失败**
A: 确保安装了Visual Studio Build Tools和C++工作负载

**Q: Windows Sandbox不可用**
A: 检查Windows版本并启用虚拟化功能

**Q: 数据库连接超时**
A: 确认PostgreSQL正在运行且防火墙允许5432端口

**Q: API调用失败**
A: 检查API密钥是否正确，网络是否通畅

## 贡献指南

欢迎提交Issue和Pull Request！

### 代码规范
- 遵循Rust官方风格指南
- 所有公共API必须有文档注释
- 新功能必须包含测试
- 使用`cargo fmt`和`cargo clippy`

### 提交流程
1. Fork仓库
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建Pull Request

## 许可证

MIT License - 详见LICENSE文件

## 致谢

感谢以下开源项目：
- Rust编程语言
- Tokio异步运行时
- SQLx数据库库
- Clap命令行框架
- 所有AI模型提供商

---

**开发者**: WinSage Team
**版本**: v0.1.0
**最后更新**: 2026-05-04
