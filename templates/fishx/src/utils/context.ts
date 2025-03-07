/**
 * 公共Context对象，用于跨组件通信
 * 主要用在Form、SearchForm中
 * 子表单中进行消费
 */
import { FormInstance } from 'antd';
import { createContext, useContext } from 'react';

export const FormContext = createContext<{
  form?: FormInstance;
  initValues: (type: string, name: string, value: any) => void;
  getValue: (name: string) => any;
} | null>(null);

export const useFormContext = () => {
  const context = useContext(FormContext);
  if (!context) {
    return {
      initValues() { },
      getValue() { return null; },
    };
  }
  return context;
};
