import { memo, useEffect, useCallback } from 'react';
import MarsRender from '@materials/MarsRender/MarsRender';
import { FormContext } from '@materials/utils/context';
import { usePageStore } from '@materials/stores/pageStore';
import { handleActionFlow } from '@materials/utils/action';
import { ComItemType, ConfigType, EventType } from '@materials/types/index';
import { getInitValue } from '@materials/utils/util';

let eventFunction: { [key: string]: (params?: any) => void } = {};
const createEvents = (events: EventType[]) => {
  eventFunction = {};
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

/**
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const Page = ({ config, elements }: { config?: ConfigType; elements?: ComItemType[] }) => {

  const { formItemData, setFormItemData } = usePageStore((state) => {
    return {
      formItemData: state.page.pageData.formItemData,
      setFormItemData: state.setFormItemData,
    };
  });

  useEffect(() => {
    createEvents(config?.events || []);
    // 页面初始化
    eventFunction['onLoad']?.();
  }, [config?.events]);

  useEffect(() => {
    createEvents(config?.events || []);
    // 实例化挂载完成
    eventFunction['onMount']?.();
    return () => {
      // 销毁事件
      eventFunction['onDestory']?.();
    }
  }, []);

  const initValues = useCallback((type: string, name: string, value: any) => {
    if (name) {
      const initValue = getInitValue(type, value);
      setFormItemData({
        name,
        value: initValue,
      });
    }
  }, []);

  const getValue = useCallback((name: string) => {
    const value = formItemData[name];
    return value;
  }, [formItemData]);

  return (
    // FormContext.Provider 用于管理不在表单内的控件 取值 赋值
    <FormContext.Provider value={{ initValues, getValue, inForm: false }}>
      <div style={config?.style}>{<MarsRender elements={elements || []} />}</div>
    </FormContext.Provider>
  );
};
export default memo(Page);
