# WinSage 安装脚本

Write-Host "=== WinSage 安装向导 ===" -ForegroundColor Green
Write-Host ""

# 检查Rust是否安装
Write-Host "检查 Rust 工具链..." -ForegroundColor Cyan
$rustInstalled = Get-Command rustc -ErrorAction SilentlyContinue

if (-not $rustInstalled) {
    Write-Host "Rust 未安装，正在安装..." -ForegroundColor Yellow
    winget install Rustlang.Rust.MSVC
} else {
    Write-Host "Rust 已安装: $(rustc --version)" -ForegroundColor Green
}

# 检查Visual Studio Build Tools
Write-Host "`n检查 Visual Studio Build Tools..." -ForegroundColor Cyan
$vsWhere = Get-Command vswhere.exe -ErrorAction SilentlyContinue

if ($vsWhere) {
    $cppTools = & vswhere.exe -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property displayName
    if ($cppTools) {
        Write-Host "C++ Build Tools 已安装" -ForegroundColor Green
    } else {
        Write-Host "警告: 未检测到 C++ Build Tools" -ForegroundColor Yellow
        Write-Host "请从以下地址下载并安装:" -ForegroundColor Yellow
        Write-Host "https://visualstudio.microsoft.com/visual-cpp-build-tools/" -ForegroundColor Yellow
    }
} else {
    Write-Host "警告: 未检测到 Visual Studio Build Tools" -ForegroundColor Yellow
    Write-Host "编译需要安装 Build Tools，请下载:" -ForegroundColor Yellow
    Write-Host "https://visualstudio.microsoft.com/visual-cpp-build-tools/" -ForegroundColor Yellow
}

# 检查Docker（用于PostgreSQL）
Write-Host "`n检查 Docker..." -ForegroundColor Cyan
$dockerInstalled = Get-Command docker -ErrorAction SilentlyContinue

if ($dockerInstalled) {
    Write-Host "Docker 已安装" -ForegroundColor Green

    # 询问是否启动PostgreSQL
    $startPostgres = Read-Host "`n是否启动 PostgreSQL 容器? (y/n)"
    if ($startPostgres -eq 'y' -or $startPostgres -eq 'Y') {
        Write-Host "启动 PostgreSQL..." -ForegroundColor Cyan
        docker-compose up -d
        Write-Host "PostgreSQL 已启动!" -ForegroundColor Green
        Write-Host "连接字符串: postgresql://postgres:postgres@localhost/winsage" -ForegroundColor Gray
    }
} else {
    Write-Host "Docker 未安装" -ForegroundColor Yellow
    Write-Host "如需使用记忆功能，请安装 Docker 或直接安装 PostgreSQL" -ForegroundColor Yellow
}

# 编译项目
Write-Host "`n编译 WinSage..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n编译成功!" -ForegroundColor Green
    Write-Host "可执行文件位置: target\release\winsage.exe" -ForegroundColor Gray
    Write-Host "`n运行命令: .\target\release\winsage.exe" -ForegroundColor Cyan
} else {
    Write-Host "`n编译失败" -ForegroundColor Red
    Write-Host "请确保已安装 Visual Studio Build Tools 和 C++ 工作负载" -ForegroundColor Yellow
}

Write-Host "`n=== 安装完成 ===" -ForegroundColor Green
