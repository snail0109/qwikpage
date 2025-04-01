import { memo, useEffect, useRef, useState } from "react";
import { Collapse, Form, Layout, Spin } from "antd";
import CreatePage, { CreatePageRef } from "@/components/CreatePage";
import SearchBar from "@/components/Searchbar/SearchBar";
import GroupTitle from "@/components/GroupTitle";
import ProjectCard from "./components/ProjectCard";
import styles from "./index.module.less";
import CreateProject from "@/components/CreateProject";
import CreateGroup from "@/components/CreateGroup";
import EmptyBox from "@/components/EmptyBox/EmptyBox";
import GroupCollapse from "@/components/GroupCollapse/GroupCollapse";
import { cmd_invoke } from "@/services/cmd_invoke";
import { message } from "@/utils/AntdGlobal";
import { IGroup } from "@/types";
import ExpandIcon from "@/assets/icons/ExpandIcon.svg?react";

function Category() {
    const [form] = Form.useForm();
    const [loading, setLoading] = useState(false);
    const [dataSource, setDataSource] = useState<IGroup[]>([]);
    const [activeKeys, setActiveKeys] = useState<string[]>([]);
    const createPageRef = useRef<CreatePageRef>();
    const createProjectRef = useRef<{ open: (type: string, groupId: string) => void }>();
    const createGroupRef = useRef<{ open: () => void }>();

    useEffect(() => {
        load_groups_with_projects();
    }, []);

    const load_groups_with_projects = (keyword?: string) => {
        setLoading(true);
        cmd_invoke("load_groups_with_projects", { keyword })
            .then((res) => {
                setDataSource(res.groups);
                setActiveKeys(res.groups.map((item: { id: string }) => item.id));
            })
            .finally(() => {
                setLoading(false);
            });
    };

    // 新建项目
    const handleCreate = (groupId: string) => {
        createProjectRef.current?.open("project", groupId);
    };

    // 新建项目分组
    const handleCreateGroup = () => {
        createGroupRef.current?.open();
    };

    // 删除分组
    const handleDeleteGroup = (groupId: string) => {
        setLoading(true);
        cmd_invoke("delete_group", { id: groupId })
            .then(() => {
                message.success("删除分组成功");
                load_groups_with_projects();
            })
            .finally(() => {
                setLoading(false);
            });
    };

    // 更新分组名称
    const updateGroupName = async (groupId: string, newName: string) => {
        try {
            const res = await cmd_invoke("edit_group", { id: groupId, groupName: newName });
            // 刷新当前修改的分组名
            setDataSource((pre) => pre.map((group) => (group.id === groupId ? { ...group, name: newName } : group)));
            return true;
        } catch (error) {
            message.error("修改失败,请重试");
            console.error("修改失败", error);
        }
        return false;
    };

    const search = () => {
        const keyword = form.getFieldValue("keyword");
        load_groups_with_projects(keyword);
    };

    // 折叠面板展开折叠
    const onChange = (key: string[]) => {
        setActiveKeys(key);
    };

    return (
        <Layout.Content
            className={styles.projectList}
            onDragOver={(e) => {
                // 阻止从操作系统向浏览器中拖拽文件时，浏览器默认行为
                e.preventDefault();
            }}
            onDrop={(e) => {
                // 阻止从操作系统向浏览器中拖拽文件时，浏览器默认行为
                e.preventDefault();
            }}
        >
            {/* 搜索工具条 */}
            <SearchBar
                showGroup={false}
                form={form}
                from={"分组"}
                submit={search}
                refresh={search}
                onCreate={handleCreate}
                onCreateGroup={handleCreateGroup}
            />

            <div className={styles.projectContent}>
                <Spin spinning={loading} size="large" tip="加载中...">
                    <Collapse
                        ghost
                        collapsible="icon"
                        // 设置默认展开所有项
                        activeKey={activeKeys}
                        onChange={onChange}
                        items={dataSource.map((item: any) => {
                            return {
                                key: item.id,
                                collapsible: item.projects.length <= 0 ? "disabled" : "icon",
                                label: (
                                    <GroupTitle
                                        groupItem={item}
                                        onCreate={handleCreate}
                                        onDelete={handleDeleteGroup}
                                        onUpdateGroup={updateGroupName}
                                    />
                                ),
                                children:
                                    item.projects.length <= 0 ? (
                                        <EmptyBox
                                            groupId={item.id}
                                            title="该分组下暂无项目，请新增项目"
                                            lastCharsCount={4}
                                            onCreate={handleCreate}
                                        />
                                    ) : (
                                        <ProjectCard list={item.projects} />
                                    ),
                            };
                        })}
                        expandIcon={({ isActive }) => (
                            <ExpandIcon
                                width={20}
                                height={20}
                                style={{
                                    transform: isActive ? "rotate(0deg)" : "rotate(-90deg)",
                                    transition: "transform 0.3s ease",
                                }}
                            />
                        )}
                    />
                    {/* <GroupCollapse
                        loading={loading}
                        dataSource={dataSource}
                        renderHeader={(item) => (
                            <GroupTitle groupItem={item} onCreate={handleCreate} onUpdateGroup={updateGroupName} />
                        )}
                        renderChildren={(item) => <ProjectCard list={item.projects} />}
                        renderemptyChildren={(id) => (
                            <EmptyBox
                                groupId={id}
                                title="该分组下暂无项目，请新增项目"
                                lastCharsCount={4}
                                onCreate={handleCreate}
                            />
                        )}
                    /> */}
                </Spin>
            </div>

            {/* 新建分组 */}
            <CreateGroup createRef={createGroupRef} update={search} />
            {/* 新建项目 */}
            <CreateProject createRef={createProjectRef} update={search} />
            {/* 新建页面 */}
            <CreatePage createRef={createPageRef} update={search} />
        </Layout.Content>
    );
}

export default memo(Category);
