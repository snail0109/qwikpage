pub const MATERIAL_INDEX: &str = r#"
import { defineComponent } from 'vue';
import type { PropType } from 'vue';
import { withInstall } from '@/utils/type';
import type { MaterialType } from '@/types';

const MarsRender = defineComponent({
  name: 'QRender',
  inheritAttrs: false,
  props: {
    elements: {
      type: Array as PropType<MaterialType['elements']>,
      default: () => []
    }
  },
  setup(props, { attrs }) {
    return () => {
      const elements = props.elements || [];
      return elements.map(item => {
        if (!item) {
          return <></>
        }
        return <q-material {...attrs} item={item} key={item.id} />
      })
    }
  }
});

export default withInstall(MarsRender);
"#;

pub const MATERIAL_ITEM_INDEX: &str = r#"
import { defineComponent, ref, watch, onMounted, inject } from "vue";
import type { PropType } from 'vue';
import { storeToRefs } from "pinia";
import { Tooltip } from 'ant-design-vue';
import { isNull, renderFormula, isFormPlugin } from '@/utils/util';
import appStore from "@/stores";
import { withInstall, defaultFormContext } from "@/utils/type";
import { setComponentRef } from '@/utils/useComponentRefs';
import { omit, isEmpty, cloneDeep } from 'lodash-es';
import { handleActionFlow } from '@/utils/action';
import type { ConfigType, ComItemType, EventType, UseFormContextType } from "@/types";
import * as components from '@/components/components'

let cachedComponents: any = {};
const Material = defineComponent({
  name: "QMaterial",
  inheritAttrs: false,
  props: {
    item: {
      type: Object as PropType<ComItemType>,
      default: () => ({})
    }
  },
  setup(props, { attrs }) {
    const useFormContext = inject<UseFormContextType>('useFormContext', () => defaultFormContext());
    const { inForm } = useFormContext();
    const { pageState } = storeToRefs(appStore.page);
    const currentCom = ref("");
    const config = ref<ConfigType>();
    const cached = ref(false);

    // 处理表单正则
    const handleFormRegExp = (config: ConfigType) => {
      if (config.props?.formItem) {
        // 判断当前控件是否处于Form内，是则解析正则等表单项属性
        if (inForm) {
          const rules = config.props?.formItem.rules || [];
          rules.map((item: any) => {
            if (item.pattern) {
              // 把字符串转成正则对象
              item.pattern = new RegExp(item.pattern);
            }
          });
          config.props.formItem.rules = rules;
          // 处理表单布局
          const { labelCol, wrapperCol } = config.props.formItem;
          if (isNull(labelCol?.span) && isNull(labelCol?.offset)) {
            delete config.props.formItem?.labelCol;
          }
          if (isNull(wrapperCol?.span) && isNull(wrapperCol?.offset)) {
            delete config.props.formItem?.wrapperCol;
          }
        } else {
          config.props.formItem.label = ''; // 清空标题
        }
      }
    }

    // 处理绑定变量
    const handleBindVariable = (config: ConfigType) => {
      Object.keys(config.props || {}).map((key) => {
        const variableObj = config.props[key];
        // 如果组件属性是对象，则判断是静态值还是变量
        if (typeof variableObj === 'object') {
          // 如果是静态值，则直接赋值。
          if (variableObj?.type === 'static') {
            config.props[key] = variableObj.value;
          } else if (variableObj?.type === 'variable') {
            // 绑定变量时，可能是变量，也可能是绑定某一个表单值
            config.props[key] = renderFormula(variableObj.value);
          }
        }
      });
    }

    // 生成事件函数，挂载到组件上，组件中的按钮在触发事件时，会执行这里的事件函数
    const createEvents = () => {
      const eventFunction: { [key: string]: (params: any) => void } = {};
      const events = config.value?.events || [];

      // 没有配置事件流，直接返回
      if (!events?.length) {
        return {};
      }
      // 把重复的事件push到数组中（一个点击事件，可能有多个事件流）
      const obj: { [key: string]: any[] } = {};
      events.forEach((event: EventType) => {
        if (event.actions?.length > 0) {
          obj[event.eventName] = (obj[event.eventName] || []).concat([event.actions]);
        }
      });
      // 遍历对象，按顺序执行事件流
      for (const key in obj) {
        eventFunction[key] = (params: any) => {
          // 同一个事件：循环执行多个事件流
          obj[key].forEach((actions) => {
            handleActionFlow(actions, params);
          });
        };
      }
      return eventFunction;
    };

    const updateConfig = () => {
      const item = props.item;
      const elementsMap = pageState.value.page.pageData.elementsMap;
      if (Object.keys(elementsMap).length === 0) return;
      const newConfig = cloneDeep(elementsMap[item.id].config);
      handleFormRegExp(newConfig);
      handleBindVariable(newConfig);
      config.value = newConfig;
    }

    onMounted(() => {
      const item = props.item;
      const elementsMap = pageState.value.page.pageData.elementsMap;
      if (Object.keys(elementsMap).length === 0) return;
      if (cachedComponents[item.type]) {
        // 从缓存对象内获取
        cached.value = true;
        currentCom.value = cachedComponents[item.type];
      } else {
        currentCom.value = item.type;
      }
      updateConfig();
    });

    watch(
      () => [
        pageState.value.page.pageData.variableData,
        pageState.value.page.pageData.formData,
        pageState.value.page.pageData.formItemData,
        pageState.value.page.pageData.elementsMap,
      ],
      () => {
        updateConfig();
      },
      { deep: true }
    );

    return () => {
      if (
        !currentCom.value ||
        config.value?.props.showOrHide == false ||
        !(currentCom.value in components)
      ) {
        return null;
      }
      const item = props.item;
      let value;
      if (config.value?.props.formItem && !isEmpty(config.value.props.formItem)) {
        value = inForm ? pageState.value.page.pageData.formData[inForm]?.[config.value.props.formItem.name] : pageState.value.page.pageData.formItemData[item.id];
      }
      const DynamicComponent = components[currentCom.value as keyof typeof components] as any;
      const isInForm = isFormPlugin(pageState.value.page.pageData.elementsMap[item.id], true);
      const tooltip = config.value?.props?.formItem?.tooltip || config.value?.props?.tooltip;
      return !isInForm && tooltip ? (
        <Tooltip title={tooltip} placement="topLeft">
          <DynamicComponent
            {...attrs}
            key={item.id}
            id={item.id}
            type={item.type}
            formItemValue={value}
            config={{ ...config.value, props: { ...omit(config.value?.props, ['showOrHide']) } }}
            elements={item.elements || []}
            {...createEvents()}
            ref={(el: any) => setComponentRef(item.id, el)}
          />
        </Tooltip>
      ) : (
        <DynamicComponent
          {...attrs}
          key={item.id}
          id={item.id}
          type={item.type}
          formItemValue={value}
          config={{ ...config.value, props: { ...omit(config.value?.props, ['showOrHide']) } }}
          elements={item.elements || []}
          {...createEvents()}
          ref={(el: any) => setComponentRef(item.id, el)}
        />
      )
    }
  },
});

export default withInstall(Material);
"#;