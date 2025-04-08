use crate::FileTemplate;

pub const VITE_CONFIG: &str = r#"
import { defineConfig, PluginOption } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path';
import svgr from 'vite-plugin-svgr';
export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  plugins: [
    react() as PluginOption,
    svgr({ svgrOptions: { icon: true } }),
  ],
})
"#;

pub const TS_CONFIG_NOE: &str = r#"
{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}
"#;

pub const TS_CONFIG: &str = r#"
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "Bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "baseUrl": "./",
    "paths": {
      "@/*": ["src/*"]
    },

    /* Linting */
    "strict": true,
    "noUnusedLocals": false,
    "noUnusedParameters": false,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }],
  "exclude": ["node_modules", "lib", "es", "dist", "typings", "**/__test__", "test", "docs", "tests"]
}
"#;


pub const HTML: &str = r#"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>QwikPage</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
"#;

pub const DTS: &str = r#"
/// <reference types="vite/client" />
"#;

pub const APP_REACT: &str = r#"
import { RouterProvider } from "react-router-dom";
import { ConfigProvider, App as AntdApp } from "antd";
import router from "./config/router";
import dayjs from "dayjs";
import "dayjs/locale/zh-cn";
import weekday from "dayjs/plugin/weekday";
import localeData from "dayjs/plugin/localeData";
import relativeTime from "dayjs/plugin/relativeTime";
import locale from "antd/locale/zh_CN";
dayjs.extend(relativeTime);
dayjs.extend(weekday);
dayjs.extend(localeData);
dayjs.locale("zh-cn");

function App() {
    return (
        <ConfigProvider
            locale={locale}
        >
            <AntdApp>
                <RouterProvider router={router} />
            </AntdApp>
        </ConfigProvider>
    );
}

export default App;
"#;

pub const MAIN: &str = r#"
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
"#;

pub const START_SH_CONFIG: &str = r#"
#!/bin/bash

# 检查 Node.js 版本，要求最低版本为 18
echo "检查 Node.js 版本"
node_version=$(node -v)
if [[ ! $node_version =~ ^v1[89] ]] && [[ ! $node_version =~ ^v[2-9][0-9] ]]; then
    echo "Node.js 版本必须 >= 18，当前版本为 $node_version"
    exit 1
fi

# 检查 npm 是否安装
echo "检查 npm"
if ! command -v npm &> /dev/null; then
    echo "npm 未安装，请安装 npm"
    exit 1
else
    echo "安装依赖"
    if npm install; then
        echo "npm install 成功"
    else
        echo "npm install 失败"
        exit 1
    fi
fi

# 启动项目
echo "启动项目"
if npm run dev; then
    echo "项目启动成功"
else
    echo "项目启动失败"
    exit 1
fi

"#;

pub const START_WIN_CONFIG: &str = r#"
@echo off
echo ===================================
echo React Template Project Startup Script
echo ===================================

:: Check if Node.js is installed
where node >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: Node.js not detected. Please install Node.js first.
    pause
    exit /b 1
)

:: Check if package.json exists
if not exist package.json (
    echo Error: package.json not found in current directory.
    echo Please make sure to run this script in the React Template project root directory.
    pause
    exit /b 1
)

:: Check if node_modules exists, install dependencies if not
if not exist node_modules\ (
    echo node_modules folder not detected, installing dependencies...
    echo.
    call npm install
    
    if %ERRORLEVEL% neq 0 (
        echo.
        echo Dependency installation failed. Please check your network connection or package.json file.
        pause
        exit /b 1
    )
    
    echo.
    echo Dependencies installed successfully!
) else (
    echo node_modules folder detected, skipping installation step.
)

echo.
echo Starting React Template development server...
echo Press Ctrl+C to stop the server.
echo.

:: Start React Template development server
call npm run dev

pause

"#;

pub fn template_files() -> [FileTemplate; 9] {
    [
        FileTemplate {
            filename: String::from("vite.config.ts"),
            content: String::from(VITE_CONFIG),
        },
        FileTemplate {
            filename: String::from("tsconfig.node.json"),
            content: String::from(TS_CONFIG_NOE),
        },
        FileTemplate {
            filename: String::from("tsconfig.json"),
            content: String::from(TS_CONFIG),
        },
        FileTemplate {
            filename: String::from("index.html"),
            content: String::from(HTML),
        },
        FileTemplate {
            filename: String::from("env.d.ts"),
            content: String::from(DTS),
        },
        FileTemplate {
            // 默认导入了 element-ui
            filename: String::from("src/App.tsx"),
            content: String::from(APP_REACT),
        },
        FileTemplate {
            filename: String::from("src/main.tsx"),
            content: String::from(MAIN),
        },
        FileTemplate {
          filename: String::from("start"),
          content: String::from(START_SH_CONFIG),
        },
        FileTemplate {
          filename: String::from("start.cmd"),
          content: String::from(START_WIN_CONFIG),
        },
    ]
}