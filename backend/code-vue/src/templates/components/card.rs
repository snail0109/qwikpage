pub const CARD_INDEX: &str = r#"
import { defineComponent, ref, computed } from 'vue';
import { Card as ACard, Button } from 'ant-design-vue';
import { omit } from 'lodash-es';
import { withInstall } from '@/utils/type';

const Card = defineComponent({
  name: 'MCard',
  props: {
    config: {
      type: Object,
      required: true,
    },
    elements: {
      type: Array,
      default: () => [],
    },
    onClick: {
      type: Function,
      default: () => {},
    },
  },
  setup(props, { expose }) {
    const visible = ref(true);

    // 对外暴露方法
    expose({
      show: () => (visible.value = true),
      hide: () => (visible.value = false),
    });

    // 处理 meta 数据
    const meta = computed(() => props.config.props.meta);

    // 处理点击事件
    const handleClick = () => {
      props.onClick?.();
    };

    return () => (
      visible.value && (
        <ACard
          style={props.config.style}
          {...omit(props.config.props, ['cover', 'meta', 'extra'])}
          cover={
            props.config.props.cover ? (
              <img src={props.config.props.cover} alt="cover" />
            ) : null
          }
          extra={
            props.config.props.extra?.text ? (
              <Button
                {...props.config.props.extra}
                onClick={handleClick}
              >
                {props.config.props.extra.text}
              </Button>
            ) : null
          }
        >
          {(meta.value.title || meta.value.description) && (
            <Card.Meta {...meta.value} />
          )}
          <q-render elements={props.elements || []} />
        </ACard>
      )
    );
  },
});

export default withInstall(Card);
"#;