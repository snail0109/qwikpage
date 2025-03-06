import { useRef } from 'react';
import { Button, Table, Divider, ConfigProvider } from 'antd';
import { FieldNumberOutlined, FieldStringOutlined, PlusOutlined } from '@ant-design/icons';
import VariableSetting from './VariableSetting';
import { PageVariable } from '@/packages/types';
import { usePageStore } from '@/stores/pageStore';
import type { TableProps } from "antd";
import styles from './index.module.less';

export default () => {
  const variableRef = useRef<{ open: (type: 'add' | 'edit', variable?: PageVariable) => void }>();
  // 页面组件
  const { variables, removeVariable } = usePageStore((state) => ({
    variables: state.page.pageData.variables,
    removeVariable: state.removeVariable,
  }));

  console.log(variables);

  const columns: TableProps<PageVariable>["columns"] = [
    {
      title: "名称",
      dataIndex: "name",
      key: "name",
      width: "20%",
      align: "center",
      render: (_, row) => (
        <div className={styles.iconCol}>
          {row.type === 'string' ? <FieldStringOutlined /> : <FieldNumberOutlined />}
          {row.name}
        </div>
      ),
    },
    {
      title: "变量默认值",
      dataIndex: "defaultValue",
      key: "defaultValue",
      width: "30%",
    },
    {
      title: "变量说明",
      dataIndex: "remark",
      key: "remark",
      width: "30%",
    },
    {
      title: "操作",
      key: "action",
      width: 120,
      align: "center",
      render: (_, row) => (
        <div style={{ display: 'flex', alignItems: "center" }}>
          <Button type="link" onClick={(event) => handleEdit(event, row)}>
            修改
          </Button>
          <Divider type="vertical" />
          <Button type="link" onClick={(event) => handleRemove(event, row.name)}>
            删除
          </Button>
        </div>
      ),
    },
  ];

  // 新增变量
  const handleAdd = () => {
    variableRef.current?.open('add');
  };

  // 修改变量
  const handleEdit = (event: React.MouseEvent, item: PageVariable) => {
    event.preventDefault();
    variableRef.current?.open('edit', item);
  };

  // 删除变量
  const handleRemove = (event: React.MouseEvent, name: string) => {
    event.preventDefault();
    removeVariable(name);
  };

  return (
    <ConfigProvider
      theme={{
        components: {
          Table: {
            headerBg: '#EDEDED',
            headerBorderRadius: 2,
          },
        },
      }}
    >
      <div className={styles.variableConfigHeader}>
        <Button type="link" icon={<PlusOutlined />} onClick={() => handleAdd()}>
          新增
        </Button>
      </div>
      <Table<PageVariable>
        size="small"
        columns={columns}
        dataSource={variables}
        className={styles.variableConfigTable}
        pagination={false}
      />
      <div className={styles.pageVariableConfig}>
        <div className={styles.pageVariableConfigHeader}>
          <span style={{ fontWeight: "bold" }}>页面变量</span>
          <Button type="link" icon={<PlusOutlined />} onClick={() => handleAdd()}>
            新增
          </Button>
        </div>
        <Table<PageVariable>
          size="small"
          columns={columns}
          dataSource={variables}
          className={styles.variableConfigTable}
          pagination={false}
        />
      </div>
      <VariableSetting ref={variableRef} />
    </ConfigProvider>
  );
};
