import { forwardRef, useImperativeHandle, useState, useRef } from "react";
import { Form, Modal, ConfigProvider } from "antd";
import Editor, { loader } from "@monaco-editor/react";
import { usePageStore } from "@/stores/pageStore";
import styles from "../index.module.less";
import { isNotEmpty } from "@/packages/utils/util";

const ApiTestModal = (props: any, ref: any) => { 
  const editorRef = useRef<any>(null);
  const theme = usePageStore((state) => state.theme);
  const [form] = Form.useForm();
  const [open, setOpen] = useState(false);
  const [testValue, setTestValue] = useState("");

  useImperativeHandle(ref, () => ({
    showModal: (data?: string) => {
      if (data) {
        setTestValue(data);
      }
      setOpen(true);
    },
  }));

  // 确定
  async function handleOk() {
    handleCancel();
  }

  // 取消
  function handleCancel() {
    setOpen(false);
    form.resetFields();
  }

  // 初始化monaco，默认为jsdelivery分发，由于网络原因改为本地cdn
  loader.config({
    paths: {
      vs: `/monaco-editor/0.52.2/min/vs`,
    },
    "vs/nls": { availableLanguages: { "*": "zh-cn" } },
  });

  return (
    <Modal wrapClassName={styles.apiSettingModal} width={"597px"} title="测试" open={open} onCancel={handleCancel}>
      <ConfigProvider
        theme={{
          token: {
            fontSize: 12,
          },
        }}
      >
        <Editor
          height="calc(100vh - 36px)"
          language="json"
          className={styles.dslEditor}
          value={
            isNotEmpty(testValue)
              ? typeof testValue === "string"
                ? testValue
                : JSON.stringify(testValue, null, 2)
              : ""
          }
          theme={theme === "dark" ? "vs-dark" : "vs-light"}
          options={{
            lineNumbers: "on",
            minimap: {
              enabled: false,
            },
          }}
          onMount={(editor, monaco) => {
            editorRef.current = { editor, monaco };
          }}
        />
      </ConfigProvider>
    </Modal>
  );
};

export default forwardRef(ApiTestModal);
