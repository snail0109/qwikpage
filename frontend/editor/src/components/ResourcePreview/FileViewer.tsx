import { useEffect, useMemo, useState } from "react";
import { Tooltip, Flex, Divider, theme, Table } from 'antd';
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { DeleteOutlined } from '@ant-design/icons';
import { useResource } from "@/context/resource";
import EditIcon from "@/assets/icons/EditIcon.svg?react";
import JSIcon from "@/assets/icons/js.svg?react";
import AttachIcon from "@/assets/icons/attach.svg?react";
import styles from './index.module.less';
import type { IResource } from '@/types';
import type { TableProps } from 'antd';

const { useToken } = theme;

interface IProps {
  resource_name: string;
  data: IResource[]
}

const JsPreviewer = (props: IProps) => {
  const { token } = useToken();
  const { resource_type, onEditResource, onDeleteResource } = useResource();
  const { data, resource_name } = props;

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
    }
  ]

  const handleEdit = () => {
    onEditResource(resource_name, name)
  }
  const handleDelete = () => {
    onDeleteResource(resource_name, name);
  }

  return (
    <div className={styles.jsPreview} key={resource_name}>
      <Table<IResource> size="small" pagination={false} scroll={{ x: 0, y: 180 }} columns={columns} dataSource={data} />
    </div>
  )
};

export default JsPreviewer;
