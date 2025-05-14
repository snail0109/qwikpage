pub const FLEX_INDEX: &str = r#"
import { defineComponent, ref } from 'vue';
import { Flex as AFlex } from 'ant-design-vue';
import { withInstall } from '@/utils/type';
import { commonProps } from '@/types';

const Flex = defineComponent({
  name: 'QFlex',
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, slots, expose }) {
    const visible = ref(true);

    const show = () => {
      visible.value = true;
    }
    const hide = () => {
      visible.value = false;
    }

    expose({ show, hide })

    return () => visible.value && (
      <AFlex
        {...attrs}
        id={props.id}
        style={props.config.style}
        {...props.config.props}
      >
        {slots?.default()}
      </AFlex>
    );
  }
});

export default withInstall(Flex);
"#;