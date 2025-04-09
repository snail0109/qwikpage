pub const GRID_INDEX: &str = r#"
import { defineComponent, ref } from 'vue';
import { Row } from 'ant-design-vue';
import { withInstall } from '@/utils/type';
import { commonProps } from '@/types';

const Grid = defineComponent({
  name: 'QGrid',
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
      <Row
        {...attrs}
        id={props.id}
        style={props.config.style}
        {...props.config.props}
      >
        <q-render elements={props.elements || []} />
      </Row>
    );
  }
});

export default withInstall(Grid);
"#;