import { resourceService } from "@/services";
import { appConfigDir, appDataDir, join } from "@tauri-apps/api/path";
import { Button, Divider, Flex, Form, Input, Layout, Space, Tooltip } from "antd";
import { set } from "lodash-es";
import { useEffect, useRef, useState } from "react";
import styles from "./index.module.less";
import searchBarstyles from "@/components/SearchBar/index.module.less";
import pageStyles from "@/pages/home/index.module.less";
import { RedoOutlined } from "@ant-design/icons";
import { useParams } from "react-router-dom";
import CreateGroup, { IOpenParams } from "./components/CreateGroup";
import ResourceGroupList, { IResourceGroup } from "./components/ResourceGroupList";

const tabs = [
    {
        label: "图片",
        value: "img",
    },
    {
        label: "字体",
        value: "font",
    },
    {
        label: "第三方JS",
        value: "js",
    },
    {
        label: "附件",
        value: "attachment",
    },
    {
        label: "其它",
        value: "other",
    },
];

export default function Home() {
    const { projectId: project_id } = useParams();
    const [data, setData] = useState<IResourceGroup[]>([]);
    const [resource_type, setResourceType] = useState("img");
    const [loading, setLoading] = useState(true);

    const createGroupRef = useRef<{ open: (params: IOpenParams) => void }>();

    const [form] = Form.useForm();

    useEffect(() => {
        setLoading(true);
        resourceService
            .load_resource({
                project_id: project_id!,
                resource_type,
            })
            .then((res) => {
                setData(res);
                console.log(res);
                setLoading(false);
            })
            .catch((err) => {
                console.log(err);
                setLoading(false);
            });
    }, []);

    const searchSubmit = () => {};

    // 新建资源分组
    const handleAddResGroup = () => {
        createGroupRef.current?.open({ action: "create" });
    };

    const handleEditResGroup = (group_name: string) => {
        createGroupRef.current?.open({
            action: "edit",
            group_name,
        });
    };

    const refresh = () => {
        const keyword = form.getFieldValue("keyword");
        resourceService
            .load_resource({
                // @ts-ignore
                project_id: project_id,
                resource_type,
                keyword,
            })
            .then((res) => {
                setData(res);
                console.log(res);
            });
    };

    return (
        // TODO 抽取公共组件
        <Layout.Content className={pageStyles.pageList}>
            {/* TODO SearchBar 组件替换 */}
            <div className={searchBarstyles.searchBar} style={{ justifyContent: "space-between" }}>
                <div>项目名称</div>
                <div className={searchBarstyles.searchBarForm}>
                    <Form form={form} layout="inline" initialValues={{ type: 1 }}>
                        <Form.Item name="keyword" style={{ width: 200 }}>
                            <Input placeholder="请输入查找关键字" onPressEnter={searchSubmit} />
                        </Form.Item>
                        <Form.Item>
                            <Space>
                                <Button type="primary" onClick={searchSubmit} size="middle">
                                    搜索
                                </Button>
                            </Space>
                        </Form.Item>
                    </Form>
                </div>
            </div>
            <div style={{ width: "100%", display: "flex", justifyContent: "space-between" }}>
                {/* TODO 按照设计稿实现 */}
                <div>
                    {tabs.map((tab) => (
                        <Button
                            key={tab.value}
                            type={resource_type === tab.value ? "primary" : "default"}
                            className={styles.tabButton}
                            onClick={() => setResourceType(tab.value)}
                        >
                            {tab.label}
                        </Button>
                    ))}
                </div>
                <div>
                    <Button type="primary" onClick={handleAddResGroup}>
                        创建分组
                    </Button>
                    <Tooltip title="刷新">
                        <Button icon={<RedoOutlined />} onClick={refresh}></Button>
                    </Tooltip>
                </div>
            </div>
            <div className={styles.pagesContent}>
                <ResourceGroupList
                    refresh={refresh}
                    data={data}
                    project_id={project_id!}
                    resource_type={resource_type}
                    handleEditResGroup={handleEditResGroup}
                />
            </div>
            <CreateGroup
                createRef={createGroupRef}
                update={refresh}
                project_id={project_id!}
                resource_type={resource_type}
            />
        </Layout.Content>
    );
}
