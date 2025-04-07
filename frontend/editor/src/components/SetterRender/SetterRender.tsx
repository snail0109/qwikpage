import React, { memo, useEffect } from 'react';
import { Form, Input, InputNumber, Radio, Select, Switch, Slider, FormInstance, Tooltip, Popover } from 'antd';
import * as icons from '@qwikpage/icons';
import { QuestionCircleOutlined, CaretDownOutlined } from '@ant-design/icons';
import { SchemaType } from '@/packages/types';
import MColorPicker from '../ColorPicker';
import VariableBindInput from '../VariableBind/VariableBind';
import InputSelect from '../InputSelect/InputSelect';
import InputPx from '../StyleConfig/InputPx';
import styles from './index.module.less';
import { usePageStore } from '@/stores/pageStore';
import { renderIconDefinitionToSVGElement } from '@qwikpage/icons/es/helpers';

// 如果没有设置label，则独占一行
const formLayoutFull = {
  labelCol: { span: 0 },
  wrapperCol: { span: 24 },
};
interface IAttrs {
  attrs: SchemaType[];
  form: FormInstance;
}
/**
 * 属性设置器
 * 根据JSON生成简单的属性配置
 */
const SetterRender = memo(({ attrs, form }: IAttrs) => {
  const { selectedElement, elementsMap } = usePageStore((state) => {
    return {
      selectedElement: state.selectedElement,
      elementsMap: state.page.pageData.elementsMap,
    };
  });

  const elementId = selectedElement?.id;
  const formItemId = selectedElement?.id ? elementsMap[selectedElement.id]?.config?.props?.formItem?.name : undefined;

  useEffect(() => {
    if (!formItemId || !elementId) return;

    // 准备要设置的表单值
    const formValues = { formItem: {} };
    let hasChanges = false;

    // 获取当前元素的表单项配置
    const formItemConfig = elementsMap[elementId]?.config?.props?.formItem;

    // 处理labelCol配置
    if (formItemConfig?.labelCol?.span !== undefined) {
      formValues.formItem.labelCol = { span: formItemConfig.labelCol.span };
      hasChanges = true;
    } else {
      // 如果当前元素没有配置，尝试获取父元素的配置
      const parentId = elementsMap[elementId]?.parentId;
      if (parentId && elementsMap[parentId]) {
        const parentElementLabelCol = elementsMap[parentId].config.props.labelCol;
        if (parentElementLabelCol?.span !== undefined) {
          formValues.formItem.labelCol = { span: parentElementLabelCol.span };
          hasChanges = true;
        }
      }
    }

    // 处理wrapperCol配置
    if (formItemConfig?.wrapperCol?.span !== undefined) {
      formValues.formItem.wrapperCol = { span: formItemConfig.wrapperCol.span };
      hasChanges = true;
    } else {
      // 如果当前元素没有配置，尝试获取父元素的配置
      const parentId = elementsMap[elementId]?.parentId;
      if (parentId && elementsMap[parentId]) {
        const parentElementWrapperCol = elementsMap[parentId].config.props.wrapperCol;
        if (parentElementWrapperCol?.span !== undefined) {
          formValues.formItem.wrapperCol = { span: parentElementWrapperCol.span };
          hasChanges = true;
        }
      }
    }

    // 只有当有值需要设置时才调用setFieldsValue，避免不必要的渲染
    if (hasChanges) {
      form.setFieldsValue(formValues);
    }
  }, [formItemId, elementId, elementsMap, form]);

  if (attrs.length === 0) return <></>;
  // console.log(attrs)

  // 根据type枚举
  return (
    <>
      {/* ---组件共有属性--- */}
      {/* 组件名称 */}
      {!formItemId && elementId && <Form.Item name={['formItem', 'name']} label="组件名称">
        <Input defaultValue={elementId} />
      </Form.Item>}
      {formItemId && <Form.Item name={['formItem', 'name']} label="组件名称">
        <Input defaultValue={formItemId} />
      </Form.Item>}
      {/* 是否显示 */}
      {(formItemId || elementId) && <Form.Item layout='horizontal' colon={false} key="showOrHide" name="showOrHide" label="是否显示" valuePropName="checked">
        <Switch size='small' defaultChecked />
      </Form.Item>}

      {/* ---组件属性--- */}
      {attrs.map((item: SchemaType, index) => {
        if (!item) return;
        const key = item.key || item.name?.toString() || item.label?.toString() + index.toString();
        let FormControl = <></>;

        // 检查是否是表单项配置（包含标题，name字段，和栅格等）
        const isFormItemLabelOrName = Array.isArray(item.name) &&
          item.name.length > 0 &&
          item.name[0] === 'formItem' &&
          (item.name[1] === 'label' || item.name[1] === 'name' || item.name[1] === 'labelCol' || item.name[1] === 'wrapperCol');

        // 如果是表单项的标题或字段配置，栅格之类的，则跳过渲染
        if (isFormItemLabelOrName && !formItemId) {
          return null;
        }

        if (item.type == 'Title') {
          return null;
          // return (
          //   <h2 className={styles.title} key={key}>
          //     <span style={{ marginRight: 10 }}>{item.label}</span>
          //     {/* 标题增加提示信息 */}
          //     {item.tooltip ? <Tooltip title={item.tooltip}>{<QuestionCircleOutlined />}</Tooltip> : null}

          //     {/* 标题增加跳转链接 */}
          //     {item.link ? (
          //       <a href={item.link.url} target="_blank" style={{ fontSize: 12 }}>
          //         {item.link.label}
          //       </a>
          //     ) : null}
          //   </h2>
          // );
        } else if (item.type == 'Input') {
          FormControl = <Input {...item.props} />;
        } else if (item.type === 'InputPx') {
          FormControl = <InputPx {...item.props} />;
        } else if (item.type == 'TextArea') {
          FormControl = <Input.TextArea rows={3} cols={8} {...item.props} />;
        } else if (item.type == 'InputSelect') {
          FormControl = <InputSelect {...item.props} />;
        } else if (item.type == 'Switch') {
          return (
            <Form.Item layout='horizontal' colon={false} key={key} name={item.name} label={item.label} tooltip={item.tooltip} valuePropName="checked">
              <Switch size='small' />
            </Form.Item>
          );
        } else if (item.type == 'Select') {
          FormControl = <Select {...item.props} />;
        } else if (item.type == 'Radio') {
          FormControl = <Radio.Group {...item.props} suffixIcon={<CaretDownOutlined />} />;
        } else if (item.type == 'InputNumber') {
          FormControl = <InputNumber {...item.props} style={{ width: '100%' }} />;
        } else if (item.type == 'RadioGroup') {
          FormControl = <Radio.Group {...item.props} />;
        } else if (item.type == 'RadioGroupBtn') {
          FormControl = <Radio.Group {...item.props} optionType="button" block />;
        } else if (item.type == 'ColorPicker') {
          FormControl = <MColorPicker {...item.props} format="hex" />;
        } else if (item.type == 'Slider') {
          FormControl = <Slider {...item.props} />;
        } else if (item.type === 'Variable') {
          FormControl = <VariableBindInput {...item.props} />;
        } else if (item.type === 'function' && formItemId) {
          return item.render?.(form);
        } else if (item.type === 'Icons') {
          // 获取所有的antd图标，动态渲染到下拉框中
          const iconsList: { [key: string]: any } = icons;
          FormControl = (
            <Select placeholder="请选择菜单图表" showSearch allowClear>
              {Object.keys(icons)
                .filter((item) => !['default', 'createFromIconfontCN', 'getTwoToneColor', 'setTwoToneColor', 'IconProvider'].includes(item))
                .map((key) => {
                  const svgHTMLString = renderIconDefinitionToSVGElement(iconsList[key], {
                    extraSVGAttrs: { width: '1em', height: '1em', fill: 'currentColor' },
                  })
                  return (
                    <Select.Option value={key} key={key}>
                      <span
                        className='anticon'
                        style={{
                          fontSize: '18px',
                          verticalAlign: 'middle',
                        }}
                        dangerouslySetInnerHTML={{ __html: svgHTMLString }}
                      />
                    </Select.Option>
                  );
                })}
            </Select>
          );
        }
        return (
          <Form.Item key={key} name={item.name} label={item.label} tooltip={item.tooltip} {...(item.label ? null : formLayoutFull)}>
            {FormControl}
          </Form.Item>
        );
      })}
    </>
  );
});

export default SetterRender;
