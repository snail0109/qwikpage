import { memo, useEffect, useMemo, useRef, useState } from "react";
import { useLocation, useNavigate, useParams } from "react-router-dom";
import { Layout, Button, Space, Switch } from "antd";
import { SunOutlined, MoonFilled, SettingOutlined } from "@ant-design/icons";
import { usePageStore } from "@/stores/pageStore";
import styles from "./index.module.less";
import storage from "@/utils/storage";
import { invoke } from "@tauri-apps/api/core";
import { useOsInfo } from "@/utils/os";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { WindowControls } from "./WindowControls";

const appWebview = getCurrentWebviewWindow();

/**
 * 编辑器顶部组件
 */
const Header = memo(() => {
    const [pageFrom, setPageFrom] = useState("projects");
    const navigate = useNavigate();
    const { id } = useParams();
    const ref = useRef(null);
    const location = useLocation();
    const platform = useOsInfo();
    const [isFullscreen, setIsFullscreen] = useState(false);

    const MAC_PADDING_LEFT = 72;

    // 检查全屏状态的函数
    const checkFullscreen = async () => {
        const fullscreen = await appWebview.isFullscreen();
        setIsFullscreen(fullscreen);
    };

    const { mode, theme, setMode, setTheme } = usePageStore((state) => {
        return {
            page: state.page,
            mode: state.mode,
            theme: state.theme,
            setMode: state.setMode,
            setTheme: state.setTheme,
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
        return await invoke<void>("open_folder");
    };

    return (
        <>
            <Layout.Header
                data-tauri-drag-region
                className={styles.layoutHeader}
                style={{
                    paddingLeft: macStoplightsVisible ? MAC_PADDING_LEFT : undefined,
                    paddingRight: isMac ? '12px' : 0,
                    transition: "padding-left 0.3s ease", // 添加过渡效果
                }}
            >
                <div className={styles.logo} onClick={goHome}>
                    <img
                        src={`${theme === "dark" ? "/imgs/qwikpage-logo.svg" : "/imgs/qwikpage-logo.svg"}`}
                        width={20}
                    />
                    <span>QwikPage</span>
                </div>
                {/* 用户信息&发布&发布记录 */}
                <div className={styles.user}>
                    {/* 系统设置的按钮图标 */}
                    <SettingOutlined onClick={onOpenSettingClick} />
                    {!isMac && (
                        <div className={styles.divider}></div>
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
        </>
    );
});

export default Header;
