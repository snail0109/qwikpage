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
import { fetch } from '@tauri-apps/plugin-http';

export type SettingModalProp = {
  update?: (id: string) => void;
};

// 定义参数类型
interface ParamType {
  key: string;
  value: string | { type: string; value: string };
}

// 定义简化后的参数类型
interface SimplifiedParam {
  key: string;
  value: string;
}

// 定义 API 配置类型
interface ApiConfig {
  apiUrl: string;
  method: string;
  contentType: string;
  isCors: boolean;
  params?: ParamType[];
  id?: string;
  [key: string]: any;
}

// 定义请求选项类型
interface FetchOptions {
  method: string;
  headers: Record<string, string>;
  mode: RequestMode;
  body?: string | FormData;
  [key: string]: any;
}

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
    apiUrl: "",
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
  const handleApiTest = async () => {
    const apiConfig: ApiConfig = form.getFieldsValue();
    const { apiUrl, method, contentType, isCors, params } = apiConfig;

    try {
      // params为[{key: "333", value: {type: "static", value: "Dddd"}}],需要格式化
      const simplifiedParams: SimplifiedParam[] = params?.map(param => ({
        key: param.key,
        value: typeof param.value === 'object' ? param.value.value : param.value
      })) || [];

      // 构建请求配置
      const { url, options } = buildFetchConfig(
        apiUrl,
        method,
        contentType,
        isCors,
        simplifiedParams
      );

      console.log("请求 URL:", url);
      console.log("请求配置:", options);
      // 发送请求并处理响应
      const response = await fetch(url, options);
      await handleResponse(response, options.mode);
    } catch (error: any) {
      console.error("API 测试错误:", error);
      apiTestModalRef.current?.showModal({
        error: true,
        message: error.message || "请求失败"
      });
    }
  };

  // 构建 fetch 请求配置
  const buildFetchConfig = (
    apiUrl: string,
    method: string,
    contentType: string,
    isCors: boolean,
    params: SimplifiedParam[]
  ): { url: string; options: FetchOptions } => {
    const headers: Record<string, string> = contentType === "multipart/form-data"
      ? {}
      : { "Content-Type": contentType };

    const mode = isCors ? 'cors' : 'no-cors';
    const options: FetchOptions = { method, headers, mode };

    // 处理 URL 和请求体
    let url = apiUrl;
    if (method === "GET") {
      url = appendQueryParams(apiUrl, params);
    } else {
      const body = buildRequestBody(contentType, params);
      if (body) {
        options.body = body;
      }
    }

    return { url, options };
  };

  // 为 GET 请求添加查询参数
  const appendQueryParams = (url: string, params: SimplifiedParam[]): string => {
    if (!params.length) return url;

    const queryParams = new URLSearchParams();
    params.forEach(param => queryParams.append(param.key, param.value));
    return `${url}?${queryParams.toString()}`;
  };

  // 根据内容类型构建请求体
  const buildRequestBody = (
    contentType: string,
    params: SimplifiedParam[]
  ): string | FormData | undefined => {
    switch (contentType) {
      case "application/json": {
        const jsonBody: Record<string, string> = {};
        params.forEach(param => { jsonBody[param.key] = param.value; });
        return JSON.stringify(jsonBody);
      }

      case "multipart/form-data": {
        const formData = new FormData();
        params.forEach(param => formData.append(param.key, param.value));
        return formData;
      }

      case "application/x-www-form-urlencoded": {
        const urlParams = new URLSearchParams();
        params.forEach(param => urlParams.append(param.key, param.value));
        return urlParams.toString();
      }
    }
  };

  // 处理响应
  const handleResponse = async (response: Response, mode: string): Promise<void> => {
    if (!response.ok) {
      throw new Error(`请求失败: ${response.status} ${response.statusText}`);
    }

    if (mode === 'no-cors') {
      apiTestModalRef.current?.showModal({
        message: "请求已发送，但由于 no-cors 模式限制，无法读取响应内容"
      });
      return;
    }

    const data = await response.json();
    console.log("测试数据", data);
    apiTestModalRef.current?.showModal(data);
  };

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
