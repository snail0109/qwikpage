# qwikpage-app

用于存放新的支持自由布局、支持代码导出的界面设计器工具的预研代码

## 前置要求
根据自己的电脑系统参考官方文档安装配置 [前置要求 | Tauri](https://v2.tauri.app/start/prerequisites/)

### MAC
1. 安装 Xcode
`xcode-select --install`

2. 安装 Rust
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Windows 
1. 安装 Microsoft C++ 生成工具
下载 [Microsoft C++](https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/) 生成工具 安装程序并打开它以开始安装。
在安装过程中，选中“使用 C++ 的桌面开发”选项。
![](https://v2.tauri.app/_astro/visual-studio-build-tools-installer.TFOm5FVI_RDwYY.webp)


2. webview2  
WebView 2 已安装在 Windows 10（从版本 1803 开始）和更高版本的 Windows 上。如果你正在这些版本之一上进行开发，则可以跳过此步骤，并直接转到安装 Rust。

Tauri 使用 Microsoft Edge WebView2 在 Windows 上呈现内容。

通过访问 [下载 WebView2 运行时](https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/#download) 安装 WebView2。下载并安装“常青独立安装程序（Evergreen Bootstrapper）”。

3. 安装 Rust  
[64位下载执行](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)  
[32位下载执行](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)


### 安装 node 
1.访问 [ Node.js](https://nodejs.org/zh-cn) 网站，下载并安装长期支持版本（LTS）。
node 最低使用 node 18

2.运行以下命令以检查 Node 是否成功安装：

```bash
node -v 
# v20.10.0
npm -v
# 10.2.3
```

### 安装 pnpm
`npm i pnpm -g`

### 开发
```bash
pnpm install # 安装依赖
pnpm dev  # 启动项目
```

### 本地打包
Mac  
```bash
pnpm install # 安装依赖
pnpm run build:mac
```

Windows   
```bash  
pnpm install # 安装依赖
pnpm run build:windows # 生成 .exe 文件
```

### 生成秘钥
`pnpm tauri signer generate -w ./keys/myapp.key`

### 设置秘钥打包
```bash
export TAURI_SIGNING_PRIVATE_KEY=
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=
./scripts/release.sh --channel nightly --version "0.0.3"
```