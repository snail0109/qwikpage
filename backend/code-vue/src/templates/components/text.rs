pub const TEXT_INDEX: &str = r#"
import { computed, defineComponent, ref, watch } from "vue";
import { Typography } from "ant-design-vue";
import { withInstall } from "@/utils/type";
import { commonProps } from "@/types";
import { omit } from 'lodash-es';

const Text = defineComponent({
  name: "QText",
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, expose }) {
    const { onClick, ...rest } = attrs;
    const visible = ref(true);
    const text = ref("");

    const handleClick = () => {
      onClick?.();
    }
    // 监听文本变化
    watch(
      () => props.config.props?.text,
      (newVal) => {
        text.value = newVal?.toString() || "";
      },
      { immediate: true }
    );

    // 计算文本样式
    const textStyle = computed(() => {
      const style = { ...props.config.style };
      const hiddenText = props.config.props?.hiddenText;

      switch (hiddenText) {
        case "ellipsis":
          return {
            ...style,
            whiteSpace: "nowrap",
            overflow: "hidden",
            textOverflow: "ellipsis",
            display: "block",
          };
        case "break":
          return {
            ...style,
            whiteSpace: "break-spaces",
            wordBreak: "break-all",
          };
        case "wrap":
          return {
            ...style,
            whiteSpace: "pre-wrap",
            wordBreak: "normal",
          };
        case "nowrap":
          return {
            ...style,
            whiteSpace: "nowrap",
          };
        default:
          return style;
      }
    });

    // 暴露方法
    expose({
      show: () => (visible.value = true),
      hide: () => (visible.value = false),
      setValue: (val) => (text.value = val),
      getValue: () => text.value,
    });

    return () => (
      visible.value && (
        <Typography.Text
          {...rest}
          style={textStyle.value}
          {...omit(props.config.props, ["script", "text", "hiddenText", 'formItem'])}
          onClick={handleClick}
        >
          {text.value}
        </Typography.Text>
      )
    );
  },
});

export default withInstall(Text);
"#;