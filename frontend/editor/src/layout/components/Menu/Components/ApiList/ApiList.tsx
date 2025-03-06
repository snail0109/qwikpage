import { useRef } from "react";
import { Button, Table, Tag, Divider } from "antd";
import { PlusOutlined, SettingOutlined } from "@ant-design/icons";
import type { TableProps } from "antd";
import SettingModal from "@/components/ApiConfig/components/SettingModal";
import { ApiType } from "@/packages/types";
import { usePageStore } from "@/stores/pageStore";
import InterceptorModal from "@/components/ApiConfig/components/InterceptorModal";
import styles from "./index.module.less";

export default () => {
  const modalRef = useRef<{ showModal: (data?: any) => void }>();
  const interceptorRef = useRef<{ showModal: (data?: any) => void }>();

  const columns: TableProps<ApiType>["columns"] = [
    {
      title: "名称",
      dataIndex: "name",
      key: "name",
      width: "40%",
      render: (_, row) => (
        <div className={styles.iconCol}>
          <Tag>
            {row.method.toUpperCase()}
          </Tag>
          {row.name}
        </div>
      ),
    },
    {
      title: "URL",
      dataIndex: "stgApi",
      key: "stgApi",
    },
    {
      title: "操作",
      key: "action",
      width: 120,
      align: "center",
      render: (event, row) => (
        <div style={{ display: 'flex', alignItems: "center" }}>
          <Button type="link" onClick={() => handleEdit(event, row)}>
            修改
          </Button>
          <Divider type="vertical" />
          <Button type="link" onClick={() => handleRemove(event, row.id)}>
            删除
          </Button>
        </div>
      ),
    },
  ];

  // 页面组件
  const { apis, removeApi } = usePageStore((state) => ({
    apis: state.page.pageData.apis,
    removeApi: state.removeApi,
  }));

  // 新增接口
  const handleAdd = () => {
    modalRef.current?.showModal();
  };

  // 修改接口
  const handleEdit = (event: React.MouseEvent, item: ApiType) => {
    event.preventDefault();
    modalRef.current?.showModal(item.id);
  };

  // 删除删除
  const handleRemove = (event: React.MouseEvent, id: string) => {
    event.preventDefault();
    removeApi(id);
  };

  // 新增全局拦截器
  const handleInterceptors = () => {
    interceptorRef.current?.showModal();
  };

  return (
    <>
      <div className={styles.apiConfigHeader}>
        <Button type="link" icon={<PlusOutlined />} onClick={() => handleAdd()}>
          新增
        </Button>
        <Divider type="vertical" />
        <Button type="link" icon={<SettingOutlined />} onClick={() => handleInterceptors()}>
          全局拦截器
        </Button>
      </div>

      <Table<ApiType> size="small" columns={columns} dataSource={Object.values(apis)} />
      {/* 接口设置 */}
      <SettingModal ref={modalRef}></SettingModal>
      {/* 拦截器设置 */}
      <InterceptorModal ref={interceptorRef}></InterceptorModal>
    </>
  );
};
