pub const BUTTON_INDEX: &str = r#"
import { defineComponent, ref } from 'vue';
import { Button as AButton } from 'ant-design-vue';
import { withInstall } from '@/utils/type';
import { commonProps } from '@/types';

const Button = defineComponent({
  name: 'QButton',
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, expose }) {
    const visible = ref(true);
    const disabled = ref(false);
    const loading = ref(false);

    const handleClick = () => {
      props.onClick?.();
    }

    const show = () => {
      visible.value = true;
    }
    const hide = () => {
      visible.value = false;
    }
    const enable = () => {
      disabled.value = false;
    }

    const disable = () => {
      disabled.value = true;
    }

    const startLoading = () => {
      loading.value = true;
    }

    const hideLoading = () => {
      loading.value = false;
    }

    expose({ show, hide, enable, disable, startLoading, hideLoading })

    const renderIcon = () => {
      if (props.config.props.icon) {
        return (
          <q-base-icon
            icon={props.config.props.icon}
          />
        )
      }
      return null;
    }

    return () => {
      const id = props.id;
      const config = props.config;
      return visible.value && (
        <AButton
          {...attrs}
          id={id}
          style={config.style}
          loading={loading.value}
          disabled={disabled.value}
          {...config.props}
          icon={renderIcon()}
          onClick={handleClick}
        >
          {config.props.text}
        </AButton>
      )
    };
  }
});

export default withInstall(Button);
"#;