import { useEffect, useState } from "react";
import { Form, Input, InputNumber, Alert, Col, Row, Tabs } from "antd";
import Editor, { loader } from "@monaco-editor/react";
import { usePageStore } from "@/stores/pageStore";
import styles from "../index.module.less";

interface ReturnStructureProps {
  mode: "base" | "advanced";
}

loader.config({
  paths: {
    vs: window.location.origin + "/monaco-editor/0.50.0/min/vs",
  },
  "vs/nls": { availableLanguages: { "*": "zh-cn" } },
});

const ReturnStructure = function ({ mode }: ReturnStructureProps) {
  const theme = usePageStore((state) => state.theme);
  const [editorReady, setEditorReady] = useState(false);
  const form = Form.useFormInstance();

  useEffect(() => {
    setEditorReady(true);
    return () => setEditorReady(false);
  }, []);

  // 创建Editor组件
  const createControlledEditor = (namePath: string[]) => {
    if (!form) return null;

    // 获取字段值并处理多种格式
    const fieldValue = form.getFieldValue(namePath);
    console.log('Raw field value:', fieldValue);

    // 统一处理值：可能是undefined、字符串或{type, value}对象
    const editorValue = (() => {
      if (!fieldValue) return ''; // undefined或空值
      if (typeof fieldValue === 'string') return fieldValue; // 直接字符串
      if (fieldValue.value) return fieldValue.value; // {type, value}对象
      return JSON.stringify(fieldValue); // 其他情况转为字符串
    })();

    console.log('Processed editor value:', editorValue);

    return (
      <div className={styles.returnEditor}>
        {editorReady && (
          <Editor
            language="javascript"
            theme={theme === "dark" ? "vs-dark" : "vs-light"}
            value={editorValue}
            onChange={(val) => {
              // 根据原始值类型决定保存格式
              const newValue = typeof fieldValue === 'object'
                ? { ...fieldValue, value: val } // 保持对象结构
                : val; // 直接保存字符串

              form.setFieldsValue({
                [namePath.join('.')]: newValue
              });
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

  const baseConfig = (
    <>
      <Alert message="用来定义接口返回结构，推荐结构：{`{ code: 0, data: {}, msg: '' }`}"
        type="info" showIcon style={{ marginBottom: 15 }} />
      <Row gutter={80}>
        <Col span={12}>
          <Form.Item
            label="业务码"
            name={["result", "code"]}
            tooltip={<p>接口返回业务状态码，默认是：code</p>}
          >
            <Input placeholder="默认为：code" maxLength={15} showCount />
          </Form.Item>
        </Col>
        <Col span={12}>
          <Form.Item
            label="成功值"
            name={["result", "codeValue"]}
            wrapperCol={{ span: 24 }}
            tooltip={<p>接口返回成功时对应的状态码值，默认是：0</p>}
          >
            <InputNumber style={{ width: '100%' }} placeholder="默认为：0" />
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
            <Input placeholder="默认为：data" maxLength={15} showCount />
          </Form.Item>
        </Col>
        <Col span={12}>
          <Form.Item
            label="报错字段"
            name={["result", "msg"]}
            tooltip={<p>接口返回报错字段，默认是：msg</p>}
          >
            <Input placeholder="默认为：msg" maxLength={15} showCount />
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
