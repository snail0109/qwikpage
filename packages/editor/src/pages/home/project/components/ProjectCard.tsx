import { useNavigate } from "react-router-dom";
import { Typography, Avatar, Dropdown, Tooltip } from "antd";
import { GlobalOutlined, MoreOutlined, SettingOutlined, FolderOpenOutlined, EyeOutlined } from "@ant-design/icons";
import type { MenuProps } from "antd";
import { openUrl } from '@tauri-apps/plugin-opener';
import { Project } from "@/invokeApi/types";
import styles from "./../page.module.less";
const { Paragraph } = Typography;

/**
 * 页面列表
 */

export default function Category({ list }: { list: Project[] }) {
    const navigate = useNavigate();
    // 单击打开项目配置
    const handleOpenProject = (id: string) => {
        navigate(`/project/${id}/config`);
    };

    // 双击加载项目下子页面
    const handleOpenPages = (id: string) => {
        navigate(`/project/pages?projectId=${id}`);
    };

    // 卡片下拉项
    const items: MenuProps["items"] = [
        {
            key: "config",
            icon: <SettingOutlined />,
            label: "项目配置",
        },
    ];

    // 环境跳转
    const onClick = (_key: string, id: string) => {
        return handleOpenProject(id);
    };

    // 预览跳转
    const handlePreview = async (id: string) => {
        const previewUrl = `http://127.0.0.1:8000/project/${id}`;
        await openUrl(previewUrl)
    };

    // 项目列表
    return (
        <>
            <div className={styles.projectGrid}>
                {list.map((project) => {
                    return (
                        <div className={styles.projectCard} key={project.id}>
                            {/* 卡片头部 */}
                            <div className={styles.cardHeader} onClick={() => handleOpenProject(project.id)}>
                                <h3 className={styles.cardTitle}>
                                    <GlobalOutlined className={styles.cardIcon} />
                                    {project.name}
                                </h3>
                            </div>
                            {/* 卡片内容 */}
                            <div className={styles.cardContent} onClick={() => handleOpenPages(project.id)}>
                                <Paragraph className={styles.description}>{project.remark}</Paragraph>
                                <div className={styles.metaInfo}>
                                    <FolderOpenOutlined className={styles.metaIcon} />
                                    <p>
                                        <span>{project.count} </span>个页面
                                    </p>
                                </div>
                            </div>
                            {/* 卡片更多 */}
                            <div className={styles.moreInfo}>
                                <Dropdown
                                    menu={{ items, onClick: ({ key }) => onClick(key, project.id) }}
                                    arrow
                                    placement="bottomRight"
                                    trigger={["click"]}
                                >
                                    <MoreOutlined className={styles.moreIcon} />
                                </Dropdown>
                            </div>
                            {/* 卡片预览 */}
                            <div className={styles.moreInfo} style={{ right: 40 }}>
                                <Tooltip title="预览">
                                    <EyeOutlined className={styles.moreIcon} onClick={() => handlePreview(project.id)} />
                                </Tooltip>
                            </div>

                            {/* 项目Logo */}
                            <Avatar src={project.logo} className={styles.projectLogo} />
                        </div>
                    );
                })}
            </div>
        </>
    );
}
