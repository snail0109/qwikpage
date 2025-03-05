@echo off

REM 检查 Node.js 版本，要求最低版本为 18
echo 检查 Node.js 版本
for /f "tokens=2 delims=v" %%i in ('node -v') do set node_version=%%i
for /f "tokens=1 delims=." %%i in ("%node_version%") do set major_version=%%i

if %major_version% lss 18 (
    echo Node.js 版本必须 >= 18，当前版本为 v%node_version%
    exit /b 1
)
REM 检查 npm 是否安装
echo 检查 npm
where npm >nul 2>nul
if %errorlevel% neq 0 (
    echo npm 未安装，请安装 npm
    exit /b 1
) else (
    echo 安装依赖
    npm install
    if %errorlevel% neq 0 (
        echo npm install 失败
        exit /b 1
    ) else (
        echo npm install 成功
    )
)
REM 启动项目
echo 启动项目
npm run dev
if %errorlevel% neq 0 (
    echo 项目启动失败
    exit /b 1
) else (
    echo 项目启动成功
)

