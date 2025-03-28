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

#[derive(Debug)]
pub struct FileTemplate {
    pub filename: String,
    pub content: String,
}

pub fn template_files() -> [FileTemplate; 8] {
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
    ]
}