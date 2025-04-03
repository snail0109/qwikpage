pub const COM_INDEX: &str = r#"
import type { App } from 'vue';
import * as components from './components';

export const install = function (app: App) {
  Object.keys(components).forEach(key => {
    const component = (components as Record<string, any>)[key];
    if (component.install) {
      app.use(component);
    }
  });
  return app;
}

export default { install };
"#;