pub const SELECT_INDEX: &str = r#"
import { defineComponent, ref, inject, watch } from "vue";
import { Select as ASelect, FormItem as AFormItem } from "ant-design-vue";
import type { FormItemProps, SelectProps } from "ant-design-vue";
import { withInstall, defaultFormContext } from "@/utils/type";
import { handleApi } from "@/utils/handleApi";
import { isNotEmpty } from "@/utils/util";
import { storeToRefs } from "pinia";
import appStore from "@/stores";
import { commonProps } from "@/types";
import type { UseFormContextType } from "@/types";
import { isObject, isArray } from "lodash-es";

export interface IConfig {
  defaultValue: string;
  formItem: FormItemProps;
  formWrap: SelectProps;
  field: {
    label: string;
    value: string;
  };
  source: Array<{ label: string; value: any }>;
}

const Select = defineComponent({
  name: "QSelect",
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, expose }: any) {
    const { onChange, ...rest } = attrs;
    const useFormContext = inject<UseFormContextType>("useFormContext", () =>
      defaultFormContext()
    );
    const { initValues, getValue: getItemValue } = useFormContext();

    const { pageState } = storeToRefs(appStore.page);

    const data = ref<Array<{ label: string; value: string }>>([]);
    const visible = ref(true);
    const disabled = ref(false);

    // 列表加载
    const getDataList = (opts: any) => {
      handleApi(props.config.api, opts).then((res) => {
        if (res?.code === 0) {
          if (!Array.isArray(res.data)) {
            console.error("[checkbox]", "data数据格式错误，请检查");
            data.value = [];
          } else {
            // 判断是否需要做数据转换
            let options = [];
            if (
              props.config.props.field.label === "label" &&
              props.config.props.field.value === "value"
            ) {
              options = res.data;
              if (
                typeof res.data[0] === "string" ||
                typeof res.data[0] === "number"
              ) {
                options = res.data.map((item: string | number) => {
                  return { label: item, value: item };
                });
              }
            } else {
              options = res.data.map((item: any) => {
                const label = item[props.config.props.field.label || "label"];
                const value = item[props.config.props.field.value || "value"];
                return {
                  label: isNotEmpty(label) ? label : "-",
                  value: isNotEmpty(value) ? value : "",
                };
              });
            }
            data.value = options;
          }
        }
      });
    };

    // 初始化默认值
    watch(
      () => props.config.props.defaultValue,
      (newVal) => {
        const name: string = String(
          props.config.props.formItem?.name || props.id
        );
        let value = newVal;
        // 处理被引号包裹的字符串
        if (typeof value === "string") {
          // 去除外层的单引号或双引号
          if (
            (value.startsWith('"') && value.endsWith('"')) ||
            (value.startsWith("'") && value.endsWith("'"))
          ) {
            value = value.slice(1, -1);
          } else {
            if (value && !isNaN(Number(value))) {
              value = Number(value);
            }
          }
        }

        initValues(props.type, name, value);
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

    // 计算数据源
    watch(
      () => [props.config.api, pageState.value.page.pageData.variableData],
      () => {
        getDataList({});
      },
      { immediate: true }
    );

    // 输入事件
    const handleChange = (val: any[]) => {
      const name = props.config.props.formItem?.name || props.id;
      initValues(props.type, name, val);
      onChange?.(val);
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

    const update = (data: any) => {
      // 重新加载表格数据
      getDataList(data);
    };

    const setValue = (value: any) => {
      const name = props.config.props.formItem?.name || props.id;
      if (isObject(value) && name in value) {
        initValues(props.type, name, value[name as keyof typeof value]);
      } else if (isArray(value)) {
        initValues(props.type, name, value);
      } else {
        console.error("[checkbox]", "setValue参数错误，请检查", value);
      }
    };

    const getValue = () => {
      const name = props.config.props.formItem?.name || props.id;
      return getItemValue(String(name));
    };

    expose({ show, hide, enable, disable, update, setValue, getValue });

    return () =>
      visible.value && (
        <AFormItem {...props.config.props.formItem}>
          <ASelect
            {...rest}
            {...props.config.props.formWrap}
            disabled={disabled.value}
            style={props.config.style}
            value={props.formItemValue}
            options={data.value}
            onChange={(vals: any[]) => handleChange(vals)}
          ></ASelect>
        </AFormItem>
      );
  },
});

export default withInstall(Select);
"#;