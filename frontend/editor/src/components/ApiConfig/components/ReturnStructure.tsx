import { useEffect, useState } from "react";
import { Form, Input, InputNumber, Alert, Col, Row, Tabs } from "antd";
import Editor, { loader } from "@monaco-editor/react";
import { usePageStore } from "@/stores/pageStore";
import styles from "../index.module.less";
import { baseReturnMap, responseFuncMap } from "./SettingModal";

interface ReturnStructureProps {
  mode: "base" | "advanced";
  isEdit: boolean;
  onDataChange?: (data: any) => void; // 添加数据变化回调
}

loader.config({
  paths: {
    vs: window.location.origin + "/monaco-editor/0.50.0/min/vs",
  },
  "vs/nls": { availableLanguages: { "*": "zh-cn" } },
});

const ReturnStructure = function ({ mode, isEdit, onDataChange }: ReturnStructureProps) {
  const theme = usePageStore((state) => state.theme);
  const [editorReady, setEditorReady] = useState(false);
  const form = Form.useFormInstance();

  useEffect(() => {
    setEditorReady(true);
    return () => setEditorReady(false);
  }, []);

  // 监听表单值变化
  useEffect(() => {
    // 创建一个监听器来跟踪表单中 result 字段的变化
    const unsubscribe = form.getFieldInstance('result') 
      ? form.getFieldInstance('result').subscribe(({ values }) => {
          if (values && values.result && onDataChange) {
            onDataChange(values.result);
          }
        })
      : undefined;

    return () => {
      if (unsubscribe) {
        unsubscribe();
      }
    };
  }, [form, onDataChange]);

  // 创建Editor组件
  const createControlledEditor = (namePath: string[]) => {
    if (!form) return null;

    // 获取字段值并处理多种格式
    const fieldValue = form.getFieldValue(namePath);

    // 统一处理值：可能是undefined、字符串或{type, value}对象
    const editorValue = (() => {
      if (!fieldValue) {
        // 如果没有值，使用默认值
        const key = namePath[namePath.length - 1];
        return responseFuncMap[key]?.value || '';
      }
      if (typeof fieldValue === 'string') return fieldValue; // 直接字符串
      if (fieldValue.value) return fieldValue.value; // {type, value}对象
      return JSON.stringify(fieldValue); // 其他情况转为字符串
    })();

    return (
      <div className={styles.returnEditor}>
        {editorReady && (
          <Editor
            language="javascript"
            theme={theme === "dark" ? "vs-dark" : "vs-light"}
            value={editorValue}
            onChange={(val) => {
              // 根据原始值类型决定保存格式
              let newValue;
              if (!fieldValue || typeof fieldValue === 'string') {
                // 如果原来是字符串或没有值，保存为对象格式
                newValue = { type: 'custom', value: val };
              } else {
                // 保持对象结构
                newValue = { ...fieldValue, value: val };
              }

              form.setFieldValue(namePath, newValue);
              
              // 通知父组件数据已更改
              const currentResult = form.getFieldValue(['result']);
              if (onDataChange) {
                onDataChange(currentResult);
              }
            }}
            options={{
              lineNumbers: "on",
              minimap: { enabled: false },
              scrollBeyondLastLine: false,
            }}
          />
        )}
      </div>
    );
  };

  // 监听基础模式下的表单项变化
  const handleBaseFieldChange = () => {
    // 通知父组件数据已更改
    const currentResult = form.getFieldValue(['result']);
    if (onDataChange) {
      onDataChange(currentResult);
    }
  };

  const baseConfig = (
    <>
      <Alert message="用来定义接口返回结构，推荐结构：{`{ code: 0, data: {}, msg: '' }`}"
        type="info" showIcon style={{ marginBottom: 15 }} />
      <Row gutter={80}>
        <Col span={12}>
          <Form.Item
            label="状态码"
            name={["result", "statusCode"]}
            tooltip={<p>接口返回业务状态码，默认是：status</p>}
          >
            <Input 
              placeholder="默认为：status" 
              showCount 
              onChange={handleBaseFieldChange}
            />
          </Form.Item>
        </Col>
      </Row>
      <Row gutter={80}>
        <Col span={12}>
          <Form.Item
            label="业务码"
            name={["result", "code"]}
            tooltip={<p>接口返回业务业务码，默认是：code</p>}
          >
            <Input 
              placeholder="默认为：code" 
              maxLength={15} 
              showCount 
              onChange={handleBaseFieldChange}
            />
          </Form.Item>
        </Col>
        <Col span={12}>
          <Form.Item
            label="成功值"
            name={["result", "codeValue"]}
            wrapperCol={{ span: 24 }}
            tooltip={<p>接口返回成功时对应的状态码值，默认是：0</p>}
          >
            <InputNumber 
              style={{ width: '100%' }} 
              placeholder="默认为：0" 
              onChange={handleBaseFieldChange}
            />
          </Form.Item>
        </Col>
      </Row>
      <Row gutter={80}>
        <Col span={12}>
          <Form.Item
            label="结果字段"
            name={["result", "data"]}
            tooltip={<p>接口返回结果字段，默认是：data</p>}
          >
            <Input 
              placeholder="默认为：data" 
              maxLength={15} 
              showCount 
              onChange={handleBaseFieldChange}
            />
          </Form.Item>
        </Col>
        <Col span={12}>
          <Form.Item
            label="报错字段"
            name={["result", "msg"]}
            tooltip={<p>接口返回报错字段，默认是：msg</p>}
          >
            <Input 
              placeholder="默认为：msg" 
              maxLength={15} 
              showCount 
              onChange={handleBaseFieldChange}
            />
          </Form.Item>
        </Col>
      </Row>
    </>
  );

  const advancedConfig = (
    <Tabs size="small" >
      <Tabs.TabPane tab="状态码" key="statusCode">
        <Form.Item name={["result", "statusCode"]}>
          {createControlledEditor(["result", "statusCode"])}
        </Form.Item>
      </Tabs.TabPane>
      <Tabs.TabPane tab="业务码" key="code">
        <Form.Item name={["result", "code"]}>
          {createControlledEditor(["result", "code"])}
        </Form.Item>
      </Tabs.TabPane>
      <Tabs.TabPane tab="结果字段" key="data">
        <Form.Item name={["result", "data"]}>
          {createControlledEditor(["result", "data"])}
        </Form.Item>
      </Tabs.TabPane>
      <Tabs.TabPane tab="报错处理" key="msg">
        <Form.Item name={["result", "msg"]}>
          {createControlledEditor(["result", "msg"])}
        </Form.Item>
      </Tabs.TabPane>
    </Tabs>
  );

  return mode === 'base' ? baseConfig : advancedConfig;
};

export default ReturnStructure;
