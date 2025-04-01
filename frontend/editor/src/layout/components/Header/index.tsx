import { memo, useEffect, useMemo, useRef, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { Layout, Button, message, Space, Select } from "antd";
import { SettingOutlined } from "@ant-design/icons";
import { usePageStore } from "@/stores/pageStore";
import styles from "./index.module.less";
import storage from "@/utils/storage";
import { invoke } from "@tauri-apps/api/core";
import { useOsInfo } from "@/utils/os";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { WindowControls } from "./WindowControls";
import { save } from "@tauri-apps/plugin-dialog";
import { projectService, pageService } from "@/services";
import Logo from "@/assets/icons/qwikpage-logo.svg?react";
import ExportIcon from "@/assets/icons/ExportIcon.svg?react";
import SaveIcon from "@/assets/icons/SaveIcon.svg?react";
import { PanelKey } from "@/constants/panelKeys";
import { ISystemSettingRef, SystemSetting } from "../SystemSetting";
import problue from "@/assets/image/header/headerT_blue.png";
import progreen from "@/assets/image/header/headerT_green.png";
import propurple from "@/assets/image/header/headerT_purple.png";
import prored from "@/assets/image/header/headerT_red.png";
import ExpandArrowIcon from "@/assets/icons/ExpandArrowIcon.svg?react";
import { EyeOutlined, SaveOutlined, LeftOutlined } from '@ant-design/icons';
import { openUrl } from '@tauri-apps/plugin-opener';

const appWebview = getCurrentWebviewWindow();

// 根据 themeColor 映射到相应的图片
const themeColorToImageMap: { [key: string]: string } = {
    blue: problue,
    green: progreen,
    purple: propurple,
    red: prored,
};

/**
 * 编辑器顶部组件
 */
const Header = memo(() => {
    const [pageFrom, setPageFrom] = useState("projects");
    const navigate = useNavigate();
    const location = useLocation();
    const searchParams = new URLSearchParams(location.search);
    const projectId = searchParams.get('projectId') || undefined;
    const platform = useOsInfo();
    const [isFullscreen, setIsFullscreen] = useState(false);
    const [headerStyle, setHeaderStyle] = useState<React.CSSProperties>({
        paddingLeft: undefined,
        paddingRight: '0',
        transition: "padding-left 0.3s ease",
    });
    const [saveLoading, setSaveLoading] = useState(false);
    const [exportLoading, setExportLoading] = useState(false);
    const [loading, setLoading] = useState(false);

    const settingRef = useRef<ISystemSettingRef>();

    const MAC_PADDING_LEFT = 72;

    // 检查全屏状态的函数
    const checkFullscreen = async () => {
        const fullscreen = await appWebview.isFullscreen();
        setIsFullscreen(fullscreen);
    };

    const { mode, theme, setMode, setTheme, page, savePageInfo, currentTab, isEdit, updateEditState, canvasWidth, updateCanvasWidth } = usePageStore((state) => {
        return {
            page: state.page,
            mode: state.mode,
            theme: state.theme,
            setMode: state.setMode,
            setTheme: state.setTheme,
            savePageInfo: state.savePageInfo,
            currentTab: state.currentTab,
            isEdit: state.isEdit,
            updateEditState: state.updateEditState,
            canvasWidth: state.canvasWidth,
            updateCanvasWidth: state.updateCanvasWidth,
        };
    });

    // 返回首页
    const goHome = () => {
        setMode("edit");
        // 点击Logo返回最近操作的列表，对用户友好
        const isProject = /projects\/\d+\/\w+/.test(location.pathname);
        const isPage = /editor\/[a-f0-9\\-]+\/(edit|publishHistory)/.test(location.pathname);
        if (isProject) return navigate("/projects");
        if (isPage) return navigate("/pages");
        navigate("/projects");
    };

    const macStoplightsVisible = useMemo(() => {
        // mac 是全屏 返回false
        return platform.osType === "macos" && !isFullscreen;
    }, [platform, isFullscreen]);

    const isMac = useMemo(() => {
        return platform.osType === "macos";
    }, [platform]);

    // 添加窗口事件监听器
    useEffect(() => {
        // 初始检查
        checkFullscreen();

        // 监听窗口进入或退出全屏的事件
        const unlisten = appWebview.listen("tauri://resize", () => {
            checkFullscreen();
        });

        // 清理事件监听器
        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    useEffect(() => {
        setPageFrom(location.pathname.slice(1));
    }, [location]);

    // 设置主题
    useEffect(() => {
        const isDark = storage.get("marsview-theme");
        if (isDark) {
            document.documentElement.setAttribute("data-theme", "dark");
        } else {
            document.documentElement.setAttribute("data-theme", "light");
        }
        setTheme(isDark ? "dark" : "light");
    }, []);

    // 退出预览模式
    const handleExitPreview = () => {
        setMode("edit");
    };

    const onOpenSettingClick = async () => {
        settingRef.current?.open();
    };

    useEffect(() => {
        const updateHeaderStyle = async () => {
            const baseStyle: React.CSSProperties = {
                paddingLeft: macStoplightsVisible ? MAC_PADDING_LEFT : undefined,
                paddingRight: isMac ? '12px' : 0,
                transition: "padding-left 0.3s ease",
            };

            if (['/project/pages', '/resources'].includes(location.pathname) && projectId) {
                const res = await projectService.getProjectDetail(projectId);
                const backgroundImage = themeColorToImageMap[res.themeColor];
                setHeaderStyle({
                    ...baseStyle,
                    backgroundImage: `url(${backgroundImage})`,
                    color: '#fff'
                });
                return;
            }

            setHeaderStyle(baseStyle);
        };

        updateHeaderStyle();
    }, [location.pathname, projectId, macStoplightsVisible, isMac]);

    // 判断是否显示DSL相关按钮
    const showDSLButtons = currentTab === PanelKey.CodingPanel;

    // 保存DSL的处理函数
    const handleSave = async (event: React.MouseEvent) => {
        event.stopPropagation();
        setSaveLoading(true);

        try {
            // 获取当前页面数据
            const value = JSON.parse(localStorage.getItem('current_dsl_content') || '{}');

            if (!value || !value.page) {
                message.error("页面数据格式异常，请检查重试");
                return;
            }

            const { name, remark, pageData } = value.page;
            const params = {
                id: page.id,
                name,
                remark,
                pageData: JSON.stringify({ ...pageData, variableData: {}, formData: {} }),
                projectId: page.projectId,
            };

            await pageService.updatePageData(params);
            savePageInfo({
                ...params,
                pageData: JSON.parse(params.pageData),
            });
            message.success("保存成功");
        } catch (error) {
            message.error("保存失败");
            console.error('保存失败:', error);
        } finally {
            setSaveLoading(false);
        }
    };

    // 导出DSL的处理函数
    const handleExport = async (event: React.MouseEvent) => {
        event.stopPropagation();
        setExportLoading(true);

        try {
            // 获取当前页面数据
            const value = JSON.parse(localStorage.getItem('current_dsl_content') || '{}');

            if (!value || !value.page) {
                message.error("页面数据格式异常，请检查重试");
                return;
            }

            const { name, remark, pageData } = value.page;
            const jsonData = {
                id: page.id,
                name,
                remark,
                pageData: JSON.stringify({ ...pageData, variableData: {}, formData: {} }),
            };

            // 弹出保存文件的对话框
            const filePath = await save({
                filters: [
                    {
                        name: "JSON",
                        extensions: ["json"],
                    },
                ],
            });

            if (filePath) {
                // 调用后端命令，将 JSON 数据写入文件
                await invoke("export_json", { filePath, jsonData });
                message.success("导出成功");
            }
        } catch (error) {
            message.error("导出失败");
            console.error('导出失败:', error);
        } finally {
            setExportLoading(false);
        }
    };

    // 修改画布尺寸
    const handleClickCanvas = (val: string) => {
        // 直接使用pageStore中的updateCanvasWidth方法
        updateCanvasWidth(val);
    };

    // 保存页面数据
    const savePageData = async () => {
        setLoading(true);
        try {
            await pageService.updatePageData({
                id: page.id,
                projectId: page.projectId,
                pageData: JSON.stringify({ ...page.pageData, variableData: {}, formData: {} }),
            });
            message.success('页面保存成功');
            updateEditState(false);
            setLoading(false);
        } catch (error) {
            message.error('页面保存失败');
            setLoading(false);
        }
    };

    // 预览页面
    const handlePreview = () => {
        const previewUrl = `${import.meta.env.VITE_PREVIEW_URL}/project/${page.projectId}${page.path}`;
        openUrl(previewUrl);
    }

    return (
        <>
            <Layout.Header
                data-tauri-drag-region
                className={styles.layoutHeader}
                style={headerStyle}
            >
                <div className={styles.logo} onClick={goHome} style={{ color: ['/project/pages', '/resources'].includes(location.pathname) ? '#fff' : '#000' }}>
                    <Logo style={{ color: ['/project/pages', '/resources'].includes(location.pathname) ? '#fff' : '#216EF7' }} />
                    <span>QwikPage</span>
                    {/\/editor\/[^/]+\/[^/]+\/edit/.test(location.pathname) && (
                        <>
                            <div className={styles.divider}></div>
                            <div
                                className={styles.pageName}
                                onClick={async (e) => {
                                    e.stopPropagation();
                                    console.log("点击了页面名称", page.projectId);
                                    try {
                                        const projectDetail = await projectService.getProjectDetail(page.projectId);
                                        navigate(`/project/pages?projectId=${page.projectId}&projectName=${encodeURIComponent(projectDetail.name)}`);
                                    } catch (error) {
                                        console.error("获取项目名称失败", error);
                                        navigate(`/project/pages?projectId=${page.projectId}`);
                                    }
                                }}
                            >
                                {page.name}
                            </div>
                        </>
                    )}
                </div>
                <div className={styles.user}>
                    {/* 仅在DSL页签时显示按钮 */}
                    {showDSLButtons && (
                        <>
                            <div className={styles.dslBtns}>
                                <Button
                                    icon={<SaveIcon />}
                                    type="text"
                                    iconPosition={'start'}
                                    size="small"
                                    loading={saveLoading}
                                    onClick={handleSave}
                                >
                                    保存
                                </Button>
                                <Button
                                    icon={<ExportIcon />}
                                    type="text"
                                    iconPosition={'start'}
                                    size="small"
                                    loading={exportLoading}
                                    onClick={handleExport}
                                >
                                    导出
                                </Button>
                            </div>
                            <div className={styles.divider}></div>
                        </>
                    )}
                    {/* 仅在组件页签时显示按钮 */}
                    {(!currentTab || currentTab === PanelKey.ComponentPanel) && (location.pathname.includes('/editor/')) && (
                        <>
                            <Space size={0} style={{ marginRight: -10 }}>
                                <Select
                                    variant="borderless"
                                    options={[
                                        { label: '1920px', value: '1920px' },
                                        { label: '1440px', value: '1440px' },
                                        { label: '1280px', value: '1280px' },
                                        { label: '1024px', value: '1024px' },
                                        { label: '960px', value: '960px' },
                                        { label: '自适应', value: 'auto' },
                                    ]}
                                    style={{ width: 85 }}
                                    value={canvasWidth}
                                    onChange={handleClickCanvas}
                                    suffixIcon={
                                        <ExpandArrowIcon
                                            width={12}
                                            height={12}
                                            style={{
                                                transform: "rotate(-180deg)",
                                            }}
                                        />
                                    }
                                />
                                <Button type="text" icon={<SaveOutlined />} onClick={savePageData} loading={loading} size="small" style={{ color: '#000' }}>
                                    保存
                                </Button>
                                <Button type="text" icon={<EyeOutlined />} onClick={handlePreview} size="small" style={{ color: '#000' }}>
                                    预览
                                </Button>
                            </Space>
                            <div className={styles.divider}></div>
                        </>
                    )}
                    {/* 系统设置的按钮图标 */}
                    <SettingOutlined onClick={onOpenSettingClick} />
                    {!isMac && (
                        <div className={styles.divider} style={{ marginRight: '-15px' }}></div>
                    )}
                    {/* 预览模式 */}
                    {mode === "preview" && (
                        <Button type="primary" onClick={handleExitPreview}>
                            退出预览
                        </Button>
                    )}
                    <WindowControls />
                </div>
            </Layout.Header>
            <SystemSetting settingRef={settingRef} />
        </>
    );
});

export default Header;
