pub const CARD_INDEX: &str = r#"

import { defineComponent, ref, computed } from "vue";
import { Card as ACard, Button as AButton, Avatar as AAvatar } from "ant-design-vue";
import { withInstall } from "@/utils/type";
import { handleActionFlow } from "@/utils/action";
import { omit } from "lodash-es";
import { commonProps } from "@/types";

const Card = defineComponent({
  name: "QCard",
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, slots, expose }: any) {
    const { onClick, ...rest } = attrs;
    const visible = ref(true);
    const bulkActionList = computed(() => props.config.props.bulkActionList || []);
    const meta = computed(() => props.config.props.meta);
    const avatar = computed(() => props.config.props.avatar || undefined);

    // 处理meta值
    const parseMetaValue = (value: any) => {
      if (typeof value === 'object' && value !== null && 'value' in value) {
        return value.value;
      }
      return value;
    };

    const processedMeta = computed(() => {
      if (!meta.value) return null;
      return {
        ...meta.value,
        title: parseMetaValue(meta.value.title),
        description: parseMetaValue(meta.value.description)
      };
    });

    const handleOperate = (eventName: string) => {
      const btnEvent = props.config.events.find((event: any) => event.eventName === eventName);
      handleActionFlow(btnEvent?.actions, {});
    };

    const handleClick = () => {
      onClick?.();
    }

    const show = () => {
      visible.value = true;
    };

    const hide = () => {
      visible.value = false;
    };

    expose({ show, hide });

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

    return () =>
      visible.value && (
        <ACard
          {...rest}
          style={props.config.style}
          {...omit(props.config.props, ['cover', 'meta', 'title'])}
          id={props.id}
          {...(props.config.props.header ? { title: props.config.props.title } : {})}
          cover={props.config.props.cover ? <img src={props.config.props.cover} /> : null}
          extra={
            props.config.props.header && (
              <div style={{ display: 'flex', gap: '10px' }}>
                {bulkActionList.value.map((item: any, index: number) => {
                  return (
                    <AButton
                      key={item.eventName}
                      type={item.type}
                      danger={item.danger}
                      icon={renderIcon()}
                      onClick={() => handleOperate(item.eventName)}
                    >
                      {item.text}
                    </AButton>
                  );
                })}
              </div>
            )
          }
          onClick={handleClick}
        >
          {props.config.props.showmeta && (processedMeta.value?.title || processedMeta.value?.description) ? (
            <ACard.Meta 
              {...processedMeta.value} 
              avatar={avatar.value && <AAvatar src={avatar.value} />} 
            />
          ) : null}
          {slots?.default()}
        </ACard>
      );
  },
});

export default withInstall(Card);
"#;