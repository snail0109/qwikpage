import { ComponentType } from '@/packages/types';
import { isNull } from '@/packages/utils/util';
import { Form, Switch } from 'antd';
import { useEffect, useState, useImperativeHandle, forwardRef } from 'react';
import { useFormContext } from '@/packages/utils/context';
import { isObject } from "lodash-es";
import { isArray } from "lodash-es";

/**
 *
 * @param config 组件配置属性值
 * @param props 系统属性值：componentid、componentname等
 * @returns 返回组件
 */
const MSwitch = ({ id, type, config, onChange }: ComponentType, ref: any) => {
  const { initValues, getValue, inForm } = useFormContext();
  const [visible, setVisible] = useState(true);
  const [disabled, setDisabled] = useState<boolean | undefined>();

  // 初始化默认值
  useEffect(() => {
    const name: string = config.props.formItem?.name;
    const value = config.props.defaultValue;
    initValues(type, name, value);
  }, [config.props.defaultValue]);

  // 启用和禁用
  useEffect(() => {
    if (typeof config.props.formWrap.disabled === 'boolean') setDisabled(config.props.formWrap.disabled);
  }, [config.props.formWrap.disabled]);

  // 对外暴露方法
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
      getValue: () => {
        const name = config.props.formItem?.name || id;
        return getValue(name);
      },
      setValue: (value: any) => {
        const name = config.props.formItem?.name || id;
        if (isObject(value) && value[name]) {
          initValues(type, name, value[name]);
        } else if (isArray(value)) {
          initValues(type, name, value);
        } else {
          console.error("[select]", "setValue参数错误，请检查", value);
        }
      },
    };
  });

  // 监听表单值变化
  const handleChange = (val: string) => {
    onChange &&
      onChange({
        [config.props.formItem.name]: val,
      });
  };
  return (
    visible && (
      <Form.Item {...config.props.formItem} data-id={id} data-type={type} valuePropName="checked">
        <Switch {...config.props.formWrap} value={config.props.defaultValue} disabled={disabled} style={config.style} onChange={handleChange} />
      </Form.Item>
    )
  );
};
export default forwardRef(MSwitch);
