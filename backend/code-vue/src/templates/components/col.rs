pub const COL_INDEX: &str = r#"
import { defineComponent, ref } from 'vue';
import { Col as ACol } from 'ant-design-vue';
import { withInstall } from '@/utils/type';
import { commonProps } from '@/types';

const Col = defineComponent({
  name: 'QCol',
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, expose }) {
    const visible = ref(true);

    const show = () => {
      visible.value = true;
    }
    const hide = () => {
      visible.value = false;
    }

    expose({ show, hide })

    return () => visible.value && (
      <ACol
        {...attrs}
        id={props.id}
        style={props.config.style}
        {...props.config.props}
      >
        <q-render elements={props.elements || []} />
      </ACol>
    );
  }
});

export default withInstall(Col);
"#;