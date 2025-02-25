import { memo, useEffect, useState } from "react";
import { useLocation, useNavigate, useParams } from "react-router-dom";
import { Layout, Button, Space, Switch } from "antd";
import { SunOutlined, MoonFilled, SettingOutlined } from "@ant-design/icons";
import { usePageStore } from "@/stores/pageStore";
import styles from "./index.module.less";
import storage from "@/utils/storage";
import { invoke } from "@tauri-apps/api/core";

/**
 * 编辑器顶部组件
 */
const Header = memo(() => {
    const [pageFrom, setPageFrom] = useState("projects");
    const navigate = useNavigate();
    const { id } = useParams();
    const location = useLocation();
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
        const isPage = /editor\/[a-f0-9\\-]+\/(edit|publishHistory)/.test(location.pathname)
        if (isProject) return navigate("/projects");
        if (isPage) return navigate("/pages");
        navigate("/projects");
    };

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
            <Layout.Header className={styles.layoutHeader}>
                <div className={styles.logo} onClick={goHome}>
                    <img
                        src={`${theme === "dark" ? "/imgs/qwikpage-logo.svg" : "/imgs/qwikpage-logo.svg"}`}
                        width={42}
                    />
                    <span>QwikPage</span>
                </div>
                {/* 用户信息&发布&发布记录 */}
                <div className={styles.user}>
                    {/* 系统设置的按钮图标 */}
                    <SettingOutlined onClick={onOpenSettingClick}/>
                    <Space>
                        <Switch
                            checkedChildren={<MoonFilled />}
                            unCheckedChildren={<SunOutlined />}
                            defaultChecked
                            checked={theme == "dark" ? true : false}
                            onChange={(val) => {
                                invoke("set_theme", { theme: val ? "dark" : "light" })
                                storage.set("marsview-theme", val);
                                setTheme(val ? "dark" : "light");
                                document.documentElement.setAttribute("data-theme", val ? "dark" : "light");
                            }}
                        />
                    </Space>

                    {/* 预览模式 */}
                    {mode === "preview" && (
                        <Button type="primary" onClick={handleExitPreview}>
                            退出预览
                        </Button>
                    )}
                </div>
            </Layout.Header>
        </>
    );
});

export default Header;
