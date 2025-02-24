import { useNavigate } from "react-router-dom";
import { Typography, Avatar, Dropdown, Tooltip, message } from "antd";
import { GlobalOutlined, MoreOutlined, SettingOutlined, FolderOpenOutlined, EyeOutlined, ExportOutlined } from "@ant-design/icons";
import type { MenuProps } from "antd";
import { openUrl } from '@tauri-apps/plugin-opener';
import { invoke } from "@tauri-apps/api/core";
import { IProject } from "@/types";
import styles from "./../page.module.less";
import problue from "@/assets/image/probg_blue.png";
import progreen from "@/assets/image/progb_green.png";
import propurple from "@/assets/image/probg_purple.png";
import prored from "@/assets/image/progb_red.png";
const { Paragraph } = Typography;

// 根据 themeColor 映射到相应的图片
const themeColorToImageMap: { [key: string]: string } = {
    blue: problue,
    green: progreen,
    purple: propurple,
    red: prored,
};

/**
 * 页面列表
 */

export default function Category({ list }: { list: IProject[] }) {
    const navigate = useNavigate();
    // 单击打开项目配置
    const handleOpenProject = (id: string) => {
        navigate(`/project/${id}/config`);
    };

    // 双击加载项目下子页面
    const handleOpenPages = (id: string) => {
        navigate(`/project/pages?projectId=${id}`);
    };

    // 导出项目代码
    const handleExportProjectCode = async (id: string) => {
        return await invoke<void>("export_project", { id: id });
    };

    // 卡片下拉项
    const items: MenuProps["items"] = [
        {
            key: "config",
            icon: <SettingOutlined />,
            label: "项目配置",
        },
        {
            key: "export",
            icon: <ExportOutlined />,
            label: "导出代码",
        },
        {
            key: "resource_mgr",
            icon: <ExportOutlined />,
            label: "静态资源管理",
        },
    ];

    // 环境跳转
    const onClick = (_key: string, id: string) => {
        if (_key === 'config') {
            return handleOpenProject(id);
        }
        if (_key === 'export') {
            return handleExportProjectCode(id).catch(res => {
                message.error(res);
            });
        }
        if (_key === 'resource_mgr') {
            return navigate(`/resources/${id}`);
        }
    };

    // 预览跳转
    const handlePreview = async (id: string) => {
        const previewUrl = `${import.meta.env.VITE_PREVIEW_URL}/project/${id}`;
        await openUrl(previewUrl)
    };

    // 项目列表
    return (
        <>
            <div className={styles.projectGrid}>
                {list.map((project) => {
                    const backgroundImage = themeColorToImageMap[project.themeColor];
                    return (
                        <div className={styles.projectCard} key={project.id}>
                            {/* 卡片头部 */}
                            <div
                                className={styles.cardHeader}
                                onClick={() => handleOpenProject(project.id)}
                                style={{
                                    backgroundImage: `url(${backgroundImage})`,
                                    backgroundSize: 'cover',
                                }}>
                                <h3 className={styles.cardTitle}>
                                    {project.name}
                                </h3>
                            </div>
                            {/* 卡片内容 */}
                            <div className={styles.cardContent} onClick={() => handleOpenPages(project.id)}>
                                <Paragraph className={styles.description}>{project.remark}</Paragraph>
                                <div className={styles.metaInfo} style={{ paddingTop: '5px' }}>
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
