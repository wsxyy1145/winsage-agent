# WinSage 演示脚本
# 此脚本展示WinSage的基本功能

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "   WinSage AI Agent - 功能演示" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# 检查可执行文件是否存在
$exePath = ".\target\release\winsage.exe"
if (-not (Test-Path $exePath)) {
    Write-Host "错误: 未找到编译后的可执行文件" -ForegroundColor Red
    Write-Host "请先运行: cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host "1. 查看版本信息" -ForegroundColor Green
& $exePath --help

Write-Host "`n2. 查看沙箱状态" -ForegroundColor Green
& $exePath sandbox

Write-Host "`n3. 配置文件位置" -ForegroundColor Green
Write-Host "   配置目录: $env:APPDATA\winsage" -ForegroundColor Gray

Write-Host "`n4. 可用功能演示" -ForegroundColor Green
Write-Host "   ✓ OpenAI GPT 集成" -ForegroundColor White
Write-Host "   ✓ Anthropic Claude 集成" -ForegroundColor White
Write-Host "   ✓ Azure OpenAI 集成" -ForegroundColor White
Write-Host "   ✓ Ollama 本地模型集成" -ForegroundColor White
Write-Host "   ✓ Windows Sandbox 安全隔离" -ForegroundColor White
Write-Host "   ✓ PostgreSQL 记忆系统" -ForegroundColor White
Write-Host "   ✓ 交互式 CLI 界面" -ForegroundColor White

Write-Host "`n5. 快速开始" -ForegroundColor Green
Write-Host "   步骤 1: 配置 API 密钥" -ForegroundColor Cyan
Write-Host "           & $exePath config" -ForegroundColor Gray
Write-Host "   步骤 2: 启动聊天" -ForegroundColor Cyan
Write-Host "           & $exePath chat" -ForegroundColor Gray

Write-Host "`n6. 示例对话场景" -ForegroundColor Green
Write-Host "   场景 A: 日常问答" -ForegroundColor Yellow
Write-Host "   你: 请解释什么是量子计算？" -ForegroundColor Gray
Write-Host "   AI: [详细解释量子计算原理...]" -ForegroundColor Gray

Write-Host "`n   场景 B: 代码生成" -ForegroundColor Yellow
Write-Host "   你: 用Python写一个快速排序算法" -ForegroundColor Gray
Write-Host "   AI: [生成完整的Python代码...]" -ForegroundColor Gray

Write-Host "`n   场景 C: 系统命令（沙箱保护）" -ForegroundColor Yellow
Write-Host "   你: 列出C盘根目录的文件" -ForegroundColor Gray
Write-Host "   AI: [在沙箱中安全执行 dir C:\ ...]" -ForegroundColor Gray

Write-Host "`n7. 环境变量使用" -ForegroundColor Green
Write-Host "   PowerShell:" -ForegroundColor Cyan
Write-Host '   $env:WINSAGE_MODELS_OPENAI_API_KEY = "sk-your-key"' -ForegroundColor Gray
Write-Host "   CMD:" -ForegroundColor Cyan
Write-Host '   set WINSAGE_MODELS_OPENAI_API_KEY=sk-your-key' -ForegroundColor Gray

Write-Host "`n8. Docker 快速部署 PostgreSQL" -ForegroundColor Green
Write-Host "   docker-compose up -d" -ForegroundColor Gray

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "   准备就绪！开始你的 AI 之旅吧！" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

Write-Host "提示: 阅读以下文档了解更多信息" -ForegroundColor Yellow
Write-Host "  - QUICKSTART.md     : 5分钟快速上手" -ForegroundColor Gray
Write-Host "  - README.md         : 完整项目说明" -ForegroundColor Gray
Write-Host "  - EXAMPLES.md       : 详细使用示例" -ForegroundColor Gray
Write-Host "  - PROJECT_SUMMARY.md: 架构设计文档`n" -ForegroundColor Gray
