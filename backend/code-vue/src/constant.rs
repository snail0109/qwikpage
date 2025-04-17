use crate::FileTemplate;

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
  server: {
    proxy: {
      {proxyInfo}
    }
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
import { provide, onMounted } from "vue";
import { storeToRefs } from "pinia";
import appStore from "@/stores";
import { getInitValue } from "@/utils/util";
import type { FormContextType, PageVariable } from "@/types";

const { pageState } = storeToRefs(appStore.page);
const { setFormItemData } = appStore.page;
const { setVariables } = appStore.project;

const initValues = (
  type: string,
  name: string | number | (string | number)[],
  value: any
) => {
  if (name) {
    const initValue = getInitValue(type, value);
    setFormItemData({
      name,
      value: initValue,
    });
  }
};

const getValue = (name: string) => {
  const formItemData = pageState.value.page.pageData.formItemData;
  const value = formItemData[name];
  console.log("普通控件取值打印 [name]: value", `[${name}]: `, value);
  return value;
};

const useFormContext = (): FormContextType => {
  return {
    initValues,
    getValue,
    inForm: "",
  };
};

provide("useFormContext", useFormContext);
onMounted(() => {
  const variables: PageVariable[] = {variablesInfo};
  setVariables(variables);
});
</script>

<template>
  <div id="root">
    <router-view></router-view>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
#root {
  color: rgb(0, 0, 0);
  background-color: rgb(255, 255, 255);
  padding: 20px;
  .ant-form-inline {
    > .ant-form-item {
      margin-bottom: 15px !important;
    }
  }
}
</style>
"#;

pub const MAIN: &str = r#"

import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import Antd from 'ant-design-vue';
import { registerStore } from '@/stores';
import 'ant-design-vue/dist/reset.css';
import router from './router';
import { install } from '@/components';

const app = createApp(App);
app.use(Antd);
app.use(router);

// 注册pinia store
app.use(createPinia());
registerStore();

// 注册components组件
install(app);
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
echo ===================================
echo Vue Template Project Startup Script
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
    echo Please make sure to run this script in the Vue Template project root directory.
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
echo Starting Vue Template development server...
echo Press Ctrl+C to stop the server.
echo.

:: Start Vue Template development server
call npm run dev

pause

"#;

pub const ENV_CONFIG: &str = r#"
#接口地址
VITE_BASE_API=/qwikpageApi
"#;

pub fn template_config_file() -> FileTemplate {
    FileTemplate {
        filename: String::from("vite.config.ts"),
        content: String::from(VITE_CONFIG),
    }
}

pub fn template_app_file() -> FileTemplate {
    FileTemplate {
        filename: String::from("src/App.vue"),
        content: String::from(APP_VUE),
    }
}

pub fn template_files() -> [FileTemplate; 9] {
    [
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
        FileTemplate {
            filename: String::from(".env"),
            content: String::from(ENV_CONFIG),
        },
    ]
}
