import React, { useCallback, useEffect, useRef, useState } from 'react';
import { useRafState } from 'ahooks';
import MarsRender from '@/packages/MarsRender/MarsRender';
import { usePageStore } from '@/stores/pageStore';
import { handleActionFlow } from '@/packages/utils/action';
import { FormContext } from '@/packages/utils/context';
import { isNotEmpty, getInitValue } from '@/packages/utils/util';

/**
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const Page: React.FC = () => {
  const [position, setPosition] = useRafState({ x: 0, y: 0 });
  const [isDragging, setIsDragging] = useState(false);
  const cache = useRef({ offset: { x: 0, y: 0 }, isDragging: false });

  // 页面组件
  const { config, elements, formItemData, setFormItemData, setSelectedElement } = usePageStore((state) => {
    return {
      config: state.page.pageData.config,
      elements: state.page.pageData.elements,
      formItemData: state.page.pageData.formItemData,
      setFormItemData: state.setFormItemData,
      setSelectedElement: state.setSelectedElement,
    };
  });

  const handleMouseMove = (e: MouseEvent) => {
    if (isDragging) {
      setSelectedElement(null);
      setPosition({
        x: e.clientX - cache.current.offset.x,
        y: e.clientY - cache.current.offset.y,
      });
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    cache.current.offset = {
      x: e.clientX - position.x,
      y: e.clientY - position.y,
    };
    setIsDragging(true);
  };

  React.useEffect(() => {
    const addEventListener = () => {
      window.addEventListener('mousemove', handleMouseMove);
      window.addEventListener('mouseup', handleMouseUp);
    };
    const removeEventListener = () => {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };

    if (isDragging) {
      addEventListener();
    } else {
      removeEventListener();
    }

    return () => {
      removeEventListener();
    };
  }, [isDragging]);

  useEffect(() => {
    config.events?.forEach((event) => {
      if (event.actions?.length > 0) {
        handleActionFlow(event.actions, {});
      }
    });
  }, [config.events]);

  const initValues = useCallback((type: string, name: string, value: any) => {
    if (name && isNotEmpty(value)) {
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
    <FormContext.Provider value={{ initValues, getValue }}>
      <div
        style={{
          minHeight: 'calc(100vh - 74px - 40px)',
          ...config.style,
          // transform: `translate(${position.x}px, ${position.y}px)`,
          // cursor: isDragging ? 'move' : 'default',
        }}
        id="page"
      // onMouseDown={handleMouseDown}
      >
        {<MarsRender elements={elements || []} />}
      </div>
    </FormContext.Provider>
  );
};
export default Page;
