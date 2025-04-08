pub const FORM_INDEX: &str = r#"
import { defineComponent, ref, provide, watch, onMounted } from 'vue';
import { Form as AForm } from 'ant-design-vue';
import { storeToRefs } from "pinia";
import appStore from "@/stores";
import { withInstall } from '@/utils/type';
import { dateFormat, isNotEmpty, getInitValue } from '@/utils/util';
import type { FormContextType } from '@/types';
import { commonProps } from '@/types';

const Form = defineComponent({
  name: 'QForm',
  inheritAttrs: false,
  props: commonProps(),
  setup(props, { attrs, expose }: any) {
    const { onFinish, onChange, ...rest } = attrs;
    const { pageState } = storeToRefs(appStore.page);
    const { setFormData } = appStore.page;

    const formRef = ref();
    const visible = ref(true);

    // 提交表单
    const handleFinish = (values: any) => {
      onFinish?.(dateFormat(props.elements || [], values));
    };

    // 提交表单失败
    const handleFail = (errorInfo: any) => {
      console.log('Form Submit Failed:', errorInfo);
    }

    // 监听表单值变化
    const handleChange = (_: any, allValues: any) => {
      const values = dateFormat(props.elements || [], allValues);
      onChange?.(values);
    };
    const show = () => {
      visible.value = true;
    }
    const hide = () => {
      visible.value = false;
    }

    const reset = () => {
      formRef.value.resetFields();
    }

    const submit = () => {

    }

    const validate = async () => {
      try {
        await formRef.value.validate();
        return true;
      } catch (error) {
        console.error(error);
        return false;
      }
    }

    const init = (values: any = {}) => {
      const initData = dateFormat(props.elements || [], values);
      setFormData({
        name: props.id,
        value: initData,
        type: 'override',
      });
    }

    const getFormData = (key: string) => {
      const formData = pageState.value.page.pageData.formData;
      if (key && typeof key === 'string') {
        return formData[props.id]?.[key];
      }
      return formData[props.id];
    }

    expose({ show, hide, reset, submit, validate, init, getFormData });

    // 设置默认值
    const initValues = (type: string, name: any, value: any) => {
      if (name) {
        const initValue = getInitValue(type, value);
        setFormData({
          name: props.id,
          value: { [name]: initValue },
        });
      }
    }

    // 获取表单内某项的值
    const getValue = (name: string) => {
      const formData = pageState.value.page.pageData.formData;
      console.log("表单内控件取值打印 [name]: value", `[${name}]: `, formData[props.id]?.[name])
      return formData[props.id]?.[name];
    }

    const useFormContext = (): FormContextType => {
      return {
        initValues,
        getValue,
        inForm: props.id,
      }
    }

    provide('useFormContext', useFormContext)

    watch(
      () => pageState.value.page.pageData.formData[props.id],
      (formData) => {
        handleChange(null, formData);
      },
    )

    return () => visible.value && (
      <AForm
        {...rest}
        ref={formRef}
        id={props.id}
        style={props.config.style}
        model={pageState.value.page.pageData.formData[props.id]}
        {...props.config.props}
        onFinish={handleFinish}
        onFinishFailed={handleFail}
      >
        <q-render elements={props.elements || []} />
      </AForm>
    );
  }
});

export default withInstall(Form);
"#;