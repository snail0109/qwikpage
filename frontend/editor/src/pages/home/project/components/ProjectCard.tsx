import { useNavigate } from "react-router-dom";
import { Typography, Avatar, Dropdown, Tooltip, message } from "antd";
import type { MenuProps } from "antd";
import { openUrl } from "@tauri-apps/plugin-opener";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { IProject } from "@/types";
import styles from "./index.module.less";
import projectCardStyle from "./index.module.less";
import problue from "@/assets/image/probg_blue.png";
import progreen from "@/assets/image/progb_green.png";
import propurple from "@/assets/image/probg_purple.png";
import prored from "@/assets/image/progb_red.png";
import SettingIcon from "@/assets/icons/SettingIcon.svg?react";
import ExportIcon from "@/assets/icons/ExportIcon.svg?react";
import ResourceIcon from "@/assets/icons/ResourceIcon.svg?react";
import CodeIcon from "@/assets/icons/CodeIcon.svg?react";
import MoreIcon from "@/assets/icons/MoreIcon.svg?react";
import BrowseIcon from "@/assets/icons/BrowseIcon.svg?react";
import FolderIcon from "@/assets/icons/FolderIcon.svg?react";

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
        const project = list.find((item) => item.id === id);
        if (project) {
            navigate(`/project/pages?projectId=${id}&projectName=${encodeURIComponent(project.name)}`);
        }
    };

    // 导出项目代码
    const handleExportProjectCode = async (project_id: string, export_type: string) => {
        return await invoke<void>("export_project", { params: { project_id, export_type } });
    };

    // 卡片下拉项
    const items: MenuProps["items"] = [
        {
            key: "config",
            icon: <SettingIcon />,
            label: "项目配置",
        },
        {
            key: "export",
            icon: <ExportIcon />,
            label: "导出代码",
            children: [
                {
                    key: "vue",
                    icon: <CodeIcon />,
                    label: "VUE",
                },
                {
                    key: "fishx",
                    icon: <CodeIcon />,
                    label: "FishX",
                },
                {
                    key: "fish",
                    icon: <CodeIcon />,
                    label: "Fish",
                },
            ],
        },
        {
            key: "resource_mgr",
            icon: <ResourceIcon />,
            label: "静态资源管理",
        },
    ];

    // 环境跳转
    const onClick = (_key: string, id: string) => {
        if (_key === "config") {
            return handleOpenProject(id);
        }
        if (["fishx", "vue", "fish"].includes(_key)) {
            return handleExportProjectCode(id, _key).then(res => {
                message.success("导出成功，请到本地查看");
            }).catch((error) => {
                message.error(error);
            });
        }
        if (_key === "resource_mgr") {
            const project = list.find((item) => item.id === id);
            if (project) {
                return navigate(`/resources?projectId=${id}&projectName=${encodeURIComponent(project.name)}`);
            }
            return null;
        }
    };

    // 预览跳转
    const handlePreview = async (id: string) => {
        const previewUrl = `${import.meta.env.VITE_PREVIEW_URL}/project/${id}`;
        await openUrl(previewUrl);
    };

    // 项目列表
    return (
        <>
            <div className={projectCardStyle.projectGrid}>
                {list.map((project) => {
                    const backgroundImage = themeColorToImageMap[project.themeColor];
                    const src = project.logo.includes("com.qwikpage.desktop/resources/project_logo")
                        ? convertFileSrc(project.logo)
                        : project.logo;
                    return (
                        <div className={projectCardStyle.projectCard} key={project.id}>
                            {/* 卡片头部 */}
                            <div
                                className={projectCardStyle.cardHeader}
                                onClick={() => handleOpenProject(project.id)}
                                style={{
                                    backgroundImage: `url(${backgroundImage})`,
                                    backgroundSize: "cover",
                                }}
                            >
                                <h3 className={projectCardStyle.cardTitle}>{project.name}</h3>
                            </div>
                            {/* 卡片内容 */}
                            <div className={projectCardStyle.cardContent} onClick={() => handleOpenPages(project.id)}>
                                <Paragraph className={styles.description}>{project.remark}</Paragraph>
                                <div className={projectCardStyle.metaInfo} style={{ paddingTop: "5px" }}>
                                    <FolderIcon className={projectCardStyle.metaIcon} />
                                    <p>
                                        <span>{project.count} </span>个页面
                                    </p>
                                </div>
                            </div>
                            {/* 卡片更多 */}
                            <div className={projectCardStyle.moreInfo}>
                                <Dropdown
                                    overlayClassName={styles.projectSetting}
                                    menu={{ items, onClick: ({ key }) => onClick(key, project.id) }}
                                    arrow
                                    placement="bottomRight"
                                    trigger={["click"]}
                                >
                                    <MoreIcon className={projectCardStyle.moreIcon} />
                                </Dropdown>
                            </div>
                            {/* 卡片预览 */}
                            <div className={projectCardStyle.moreInfo} style={{ right: 40 }}>
                                <Tooltip title="预览">
                                    <BrowseIcon
                                        className={projectCardStyle.moreIcon}
                                        onClick={() => handlePreview(project.id)}
                                    />
                                </Tooltip>
                            </div>

                            {/* 项目Logo */}
                            <Avatar src={src} className={projectCardStyle.projectLogo} />
                        </div>
                    );
                })}
            </div>
        </>
    );
}
