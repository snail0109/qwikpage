import { ImageViewer } from "@/components/resourceViewers/ImageViewer";
import { resourceService } from "@/services";
import { appConfigDir, appDataDir, join } from "@tauri-apps/api/path";
import { Button, Divider, Flex, Layout, Space } from "antd";
import { set } from "lodash-es";
import { useEffect, useState } from "react";
import styles from "./index.module.less";

export default function Home() {
    const [path, setPath] = useState<string>("");
    const [activeTab, setActiveTab] = useState("图片");

    const tabs = ["图片", "字体", "第三方JS", "附件", "其它"];

    useEffect(() => {
        resourceService
            .load_resource({
                project_id: "a504d633-ac58-48bb-8b48-cff5f252df72",
                resouce_type: "img",
            })
            .then((res) => {
                console.log(res);
                setPath(res[0].path);
            });
    }, []);

    return (
        <Layout.Content>
            <div>Search Bar</div>
            <Divider />
            <Flex gap="small" className={styles.tabContainer}>
                {tabs.map((tab) => (
                    <Button
                        key={tab}
                        type={activeTab === tab ? "primary" : "default"}
                        className={styles.tabButton}
                        onClick={() => setActiveTab(tab)}
                    >
                        {tab}
                    </Button>
                ))}
            </Flex>
        </Layout.Content>
    );
}
