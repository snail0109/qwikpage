import { forwardRef, useImperativeHandle, useState, useRef } from "react";
import { Form, Modal, Tabs, ConfigProvider, Button, Spin, Radio } from "antd";
import type { TabsProps } from "antd";
import BaseSetting from "./BaseSetting";
import ReturnStructure from "./ReturnStructure";
import ReturnTips from "./ReturnTips";
import ApiTestResult from "./ApiTestResult";
import { usePageStore } from "@/stores/pageStore";
import { generateUUID } from "@/utils/util";
import styles from "../index.module.less";
import { handleApiTest } from '@/packages/utils/handleApi';
import { CustomResType } from "@/packages/types";

export type SettingModalProp = {
  update?: (id: string) => void;
};

// 定义参数类型
interface ParamType {
  key: string;
  value: string | { type: string; value: string };
}

// 自定义响应函数类型
export const responseFuncMap: CustomResType = {
  statusCode: {
    type: 'custom',
    value: `/**
* 根据状态码判断是否请求成功
* @param statusCode: 请求状态码
* @return {boolean}: true 表示请求成功，false 表示请求失败
*/
function response(statusCode){
    return [200, 201, 202, 204, 206].includes(statusCode);
}`
  },
  code: {
    type: 'custom',
    value: `/**
* 根据业务码判断是否请求成功
* @param resData: 响应数据
* @return {boolean}: true 表示请求成功，false 表示请求失败
*/
function response(resData){
    return resData.code === 0;
}`
  },
  data: {
    type: 'custom',
    value: `/**
* 返回请求结果数据
* @param resData: 响应数据
* @return {object} data: 请求结果数据
*/
function response(resData){
    return resData.data;
}`
  },
  msg: {
    type: 'function',
    value: `/**
* 返回请求提示信息
* @param resData: 响应数据
* @return {string} message: 提示信息
*/
function response(resData){
    return resData.msg;
}`
  },
}

const SettingModal = ({ update }: SettingModalProp, ref: any) => {
  const { apis, addApi, updateApi } = usePageStore((state) => ({
    apis: state.page.pageData.apis,
    addApi: state.addApi,
    updateApi: state.updateApi,
  }));
  const [form] = Form.useForm();
  const [open, setOpen] = useState(false);
  const [activeTabKey, setActiveTabKey] = useState('base-set');
  const [configMode, setConfigMode] = useState<'base' | 'advanced'>('base');
  const [loading, setLoading] = useState(false); // 添加 loading 状态
  const [testResult, setTestResult] = useState<any>(null);

  // 初始化接口配置数据
  const initValue = {
    method: "GET",
    apiUrl: "",
    params: [{ key: "", value: "" }],
    contentType: "application/json",
    replaceData: "merge",
    isCors: true,
    result: {
      statusCode: "200,201,202,204,206",
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
      children: <ReturnStructure mode={configMode} />,
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
    setTestResult(null); // 重置测试功能里面的数据
  }

  // 做网络请求测试，拿到数据，填写到之后的弹出框中
  const handleRequestTest = async () => {
    setLoading(true); // 开始加载
    setTestResult(null);

    try {
      const apiConfig = form.getFieldsValue();
      // 转换参数格式以匹配 handleApi 的要求
      const sendParams = apiConfig.params?.reduce((acc: Record<string, any>, param: ParamType) => {
        acc[param.key] = typeof param.value === 'object' ? param.value.value : param.value;
        return acc;
      }, {}) || {};

      // 使用 handleApi 发送请求
      const response = await handleApiTest(apiConfig, sendParams);
      setTestResult(response);
    } catch (error: any) {
      console.error("API 测试错误:", error);
      setTestResult({
        error: true,
        message: error.message || "请求失败"
      });
    } finally {
      setLoading(false);
    }
  };

  const customFooter = () => (
    <div style={{ display: 'flex', justifyContent: 'space-between' }}>
      <div>
        <Button
          color="primary"
          variant="outlined"
          onClick={handleRequestTest}
          disabled={loading} // 在加载时禁用按钮
          style={{ marginRight: '8px' }}
        >
          测试
        </Button>
        {activeTabKey === 'structure' && (
          <Radio.Group
            value={configMode}
            onChange={(e) => setConfigMode(e.target.value)}
            optionType="button"
            buttonStyle="solid"
            options={[
              { value: 'base', label: '基础' },
              { value: 'advanced', label: '高级' },
            ]}
          />
        )}
      </div>
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
            components: {
              Tabs: {
                titleFontSize: 14,
                titleFontSizeSM: 12,
              },
            },
          }}
        >
          <Spin spinning={loading} tip="请求测试中...">
            <Form form={form} layout="vertical" style={{ maxWidth: 800 }} autoComplete="off">
              <Tabs defaultActiveKey="1" items={items} onChange={(key) => {
                setActiveTabKey(key);
                if (key !== 'structure') {
                  setConfigMode('base');
                }
              }} />
            </Form>
            {testResult && <ApiTestResult testData={testResult} />}
          </Spin>
        </ConfigProvider>
      </Modal>
    </>
  );
};

export default forwardRef(SettingModal);
