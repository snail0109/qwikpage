import React, { forwardRef, useEffect, useImperativeHandle, useState } from 'react';
import { Form, Input, InputProps, FormItemProps } from 'antd';
import { ComponentType } from '../../types';
import { useFormContext } from '../../utils/context';
import { omit } from 'lodash-es';
import QIcon from '../../../components/icons/QIcon';
import { isObject, has, isString } from 'lodash-es';

/* 泛型只需要定义组件本身用到的属性，当然也可以不定义，默认为any */
export interface IConfig {
  defaultValue: string;
  formItem: FormItemProps;
  formWrap: InputProps & { prefixIcons?: string; suffixIcons?: string };
}
/**
 *
 * @param config 组件配置属性值
 * @param props 系统属性值：componentId、componentName等
 * @returns 返回组件
 */
const MInput = ({ id, type, formItemValue, config, onChange, onBlur, onPressEnter }: ComponentType<IConfig>, ref: any) => {
  const { initValues, getValue, inForm } = useFormContext();
  const [visible, setVisible] = useState(true);
  const [disabled, setDisabled] = useState<boolean | undefined>();
  // 初始化默认值
  useEffect(() => {
    const name: string = config.props.formItem?.name || id;
    const value = config.props.defaultValue;
    initValues(type, name, value);
  }, [config.props.defaultValue]);

  // 启用和禁用
  useEffect(() => {
    if (typeof config.props.formWrap.disabled === 'boolean') setDisabled(config.props.formWrap.disabled);
  }, [config.props.formWrap.disabled]);

  // 输入事件
  const handleChange = (val: string) => {
    const name = config.props.formItem?.name || id;
    if (!inForm) {
      // 控件不在表单内需要自行维护值
      initValues(type, name, val);
    }
    onChange?.(val);
  };

  // 失去焦点事件
  const handleBlur = (val: string) => {
    const name = config.props.formItem?.name || id;
    onBlur?.({
      [name]: val,
    });
  };
  // 回车事件
  const handlePressEnter = (val: string) => {
    const name = config.props.formItem?.name || id;
    onPressEnter?.({
      [name]: val,
    });
  };
  useImperativeHandle(ref, () => {
    return {
      show() {
        setVisible(true);
      },
      hide() {
        setVisible(false);
      },
      enable() {
        setDisabled(false);
      },
      disable() {
        setDisabled(true);
      },
      setValue: (value: any) => {
        const name = config.props.formItem?.name || id;
        if (isObject(value) && has(value, name)) {
          initValues(type, name, value[name]);
        } else if (isString(value)) {
          initValues(type, name, value);
        } else {
          console.error('[input]','setValue参数错误，请检查', value);
        }
      },
      getValue: () => {
        const name = config.props.formItem?.name || id;
        return getValue(name)
      }
    };
  });

  const Com = config.props.formWrap?.type === 'password' ? Input.Password : Input;

  return (
    visible && (
      <Form.Item {...config.props.formItem} data-id={id} data-type={type}>
        <Com
          {...omit(config.props.formWrap, ['prefixIcons', 'suffixIcons'])}
          disabled={disabled}
          variant={config.props.formWrap.variant || undefined}
          style={config.style}
          prefix={config.props.formWrap.prefixIcons ? <QIcon name={config.props.formWrap.prefixIcons} /> : null}
          suffix={config.props.formWrap.suffixIcons ? <QIcon name={config.props.formWrap.suffixIcons} /> : null}
          value={formItemValue}
          onChange={(event) => handleChange(event.target.value)}
          onBlur={(event) => handleBlur(event.target.value)}
          onPressEnter={(event: any) => handlePressEnter(event.target.value)}
        />
      </Form.Item>
    )
  );
};

export default forwardRef(MInput);
