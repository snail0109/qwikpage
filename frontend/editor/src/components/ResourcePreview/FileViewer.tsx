import { Tooltip, theme, Table, Button } from 'antd';
import { invoke } from "@tauri-apps/api/core";
import { DeleteOutlined, EyeOutlined } from '@ant-design/icons';
import { useResource } from "@/context/resource";
import JSIcon from "@/assets/icons/js.svg?react";
import AttachIcon from "@/assets/icons/attach.svg?react";
import styles from './index.module.less';
import type { IResource } from '@/types';
import type { TableProps } from 'antd';

const { useToken } = theme;

interface IProps {
  resource_name: string;
  resource_path: string;
  data: IResource[]
}

const JsPreviewer = (props: IProps) => {
  const { token } = useToken();
  const { resource_type, onEditResource, onDeleteResource } = useResource();
  const { data, resource_name, resource_path } = props;

  const columns: TableProps<IResource>['columns'] = [
    {
      title: () => {
        return <span className={styles.iconHeader}>名称</span>
      },
      dataIndex: 'name',
      key: 'name',
      render: (_, row) => (
        <div className={styles.iconCol}>
          {resource_type === 'js' ? <JSIcon /> : <AttachIcon />}
          {row.name}
        </div>
      )
    },
    {
      title: '大小',
      dataIndex: 'file_size',
      key: 'file_size',
      minWidth: 200
    },
    {
      title: '时间',
      dataIndex: 'last_modified_time',
      key: 'last_modified_time',
      width: 200
    },
    {
      title: '操作',
      key: 'action',
      width: 80,
      align: 'center',
      render: (_, row) => (
        <span className={styles.actionTool}>
          <Tooltip title="查看">
            <Button type="link" icon={<EyeOutlined />} onClick={() => handleCheck(row.path)}></Button>
          </Tooltip>
          <Tooltip title="删除">
            <Button type="link" icon={<DeleteOutlined />} onClick={() => handleDelete(row.name)}></Button>
          </Tooltip>
        </span>
      )
    }
  ]

  const handleEdit = () => {
    onEditResource(resource_name, name)
  }
  const handleCheck = async (path: string) => {
    return await invoke<void>("open_target_folder", { path: resource_path });
  }
  const handleDelete = (name: string) => {
    onDeleteResource(resource_name, name);
  }

  return (
    <div className={styles.jsPreview} key={resource_name}>
      <Table<IResource> size="small" pagination={false} scroll={{ x: 0, y: 180 }} columns={columns} dataSource={data} />
    </div>
  )
};

export default JsPreviewer;
