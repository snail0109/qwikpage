import { useState } from "react";
import { Button, Input, Divider, Modal } from "antd";
import { DeleteOutlined, PlusOutlined } from "@ant-design/icons";
import styles from "./index.module.less";
import EditIcon from "@/assets/icons/EditIcon.svg?react";

interface ProjectGroupProps {
    groupItem: {
        id: string;
        name: string;
    };
    createText?: string;
    onCreate: (groupId: string) => void;
    onDelete: (groupId: string) => void;
    onUpdateGroup: (groupId: string, newName: string) => Promise<boolean>;
}

// 项目分组
const GroupTitle = ({ groupItem, createText = "新增项目", onCreate, onDelete, onUpdateGroup }: ProjectGroupProps) => {
    const [isEditing, setIsEditing] = useState(false); // 是否正在编辑
    const [inputValue, setInputValue] = useState(groupItem.name); // 输入框的值
    const [open, setOpen] = useState(false);

    // 处理编辑分组名称
    const handleEditGroup = async () => {
        const success = await onUpdateGroup(groupItem.id, inputValue);
        if (success) {
            setIsEditing(false);
        }
    };

    // 处理输入框失焦
    const handleBlur = (event: React.FocusEvent<HTMLInputElement>) => {
        handleEditGroup();
        event.stopPropagation();
    };

    // 处理按下回车
    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === "Enter") {
            handleEditGroup();
            e.stopPropagation();
        }
    };

    // 点击新增项目，调用父组件方法
    const onCreateProject = (event: any) => {
        onCreate(groupItem.id);
        // 阻止事件冒泡
        event.stopPropagation();
    };

    // 删除分组
    const handleDelete = (event: any) => {
        // 阻止事件冒泡
        event.stopPropagation();
        onDelete(groupItem.id);
    };

    const handleCancel = () => {
        setOpen(false);
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
                                onClick={(event) => {
                                    event.stopPropagation();
                                }}
                                autoFocus
                            />
                        ) : (
                            <div style={{ display: "flex", alignItems: "center" }}>
                                {groupItem.name}
                                <EditIcon
                                    className={styles.editIcon}
                                    onClick={(event) => {
                                        setIsEditing(true);
                                        event.stopPropagation();
                                    }}
                                />
                                {groupItem.id === "-1" ? null : (
                                    <>
                                        <Divider type="vertical" />
                                        <DeleteOutlined
                                            onClick={(event) => {
                                                setOpen(true);
                                                event.stopPropagation();
                                            }}
                                        />
                                        <Modal title="删除分组" open={open} onOk={handleDelete} onCancel={handleCancel}>
                                            <p>确定删除当前分组？</p>
                                        </Modal>
                                    </>
                                )}
                            </div>
                        )}
                    </div>
                    <Button icon={<PlusOutlined />} color="primary" variant="dashed" onClick={onCreateProject}>
                        {createText}
                    </Button>
                </div>
            </div>
        </>
    );
};

export default GroupTitle;
