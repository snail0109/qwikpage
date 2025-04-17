import { useRef } from 'react';
import { Button, Table, Divider, ConfigProvider } from 'antd';
import { FieldNumberOutlined, FieldStringOutlined, PlusOutlined } from '@ant-design/icons';
import VariableSetting from './VariableSetting';
import { PageVariable } from '@/packages/types';
import { usePageStore } from '@/stores/pageStore';
import { useProjectStore } from '@/stores/projectStore';
import type { TableProps } from "antd";
import styles from './index.module.less';

export default () => {
  const variableRef = useRef<{ open: (type: 'add' | 'edit',  variableType: 'project' | 'page', variable?: PageVariable) => void }>();
  // 页面组件
  const { variables: pageVariables, removeVariable } = usePageStore((state) => ({
    variables: state.page.pageData.variables,
    removeVariable: state.removeVariable,
  }));

  // 项目信息
  const { projectVariables, removeProVariable } = useProjectStore((state) => ({
    projectVariables: state.variables,
    removeProVariable: state.removeVariable,
  }));

  console.log("页面变量", pageVariables);
  console.log("项目变量", projectVariables);

  const columns = (variableType: 'project' | 'page'): TableProps<PageVariable>["columns"] => [
    {
      title: "名称",
      dataIndex: "name",
      key: "name",
      width: "20%",
      align: "left",
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
      width: "25%",
    },
    {
      title: "变量说明",
      dataIndex: "remark",
      key: "remark",
      width: "35%",
    },
    {
      title: "操作",
      key: "action",
      width: 120,
      align: "left",
      render: (_, row) => (
        <div style={{ display: 'flex', alignItems: "center" }}>
          <Button type="link" onClick={(event) => handleEdit(event, row, variableType)} style={{ paddingLeft: 0 }}>
            修改
          </Button>
          <Divider type="vertical" />
          <Button type="link" onClick={(event) => handleRemove(event, row.name, variableType)}>
            删除
          </Button>
        </div>
      ),
    },
  ];

  // 新增变量
  const handleAdd = (variableType: 'project' | 'page') => {
    variableRef.current?.open('add', variableType);
  };

  // 修改变量
  const handleEdit = (event: React.MouseEvent, item: PageVariable, variableType: 'project' | 'page') => {
    event.preventDefault();
    variableRef.current?.open('edit', variableType, item);
  };

  // 删除变量
  const handleRemove = (event: React.MouseEvent, name: string, variableType: 'project' | 'page') => {
    event.preventDefault();
    if (variableType === 'page') {
      removeVariable(name);
    } else {
      removeProVariable(name);
    }
  };

  return (
    <ConfigProvider
      theme={{
        components: {
          Table: {
            headerBorderRadius: 2,
          },
        },
      }}
    >
      <div className={styles.variableConfigHeader}>
        <Button type="link" icon={<PlusOutlined />} onClick={() => handleAdd('project')}>
          新增
        </Button>
      </div>
      <Table<PageVariable>
        size="small"
        columns={columns('project')} 
        dataSource={projectVariables}
        className={styles.variableConfigTable}
        pagination={false}
      />
      <div className={styles.pageVariableConfig}>
        <div className={styles.pageVariableConfigHeader}>
          <span style={{ fontWeight: "bold" }}>页面变量</span>
          <Button type="link" icon={<PlusOutlined />} onClick={() => handleAdd('page')}>
            新增
          </Button>
        </div>
        <Table<PageVariable>
          size="small"
          columns={columns('page')}
          dataSource={pageVariables}
          className={styles.variableConfigTable}
          pagination={false}
        />
      </div>
      <VariableSetting ref={variableRef} />
    </ConfigProvider>
  );
};
