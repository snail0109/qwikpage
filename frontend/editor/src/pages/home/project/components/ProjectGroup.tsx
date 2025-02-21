import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, Image, Card } from 'antd';
import { EditOutlined, DeleteOutlined, EyeOutlined } from '@ant-design/icons';
import { message, Modal } from '@/utils/AntdGlobal';
import { IGroup } from '@/types';
import ProjectCard from "./ProjectCard";
import styles from './../../index.module.less';

// 页面列表项
const ProjectGroup = ({ groupItem, onCreate }: { groupItem: IGroup; onCreate: (groupId?: string) => void }) => {
  const onCreatePro = () => {
    onCreate(groupItem.id);
  };

  return (
    <>
      <div key={groupItem.id} className={styles.group}>
        <div className={styles.groupHeader}>
          <div>
            {groupItem.name}
            <EditOutlined className={styles.editIcon} />
          </div>
          <Button color="primary" variant="link" onClick={onCreatePro}>
            新增项目
          </Button>
        </div>
        <ProjectCard list={groupItem.projects} />
      </div>
    </>
  );
};

export default ProjectGroup;
