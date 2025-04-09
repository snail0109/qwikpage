import { defineConfig } from 'father';

/**
 * 导出默认的构建配置
 *
 * 使用 `defineConfig` 方法定义项目的构建配置。
 * 该配置主要用于指定输出格式、目标平台等构建相关选项。
 */
export default defineConfig({
  esm: {
    // 所有生成的 ESM 文件将被输出到 `dist` 目录中。
    output: 'dist',
  },
  // 设置为 `browser`，表示构建的目标环境是浏览器。
  platform: 'browser',
});
