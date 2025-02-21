import { memo, useEffect, useRef, useState } from "react";
import { Button, Empty, Form, Layout, Spin } from "antd";
import { PlusOutlined } from "@ant-design/icons";
import CreatePage, { CreatePageRef } from "@/components/CreatePage";
import SearchBar from "@/components/Searchbar/SearchBar";
import ProjectGroup from "./components/ProjectGroup";
import styles from "./../index.module.less";
import CreateProject from "@/components/CreateProject";
import CreateGroup from "@/components/CreateGroup";
import { cmd_invoke } from "@/services/cmd_invoke";

function Category() {
    const [form] = Form.useForm();
    const [loading, setLoading] = useState(false);
    const [dataSource, setDataSource] = useState([]);
    const createPageRef = useRef<CreatePageRef>();
    const createProjectRef = useRef<{ open: (type: string, groupId?: string) => void }>();
    const createGroupRef = useRef<{ open: () => void }>();

    useEffect(() => {
        load_groups_with_projects();
    }, []);

    const load_groups_with_projects = (keyword?: string) => {
        setLoading(true);
        cmd_invoke("load_groups_with_projects", { keyword })
            .then((res) => {
                console.log("load_groups_with_projects", res);
                setDataSource(res.groups);
            })
            .finally(() => {
                setLoading(false);
            });
    };

    // 新建项目或页面
    const handleCreate = (groupId?: string) => {
        createProjectRef.current?.open("project", groupId);
    };

    // 新建项目分组
    const handleCreateGroup = () => {
        createGroupRef.current?.open();
    };

    const search = () => {
        const keyword = form.getFieldValue("keyword");
        load_groups_with_projects(keyword)
    };

    return (
        <Layout.Content className={styles.pageList}>
            {/* 搜索工具条 */}
            <SearchBar
                showGroup={false}
                form={form}
                from={"项目"}
                submit={search}
                refresh={search}
                onCreate={handleCreate}
                onCreateGroup={handleCreateGroup}
            />

            <div className={styles.pagesContent}>
                <Spin spinning={loading} size="large" tip="加载中...">
                    {
                        dataSource.map((item: any) => {
                            return (
                                <ProjectGroup groupItem={item} onCreate={handleCreate} />
                            );
                        })
                    }
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
