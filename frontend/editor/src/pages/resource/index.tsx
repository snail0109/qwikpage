import { ImageViewer } from "@/components/resourceViewers/ImageViewer";
import { resourceService } from "@/services";
import { appConfigDir, appDataDir, join } from "@tauri-apps/api/path";
import { Button, Divider, Flex, Form, Input, Layout, Space, Tooltip } from "antd";
import { set } from "lodash-es";
import { useEffect, useState } from "react";
import styles from "./index.module.less";
import searchBarstyles from "@/components/SearchBar/index.module.less";
import pageStyles from "@/pages/home/index.module.less";
import { RedoOutlined } from "@ant-design/icons";
import { useParams } from "react-router-dom";

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
    const [data, setData] = useState<[]>();
    const [activeTab, setActiveTab] = useState("图片");

    const [form] = Form.useForm();

    useEffect(() => {
        resourceService
            .load_resource({
                project_id: "a504d633-ac58-48bb-8b48-cff5f252df72",
                resouce_type: "img",
            })
            .then((res) => {
                setData(res);
                console.log(res);
            });
    }, []);

    const searchSubmit = () => {};
    const handleAddResGroup = () => {};

    const handleRefresh = () => {};

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
                            type={activeTab === tab.value ? "primary" : "default"}
                            className={styles.tabButton}
                            onClick={() => setActiveTab(tab.value)}
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
                        <Button icon={<RedoOutlined />} onClick={handleRefresh}></Button>
                    </Tooltip>
                </div>
            </div>
            <div className={styles.pagesContent}>Content</div>
        </Layout.Content>
    );
}
