pub const VITE_CONFIG: &str = r#"
import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})

"#;

pub const TS_CONFIG_NOE: &str = r#"
{
  "extends": "@tsconfig/node22/tsconfig.json",
  "include": [
    "vite.config.*",
    "vitest.config.*",
    "cypress.config.*",
    "nightwatch.conf.*",
    "playwright.config.*",
    "eslint.config.*"
  ],
  "compilerOptions": {
    "noEmit": true,
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.node.tsbuildinfo",

    "module": "ESNext",
    "moduleResolution": "Bundler",
    "types": ["node"]
  }
}
"#;

pub const TS_CONFIG: &str = r#"
{
  "files": [],
  "references": [
    {
      "path": "./tsconfig.node.json"
    },
    {
      "path": "./tsconfig.app.json"
    }
  ]
}
"#;

pub const TS_CONFIG_APP: &str = r#"
{
  "extends": "@vue/tsconfig/tsconfig.dom.json",
  "include": ["env.d.ts", "src/**/*", "src/**/*.vue"],
  "exclude": ["src/**/__tests__/*"],
  "compilerOptions": {
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.app.tsbuildinfo",

    "paths": {
      "@/*": ["./src/*"]
    }
  }
}

"#;

pub const HTML: &str = r#"
<!DOCTYPE html>
<html lang="">
  <head>
    <meta charset="UTF-8">
    <link rel="icon" href="/favicon.ico">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Vite App</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
"#;

pub const DTS: &str = r#"
/// <reference types="vite/client" />
"#;

pub const APP_VUE: &str = r#"
<script setup lang="ts">
</script>

<template>
  <div id="root">
    <router-view></router-view>
  </div>
</template>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
#root {
  color: rgb(0, 0, 0);
  background-color: rgb(255, 255, 255);
  padding: 20px;
}
</style>
"#;

pub const MAIN: &str = r#"
import { createApp } from 'vue';
import App from './App.vue';
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';
import router from './router';

const app = createApp(App);
app.use(Antd);
app.use(router);
app.mount('#app');
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

"#;

#[derive(Debug)]
pub struct FileTemplate {
    pub filename: String,
    pub content: String,
}

pub fn template_files() -> [FileTemplate; 10] {
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
            filename: String::from("tsconfig.app.json"),
            content: String::from(TS_CONFIG_APP),
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
            filename: String::from("src/App.vue"),
            content: String::from(APP_VUE),
        },
        FileTemplate {
            filename: String::from("src/main.ts"),
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