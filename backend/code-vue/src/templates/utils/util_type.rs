pub const TYPE_INDEX: &str = r#"
import type { App, Plugin, Component } from 'vue';
import type { FormContextType } from '@/types';

// 注册组件
export const withInstall = <T>(comp: Component) => {
  const c = comp as any;
  c.install = function (app: App) {
    app.component(c.displayName || c.name, comp);
  };

  return comp as T & Plugin;
};

export const defaultFormContext = (): FormContextType => ({
  initValues: () => { },
  getValue: () => undefined,
  inForm: false
});
"#;