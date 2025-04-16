pub const SWITCH_INDEX: &str = r#"
import { defineComponent, ref, inject, watch } from "vue";
import { Switch as ASwitch, FormItem as AFormItem } from "ant-design-vue";
import type { FormItemProps, SwitchProps } from "ant-design-vue";
import { withInstall, defaultFormContext } from "@/utils/type";
import { commonProps } from "@/types";
import type { UseFormContextType } from "@/types";
import { isObject, isArray } from "lodash-es";

const Switch = defineComponent({
  name: "QSwitch",
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, expose }: any) {
    const { onChange, ...rest } = attrs;
    const useFormContext = inject<UseFormContextType>("useFormContext", () =>
      defaultFormContext()
    );
    const { initValues, getValue: getItemValue } = useFormContext();

    const visible = ref(true);
    const disabled = ref(false);
    const checked = ref(props.config.props.defaultValue);

    // 初始化默认值
    watch(
      () => props.config.props.defaultValue,
      (newVal) => {
        const name: string = String(
          props.config.props.formItem?.name || props.id
        );
        checked.value = newVal;
        initValues(props.type, name, newVal);
      },
      { immediate: true }
    );

    // 启用和禁用
    watch(
      () => props.config.props.formWrap.disabled,
      (val) => {
        if (typeof val === "boolean") {
          disabled.value = val;
        }
      }
    );

    // 输入事件
    const handleChange = (val: boolean) => {
      const name = props.config.props.formItem?.name || props.id;
      checked.value = val;
      initValues(props.type, name, val);
      onChange?.({ [name]: val });
    };

    const show = () => {
      visible.value = true;
    };
    const hide = () => {
      visible.value = false;
    };
    const enable = () => {
      disabled.value = false;
    };
    const disable = () => {
      disabled.value = true;
    };

    const setValue = (value: any) => {
      const name = props.config.props.formItem?.name || props.id;
      if (isObject(value) && name in value) {
        checked.value = value[name as keyof typeof value];
        initValues(props.type, name, value[name as keyof typeof value]);
      } else if (isArray(value)) {
        checked.value = value;
        initValues(props.type, name, value);
      } else {
        console.error("[switch]", "setValue参数错误，请检查", value);
      }
    };

    const getValue = () => {
      const name = props.config.props.formItem?.name || props.id;
      return getItemValue(String(name));
    };

    expose({ show, hide, enable, disable, setValue, getValue });

    return () =>
      visible.value && (
        <AFormItem 
          {...props.config.props.formItem} 
          valuePropName="checked"
        >
          <ASwitch
            {...rest}
            {...props.config.props.formWrap}
            disabled={disabled.value}
            style={props.config.style}
            checked={checked.value}
            onChange={(val: boolean) => handleChange(val)}
          ></ASwitch>
        </AFormItem>
      );
  },
});

export default withInstall(Switch);
"#;