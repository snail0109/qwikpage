
pub const IMG_INDEX: &str = r#"
import { defineComponent, ref } from "vue";
import { Image as AImage } from "ant-design-vue";
import { withInstall } from "@/utils/type";
import { commonProps } from "@/types";
import { omit } from 'lodash-es';

const BASE = import.meta.env.BASE_URL || '/';
const handleImage = (src: string) => {
  if (src.startsWith('http')) {
    const arr = src.split('/');
    return `${BASE}img/defaultGroup/${arr[arr.length - 1]}`
  }
  return `${BASE}img/defaultGroup/${src}`
}
const Image = defineComponent({
  name: "QImage",
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, expose }) {
    const { onClick } = attrs;
    const visible = ref(true);
    const handleClick = () => {
      onClick?.();
    }

    // 暴露方法
    expose({
      show: () => (visible.value = true),
      hide: () => (visible.value = false),
    });
    const realSrc = handleImage(props.config.props.src);

    return () => (
      visible.value && (
        <AImage
          style={props.config.style}
          {...omit(props.config.props, ['formItem'])}
          src={realSrc}
          onClick={handleClick}
        />
      )
    );
  },
});

export default withInstall(Image);
"#;