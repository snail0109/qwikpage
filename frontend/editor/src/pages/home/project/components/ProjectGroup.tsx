import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button, Input, Card } from 'antd';
import { EditOutlined, DeleteOutlined, EyeOutlined } from '@ant-design/icons';
import { message, Modal } from '@/utils/AntdGlobal';
import { cmd_invoke } from "@/services/cmd_invoke";
import { IGroup } from '@/types';
import styles from '@/styles/page.module.less';

interface ProjectGroupProps {
  groupItem: IGroup;
  onCreate: (groupId?: string) => void;
  onUpdateGroup: (groupId: string, newName: string) => void;
}

// 项目分组
const ProjectGroup = ({ groupItem, onCreate, onUpdateGroup }: ProjectGroupProps) => {
  const [isEditing, setIsEditing] = useState(false); // 是否正在编辑
  const [inputValue, setInputValue] = useState(groupItem.name); // 输入框的值

  // 处理编辑分组名称
  const handleEditGroup = async () => {
    try {
      const res = await cmd_invoke("edit_group", { id: groupItem.id, groupName: inputValue });
      console.log("修改成功", res);
      setIsEditing(false);
      // 刷新当前修改的分组名
      onUpdateGroup(groupItem.id, inputValue);
    } catch (error) {
      console.error("修改失败", error);
    }
  };

  // 处理输入框失焦
  const handleBlur = () => {
    handleEditGroup();
  };

  // 处理按下回车
  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleEditGroup();
    }
  };

  // 点击新增项目，调用父组件方法
  const onCreateProject = (event: any) => {
    onCreate(groupItem.id);
    // 阻止事件冒泡
    event.stopPropagation();
  };

  return (
    <>
      <div key={groupItem.id} className={styles.group}>
        <div className={styles.groupHeader}>
          <div>
            {isEditing ? (
              <Input
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
                onBlur={handleBlur}
                onKeyDown={handleKeyDown}
                autoFocus
              />
            ) : (
              <>
                {groupItem.name}
                <EditOutlined
                  className={styles.editIcon}
                  onClick={() => setIsEditing(true)}
                />
              </>
            )}
          </div>
          <Button color="primary" variant="link" onClick={onCreateProject}>
            新增项目
          </Button>
        </div>
      </div>
    </>
  );
};

export default ProjectGroup;
