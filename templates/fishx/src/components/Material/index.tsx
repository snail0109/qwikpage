import React, { memo, useEffect, useState } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { produce } from 'immer';
import { handleActionFlow } from '@/utils/action';
import { usePageStore } from '@/stores/pageStore';
import { isNull, renderFormula } from '@/utils/util';
import { ConfigType, MaterialProp } from '@/types';
import { useFormContext } from '@utils/context';

const Material = ({ id, type, config, children }: MaterialProp) => {
  const { inForm } = useFormContext();
  const [newConfig, setConfig] = useState<ConfigType>();

  const { elementsMap, variableData, formData, formItemData } = usePageStore(
    useShallow((state) => ({
      elementsMap: state.page.pageData.elementsMap,
      variables: state.page.pageData.variables,
      variableData: state.page.pageData.variableData,
      formData: state.page.pageData.formData,
      formItemData: state.page.pageData.formItemData,
    })),
  );

  useEffect(() => {
    if (Object.keys(elementsMap).length === 0) return;
    setConfig(config);
  }, []);

  useEffect(() => {
    if (Object.keys(elementsMap).length === 0) return;
    setConfig(() => {
      return produce(config, (draft: ConfigType) => {
        handleFormRegExp(draft);
        handleBindVariable(draft);
      });
    });
  }, [variableData, formData, formItemData, elementsMap]);

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
        // FormList比较特殊，需要传递索引 TODO: FormList name需要特殊处理
        // if (item.parentId?.startsWith('FormList') && config.props.formItem.name) {
        //   config.props.formItem.name = [item.name, config.props.formItem.name];
        // }
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
  };
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
  };

  // 生成事件函数，挂载到组件上，组件中的按钮在触发事件时，会执行这里的事件函数
  const createEvents = () => {
    const eventFunction: { [key: string]: (params: any) => void } = {};
    const events = config?.events || [];

    // 没有配置事件流，直接返回
    if (!events?.length) {
      return {};
    }
    // 把重复的事件push到数组中（一个点击事件，可能有多个事件流）
    const obj: { [key: string]: any[] } = {};
    events.forEach((event) => {
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

  return React.isValidElement(children) ? React.cloneElement(children, {
    ...(children.props || {}),
    id,
    type,
    formItemValue: formItemData[id],
    config: newConfig,
    ...createEvents()
  }) : null;
}
export default memo(Material);