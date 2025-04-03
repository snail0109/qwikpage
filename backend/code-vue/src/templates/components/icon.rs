pub const ICON_INDEX: &str = r#"
import { defineComponent, ref } from 'vue';
import { withInstall } from '@/utils/type';
import { commonProps } from '@/types';

const Icon = defineComponent({
  name: 'QIcon',
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, expose }) {
    const visible = ref(true);

    const handleClick = () => {
      props.onClick?.();
    }

    const show = () => {
      visible.value = true;
    }
    const hide = () => {
      visible.value = false;
    }

    expose({ show, hide })

    return () => {
      const config = props.config;
      return visible.value && (
        <q-base-icon
          {...attrs}
          style={config.style}
          {...config.props}
          icon={config.props.icon}
          onClick={handleClick}
        />
      )
    };
  }
});

export default withInstall(Icon);
"#;

pub const ICON_BASE_INDEX: &str = r#"
import { defineComponent } from 'vue';
import * as icons from '@qwikpage/icons';
import { renderIconDefinitionToSVGElement } from '@qwikpage/icons/es/helpers';
import { withInstall } from '@/utils/type';

const iconsList: { [key: string]: any } = icons;

const BaseIcon = defineComponent({
  name: 'QBaseIcon',
  inheritAttrs: false,
  props: {
    icon: { type: String, default: '' }
  },
  setup(props, { attrs }) {

    return () => {
      if (!props.icon || !iconsList[props.icon]) {
        return null;
      }
      const svgHTMLString = renderIconDefinitionToSVGElement(iconsList[props.icon], {
        extraSVGAttrs: { width: '1em', height: '1em', fill: 'currentColor' },
      })
      return (
        <span
          {...attrs}
          class='anticon'
          v-html={svgHTMLString}
        />
      )
    };
  }
});

export default withInstall(BaseIcon);
"#;