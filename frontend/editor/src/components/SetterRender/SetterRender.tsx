import React, { memo } from "react";
import { Form, Input, InputNumber, Radio, Select, Switch, Slider, FormInstance } from "antd";
import * as icons from "@qwikpage/icons";
import { CaretDownOutlined } from "@ant-design/icons";
import { SchemaType } from "@/packages/types";
import MColorPicker from "../ColorPicker";
import VariableBindInput from "../VariableBind/VariableBind";
import InputSelect from "../InputSelect/InputSelect";
import InputPx from "../StyleConfig/InputPx";
import { usePageStore } from "@/stores/pageStore";
import QIcon from "@/components/icons/QIcon";

// 如果没有设置label，则独占一行
const formLayoutFull = {
  labelCol: { span: 0 },
  wrapperCol: { span: 24 },
};

// 定义名称格式校验的正则表达式
const REG_EXP_VARIABLE_NAME = /^[a-zA-Z][a-zA-Z0-9_]*$/;

interface IAttrs {
  attrs: SchemaType[];
  form: FormInstance;
  handleSpecialFieldBlur: (fieldName: string | string[], originalValue: string) => void;
}

/**
 * 属性设置器
 * 根据JSON生成简单的属性配置
 */

const SetterRender = memo(({ attrs, form, handleSpecialFieldBlur = () => { } }: IAttrs) => {
  const { selectedElement, elementsMap } = usePageStore((state) => {
    return {
      selectedElement: state.selectedElement,
      elementsMap: state.page.pageData.elementsMap,
    };
  });

  const elementId = selectedElement?.id;
  const formItemId = selectedElement?.id ? elementsMap[selectedElement.id]?.config?.props?.formItem?.name : undefined;

  // 校验组件名称
  const validateComponentName = async (rule: any, value: string): Promise<any> => {
    // 格式校验
    if (value && !REG_EXP_VARIABLE_NAME.test(value)) {
      return Promise.reject(new Error("必须以字母开头，且只能包含字母、数字或下划线"));
    }
    return Promise.resolve();
  };

  if (attrs.length === 0) return <></>;

  // 根据type枚举
  return (
    <>
      {/* ---组件共有属性--- */}
      {/* 组件名称 */}
      {!formItemId && elementId && (
        <Form.Item name={"id"} label="组件名称" rules={[{ validator: validateComponentName }]}>
          <Input onBlur={() => handleSpecialFieldBlur("id", elementId)} />
        </Form.Item>
      )}
      {formItemId && (
        <Form.Item name={["formItem", "name"]} label="组件名称" rules={[{ validator: validateComponentName }]}>
          <Input onBlur={() => handleSpecialFieldBlur(["formItem", "name"], formItemId)} />
        </Form.Item>
      )}
      {/* 是否显示 */}
      {(formItemId || elementId) && (
        <Form.Item
          layout="horizontal"
          colon={false}
          key="showOrHide"
          name="showOrHide"
          label="是否显示"
          valuePropName="checked"
        >
          <Switch size="small" defaultChecked />
        </Form.Item>
      )}

      {/* ---组件属性--- */}
      {attrs.map((item: SchemaType, index) => {
        if (!item) return null;
        const key = item.key || item.name?.toString() || item.label?.toString() + index.toString();
        let FormControl = <></>;

        // 检查是否是表单项配置（包含标题，name字段，和栅格等）
        const isFormItemLabelOrName =
          Array.isArray(item.name) &&
          item.name.length > 0 &&
          item.name[0] === "formItem" &&
          (item.name[1] === "label" ||
            item.name[1] === "name" ||
            item.name[1] === "labelCol" ||
            item.name[1] === "wrapperCol");

        // 如果是表单项的标题或字段配置，栅格之类的，则跳过渲染
        if (isFormItemLabelOrName && !formItemId) {
          return null;
        }

        if (item.type === "Title") {
          return null;
        } else if (item.type === "Input") {
          FormControl = <Input {...item.props} />;
        } else if (item.type === "InputPx") {
          FormControl = <InputPx {...item.props} />;
        } else if (item.type === "TextArea") {
          FormControl = <Input.TextArea rows={3} cols={8} {...item.props} />;
        } else if (item.type === "InputSelect") {
          FormControl = <InputSelect {...item.props} />;
        } else if (item.type === "Switch") {
          return (
            <Form.Item
              layout="horizontal"
              colon={false}
              key={key}
              name={item.name}
              label={item.label}
              tooltip={item.tooltip}
              valuePropName="checked"
            >
              <Switch size="small" />
            </Form.Item>
          );
        } else if (item.type === "Select") {
          FormControl = <Select {...item.props} />;
        } else if (item.type === "Radio") {
          FormControl = <Radio.Group {...item.props} suffixIcon={<CaretDownOutlined />} />;
        } else if (item.type === "InputNumber") {
          FormControl = <InputNumber {...item.props} style={{ width: "100%" }} />;
        } else if (item.type === "RadioGroup") {
          FormControl = <Radio.Group {...item.props} />;
        } else if (item.type === "RadioGroupBtn") {
          FormControl = <Radio.Group {...item.props} optionType="button" block />;
        } else if (item.type === "ColorPicker") {
          FormControl = <MColorPicker {...item.props} format="hex" />;
        } else if (item.type === "Slider") {
          FormControl = <Slider {...item.props} />;
        } else if (item.type === "Variable") {
          FormControl = <VariableBindInput {...item.props} />;
        } else if (item.type === "function" && formItemId) {
          return item.render?.(form);
        } else if (item.type === "Icons") {
          FormControl = (
            <Select placeholder="请选择菜单图表" showSearch allowClear>
              {Object.keys(icons)
                .filter(
                  (item) =>
                    ![
                      "default",
                      "createFromIconfontCN",
                      "getTwoToneColor",
                      "setTwoToneColor",
                      "IconProvider",
                    ].includes(item)
                )
                .map((key) => {
                  return (
                    <Select.Option value={key} key={key}>
                      <QIcon name={key} />
                    </Select.Option>
                  );
                })}
            </Select>
          );
        }
        return (
          <Form.Item
            key={key}
            name={item.name}
            label={item.label}
            tooltip={item.tooltip}
            {...(item.label ? null : formLayoutFull)}
          >
            {FormControl}
          </Form.Item>
        );
      })}
    </>
  );
});

export default SetterRender;
