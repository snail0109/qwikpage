import { forwardRef, useImperativeHandle, useState, useRef } from "react";
import { Form, Modal, Tabs, ConfigProvider, Button } from "antd";
import type { TabsProps } from "antd";
import BaseSetting from "./BaseSetting";
import ReturnStructure from "./ReturnStructure";
import ReturnTips from "./ReturnTips";
import ApiTestModal from "./ApiTestModal";
import { usePageStore } from "@/stores/pageStore";
import { generateUUID } from "@/utils/util";
import styles from "../index.module.less";

export type SettingModalProp = {
  update?: (id: string) => void;
};

const SettingModal = ({ update }: SettingModalProp, ref: any) => {
  const { apis, addApi, updateApi } = usePageStore((state) => ({
    apis: state.page.pageData.apis,
    addApi: state.addApi,
    updateApi: state.updateApi,
  }));
  const [form] = Form.useForm();
  const [open, setOpen] = useState(false);
  const apiTestModalRef = useRef<{ showModal: (data?: any) => void }>();

  // 初始化接口配置数据
  const initValue = {
    method: "GET",
    url: "",
    sourceType: "json",
    params: [{ key: "", value: "" }],
    contentType: "application/json",
    replaceData: "merge",
    isCors: true,
    result: {
      code: "code",
      data: "data",
      msg: "msg",
      codeValue: 0,
    },
    tips: {
      success: "请求成功",
      fail: "请求失败",
      isSuccess: true,
      isError: true,
    },
  };
  useImperativeHandle(ref, () => ({
    showModal: (id?: string) => {
      // 初始化接口配置数据
      const apiConfig = id ? apis[id] : {};
      if (id) {
        form.setFieldsValue({ ...apiConfig });
      } else {
        // 如果API不存在，则使用默认值初始化
        form.setFieldsValue({ ...initValue, ...apiConfig });
      }

      setOpen(true);
    },
  }));

  const items: TabsProps["items"] = [
    {
      key: "base-set",
      label: `接口设置`,
      forceRender: true,
      children: <BaseSetting />,
    },
    {
      key: "structure",
      label: "返回结构设置",
      forceRender: true,
      children: <ReturnStructure />,
    },
    {
      key: "tips",
      label: "消息提示设置",
      forceRender: true,
      children: <ReturnTips />,
    },
  ];
  // 保存
  async function handleOk() {
    const valid = await form.validateFields();
    if (!valid) return;
    const values = form.getFieldsValue();
    // 如果有ID，只需要获取表单值进行合并即可，一定不能用values合并，因为里面包含初始化代码
    if (values.id) {
      updateApi(form.getFieldsValue());
    } else {
      const id = generateUUID();
      addApi({ ...initValue, ...form.getFieldsValue(), id: id });
    }
    // 确认后，把值回传给父组件
    update?.(values.id);

    handleCancel();
  }

  // 关闭弹框
  function handleCancel() {
    setOpen(false);
    form.resetFields();
  }

  // 做网络请求测试，拿到数据，填写到之后的弹出框中
  const handleApiTest = () => {
    // 获取当前页面的接口配置数据
    const apiConfig = form.getFieldsValue();

    apiTestModalRef.current?.showModal();
  }

  const customFooter = () => (
    <div style={{ display: 'flex', justifyContent: 'space-between' }}>
        <Button color="primary" variant="outlined" onClick={handleApiTest}>
          测试
        </Button>
        <div>
          <Button onClick={handleCancel} style={{ marginRight: '8px' }}>
            取消
          </Button>
          <Button type="primary" onClick={handleOk}>
            确定
          </Button>
        </div>
      </div>
  );
  

  return (
    <>
    <Modal
      wrapClassName={styles.apiSettingModal}
      width={"800px"}
      title="接口配置"
      open={open}
      onCancel={handleCancel}
      footer={customFooter}
    >
      <ConfigProvider
        theme={{
          token: {
            fontSize: 12,
          },
        }}
      >
        <Form form={form} layout="vertical" style={{ maxWidth: 800 }} autoComplete="off">
          <Tabs defaultActiveKey="1" items={items} size="small" />
        </Form>
      </ConfigProvider>
    </Modal>
    {/* 接口设置 */}
    <ApiTestModal ref={apiTestModalRef}></ApiTestModal>
    </>
  );
};

export default forwardRef(SettingModal);
