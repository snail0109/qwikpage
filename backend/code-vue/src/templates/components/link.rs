pub const LINK_INDEX: &str = r#"
import { computed, defineComponent, ref, watch } from "vue";
import { withInstall } from "@/utils/type";
import { commonProps } from "@/types";
import { omit } from 'lodash-es';

const Link = defineComponent({
  name: "QLink",
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { expose }) {
    const visible = ref(true);
    const text = ref("");

    // 监听文本变化
    watch(
      () => props.config.props?.text,
      (newVal) => {
        text.value = newVal?.toString() || "超链接文本占位";
      },
      { immediate: true }
    );

    // 暴露方法
    expose({
      show: () => (visible.value = true),
      hide: () => (visible.value = false),
    });

    return () => (
      visible.value && (
        <a
          style={props.config.style}
          {...omit(props.config.props, ["script", "text", "hiddenText", 'formItem'])}
          id={props.id}
        >
          {text.value}
        </a>
      )
    );
  },
});

export default withInstall(Link);
"#;