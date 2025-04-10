import { memo, useEffect, useCallback } from 'react';
import MarsRender from '@materials/MarsRender/MarsRender';
import { FormContext } from '@materials/utils/context';
import { usePageStore } from '@materials/stores/pageStore';
import { handleActionFlow } from '@materials/utils/action';
import { ComItemType, ConfigType } from '@materials/types/index';
import { getInitValue } from '@materials/utils/util';

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
    config?.events?.forEach((event: any) => {
      if (event.actions?.length > 0) {
        handleActionFlow(event.actions, {});
      }
    });
  }, [config?.events]);

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
