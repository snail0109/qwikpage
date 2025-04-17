import React, { Suspense, forwardRef, memo, useEffect, useState } from 'react';
import { ComItemType, ConfigType } from '@/packages/types/index';
import { handleActionFlow } from '@/packages/utils/action';
import { setComponentRef } from '@/packages/utils/useComponentRefs';
import { usePageStore } from '@/stores/pageStore';
import { useProjectStore } from '@/stores/projectStore';
import { useShallow } from 'zustand/react/shallow';
import { produce } from 'immer';
import dayjs from 'dayjs';
import * as antd from 'antd';
import * as Plots from '@ant-design/plots';
import * as icons from '@ant-design/icons';
import { isNull, loadStyle, renderFormula, isFormPlugin } from '@/packages/utils/util';
import { omit } from 'lodash-es';
import { getComponent } from '@/packages/index';
import ComWrapper from './ComWrapper';
import './index.less';

/**
 * 编辑器用于生成组件
 * @param elements 模板所有组件对象
 * @param form 只有Form组件会传递form对象，用于子表单更新数据
 * @returns
 */
const MarsRender = memo(({ elements = [] }: { elements: ComItemType[] }) => {
  return (
    <>
      {elements.map((item) => {
        if (!item) return <></>;
        return <Material item={item} key={item.id}></Material>;
      })}
    </>
  );
});

// 渲染物料
export const Material = memo(({ item }: { item: ComItemType }) => {
  const [Component, setComponent] = useState<any>(null);
  const [config, setConfig] = useState<ConfigType>();

  const { elementsMap, variableData, formData, formItemData, updateToolbar } = usePageStore(
    useShallow((state) => ({
      elementsMap: state.page.pageData.elementsMap,
      variableData: state.page.pageData.variableData,
      formData: state.page.pageData.formData,
      formItemData: state.page.pageData.formItemData,
      updateToolbar: state.updateToolbar,
    })),
  );

  const { projectVariableData } = useProjectStore(
    useShallow((state) => ({
      projectVariableData: state.variableData,
    })),
  );

  function initContext() {
    window.React = window.React || React;
    window.dayjs = window.dayjs || dayjs;
    window.antd = window.antd || antd;
    window.Plots = window.Plots || Plots;
    window.icons = window.icons || icons;
  }

  useEffect(() => {
    if (Object.keys(elementsMap).length === 0) return;
    if (elementsMap[item.id].remoteUrl) {
      initContext();
      loadStyle(item.type, elementsMap[item.id].remoteCssUrl as string);
      /* @vite-ignore */
      import(elementsMap[item.id].remoteUrl || '').then((res) => {
        setComponent(() => {
          return forwardRef(res.default);
        });
        updateToolbar();
      });
    } else {
      setComponent(getComponent(item.type));
      updateToolbar();
    }
    setConfig(elementsMap[item.id].config);
  }, []);

  useEffect(() => {
    if (Object.keys(elementsMap).length === 0) return;
    setConfig(() => {
      return produce(elementsMap[item.id].config, (draft: ConfigType) => {
        handleFormRegExp(draft);
        handleBindVariable(draft);
      });
    });
  }, [variableData, projectVariableData, formData, formItemData, elementsMap]);

  // 处理表单正则
  const handleFormRegExp = (config: ConfigType) => {
    if (config.props?.formItem) {
      // 判断当前控件是否处于Form内，是则解析正则等表单项属性
      if (item.inForm) {
        const rules = config.props?.formItem.rules || [];
        rules.map((item: any) => {
          if (item.pattern) {
            // 把字符串转成正则对象
            item.pattern = new RegExp(item.pattern);
          }
        });
        config.props.formItem.rules = rules;
        // FormList比较特殊，需要传递索引
        if (item.parentId?.startsWith('FormList') && config.props.formItem.name) {
          config.props.formItem.name = [item.name, config.props.formItem.name];
        }
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
        } else if (['variable', 'globalVariable'].includes(variableObj?.type)) {
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

  const isInForm = isFormPlugin(elementsMap[item.id], true);
  const tooltip = config?.props?.formItem?.tooltip || config?.props?.tooltip;

  if (Component && config?.props.showOrHide !== false) {
    return (
      <Suspense fallback={<antd.Spin size="default"></antd.Spin>}>
        {
          !isInForm && tooltip ? (
            <antd.Tooltip title={tooltip} placement="topLeft">
              <ComWrapper>
                <Component
                  id={item.id}
                  type={item.type}
                  formItemValue={formItemData[item.id]}
                  config={{ ...config, props: { ...omit(config?.props, ['showOrHide']) } }}
                  elements={item.elements || []}
                  // 把事件函数传递给子组件，子组件触发对应事件时，会执行回调函数
                  {...createEvents()}
                  ref={(ref: any) => setComponentRef(item.id, ref)}
                />
              </ComWrapper>

            </antd.Tooltip>
          ) : (
            <Component
              id={item.id}
              type={item.type}
              formItemValue={formItemData[item.id]}
              config={{ ...config, props: { ...omit(config?.props, ['showOrHide']) } }}
              elements={item.elements || []}
              // 把事件函数传递给子组件，子组件触发对应事件时，会执行回调函数
              {...createEvents()}
              ref={(ref: any) => setComponentRef(item.id, ref)}
            />
          )
        }
      </Suspense>
    );
  }

  return null;
});

export default MarsRender;
