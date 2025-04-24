import { forwardRef, useState, useRef, useMemo } from "react";
import { Tabs, Table } from "antd";
import type { TabsProps } from "antd";
import Editor, { loader } from "@monaco-editor/react";
import { usePageStore } from "@/stores/pageStore";
import styles from "../index.module.less";
import { isNotEmpty } from "@/packages/utils/util";

interface ApiTestResultProps {
  testData: {
    data?: any;
    headers?: Record<string, string>;
    cookies?: Record<string, string>;
    status?: number;
    error?: any;
  };
}

const ApiTestResult = ({ testData }: ApiTestResultProps) => {
  const theme = usePageStore((state) => state.theme);

  const headersTableData = useMemo(() => {
    return Object.entries(testData.headers || {}).map(([key, value]) => ({
      key,
      value,
    }));
  }, [testData.headers]);

  const cookiesTableData = useMemo(() => {
    return Object.entries(testData.cookies || {}).map(([key, value]) => ({
      key,
      value,
    }));
  }, [testData.cookies]);

  const editorValue = useMemo(() => {
    if (testData.error) {
      return JSON.stringify({ error: testData.error }, null, 2);
    }
    if (isNotEmpty(testData.data)) {
      return typeof testData.data === "string" 
        ? testData.data 
        : JSON.stringify(testData.data, null, 2);
    }
    return "无响应数据";
  }, [testData.data, testData.error]);

  const editorComponent = useMemo(
    () => (
      <Editor
        height="200px"
        language="json"
        className={styles.dslEditor}
        value={editorValue}
        theme={theme === "dark" ? "vs-dark" : "vs-light"}
        options={{
          lineNumbers: "on",
          minimap: { enabled: false },
          readOnly: true,
        }}
      />
    ),
    [editorValue, theme]
  );

  const headersComponent = useMemo(
    () => (
      <Table
        dataSource={headersTableData}
        columns={[
          { title: "Header名称", dataIndex: "key", width: "50%" },
          { title: "Header值", dataIndex: "value", width: "50%" },
        ]}
        pagination={false}
        size="small"
        bordered
        locale={{ emptyText: "无Header信息" }}
      />
    ),
    [headersTableData]
  );

  const cookiesComponent = useMemo(
    () => (
      <Table
        dataSource={cookiesTableData}
        columns={[
          { title: "Cookie名称", dataIndex: "key", width: "50%" },
          { title: "Cookie值", dataIndex: "value", width: "50%" },
        ]}
        pagination={false}
        size="small"
        bordered
        locale={{ emptyText: "无Cookie信息" }}
      />
    ),
    [cookiesTableData]
  );

  const items: TabsProps["items"] = [
    {
      key: "body",
      label: "Body",
      forceRender: true,
      children: editorComponent,
    },
    {
      key: "cookies",
      label: "Cookies",
      forceRender: true,
      children: cookiesComponent,
    },
    {
      key: "headers",
      label: "Headers",
      forceRender: true,
      children: headersComponent,
    },
  ];

  // 初始化monaco，默认为jsdelivery分发，由于网络原因改为本地cdn
  loader.config({
    paths: {
      vs: window.location.origin + "/monaco-editor/0.50.0/min/vs",
    },
    "vs/nls": { availableLanguages: { "*": "zh-cn" } },
  });

  return <Tabs defaultActiveKey="body" items={items} size="small" />;
};

export default forwardRef(ApiTestResult);
