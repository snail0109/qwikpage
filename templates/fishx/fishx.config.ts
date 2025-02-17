import {
  DvaPlugin,
  LocalePlugin,
} from 'fishx/plugins';
import { defineConfig } from 'fishx/defineConfig';

export default defineConfig({
  title: 'fishx',
  plugins: [
    new DvaPlugin(),
    new LocalePlugin(),
  ],
  router: {
    history: 'browser',
  },
  proxy: {},
  // 是否开启缓存, 默认开启，存放目录 node_modules/.cache 体积较大时可以关闭或者手动清理
  // cache: false,
});
