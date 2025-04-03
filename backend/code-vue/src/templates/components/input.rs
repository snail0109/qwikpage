pub const INPUT_INDEX: &str = r#"
import { defineComponent, ref, inject, watch } from 'vue';
import { Input as AInput, FormItem as AFormItem } from 'ant-design-vue';
import { withInstall, defaultFormContext } from '@/utils/type';
import omit from 'lodash-es/omit';
import type { UseFormContextType } from '@/types';
import { commonProps } from '@/types';

const Input = defineComponent({
  name: 'QInput',
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, expose }) {
    const useFormContext = inject<UseFormContextType>('useFormContext', () => defaultFormContext());
    const { initValues, getValue: getItemValue } = useFormContext();

    const visible = ref(true);
    const disabled = ref(false);

    // 初始化默认值
    watch(
      () => props.config.props.defaultValue,
      () => {
        const name: string = props.config.props.formItem?.name || props.id;
        const value = props.config.props.defaultValue;
        initValues(props.type, name, value);
      }
    )

    // 启用和禁用
    watch(
      () => props.config.props.formWrap.disabled,
      (val) => {
        if (typeof val === 'boolean') {
          disabled.value = val;
        }
      }
    )

    // 输入事件
    const handleChange = (val: string) => {
      const name = props.config.props.formItem?.name || props.id;
      initValues(props.type, name, val);
      props.onChange?.({
        [name]: val,
      });
    };

    // 失去焦点事件
    const handleBlur = (val: string) => {
      const name = props.config.props.formItem?.name || props.id;
      props.onBlur?.({
        [name]: val,
      });
    };

    // 回车事件
    const handlePressEnter = (val: string) => {
      const name = props.config.props.formItem?.name || props.id;
      props.onPressEnter?.({
        [name]: val,
      });
    };

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

    const setValue = (value: any) => {
      const name = props.config.props.formItem?.name || props.id;
      initValues(props.type, name, value);
    }

    const getValue = () => {
      const name = props.config.props.formItem?.name || props.id;
      return getItemValue(name)
    }

    expose({ show, hide, enable, disable, setValue, getValue })
    return () => visible.value && (
      <AFormItem {...props.config.props.formItem}>
        <AInput
          {...attrs}
          {...omit(props.config.props.formWrap, ['prefixIcons', 'suffixIcons'])}
          disabled={disabled.value}
          style={props.config.style}
          value={props.formItemValue}
          onChange={(event: any) => handleChange(event.target.value)}
          onBlur={(event: any) => handleBlur(event.target.value)}
          onPressEnter={(event: any) => handlePressEnter(event.target.value)}
        >
        </AInput>
      </AFormItem>
    );
  }
});

export default withInstall(Input);
"#;