import React, { lazy, useEffect, useState } from "react";
import { Outlet } from "react-router-dom";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";
import { ConfigProvider, Splitter } from "antd";
import { useShallow } from "zustand/react/shallow";
import { usePageStore } from "@/stores/pageStore";
import SpinLoading from "@/components/SpinLoading";
import Notice from "../components/Notice";
import styles from "./index.module.less";
import { PanelKey } from "../components/Menu";

const Menu = lazy(() => import("../components/Menu"));
const ConfigPanel = lazy(() => import("../components/ConfigPanel/ConfigPanel"));
/**
 * 编辑器布局组件
 */
const EditLayout = () => {
    const [sizes, setSizes] = useState<(number | string)[]>([266, window.innerWidth - 640, 266]);
    const mode = usePageStore(useShallow((state) => state.mode));
    const [fullScreen, setFullScreen] = useState(false);
    const [menuCollapsed, setMenuCollapsed] = useState(false);
    const [lastMenuWidth, setLastMenuWidth] = useState<number>(270);

    const onTabChange = (tab: string) => {
        if ([PanelKey.CodingPanel, PanelKey.ApiList, PanelKey.Variable].includes(tab)) {
            setFullScreen(true);
        } else {
            setFullScreen(false);
        }
    };

    const onMenuCollapse = (collapsed: boolean) => {
        setMenuCollapsed(collapsed);
        if (collapsed) {
            const currentWidth = Number(sizes[0]);
            if (currentWidth > 50) {
                setLastMenuWidth(currentWidth);
            }
            setSizes([50, Number(sizes[1]) + (Number(sizes[0]) - 50), sizes[2]]);
        } else {
            setSizes([lastMenuWidth, Number(sizes[1]) - (lastMenuWidth - 50), sizes[2]]);
        }
    };

    const handleResize = (newSizes: (number | string)[]) => {
        setSizes(newSizes);
        if (!menuCollapsed && typeof newSizes[0] === 'number' && newSizes[0] > 50) {
            setLastMenuWidth(newSizes[0]);
        }
    };

    useEffect(() => {
        if (mode === "preview") {
            setSizes([0, "100%", 0]);
        } else {
            setSizes([270, window.innerWidth - 520, 250]);
        }
    }, [mode]);
    // 模式切换，会导致子组件重新渲染
    return (
        <DndProvider backend={HTML5Backend}>
            {/* 编辑器 */}
            <div className={styles.editor} style={{ height: "100vh" }} >
                <Notice />
                <ConfigProvider
                    theme={{
                        components: {
                            Splitter: {
                                colorFill: "#e8e9eb",
                                controlItemBgActive: "#1677ff",
                                controlItemBgActiveHover: "#1677ff",
                            },
                        },
                    }}
                >
                    <Splitter
                        onResize={handleResize}
                        style={{ gap: menuCollapsed ? 0 : undefined }}
                    >
                        {/* 菜单及其tab */}
                        <Splitter.Panel
                            size={menuCollapsed ? 50 : sizes[0]}
                            min={menuCollapsed ? 50 : 270}
                            resizable={!menuCollapsed}
                            style={{
                                overflow: 'visible',
                                position: 'relative',
                                paddingRight: menuCollapsed ? 0 : 10,
                            }}
                        >
                            <React.Suspense fallback={<SpinLoading />} >
                                <Menu onTabChange={onTabChange} onCollapse={onMenuCollapse} />
                            </React.Suspense>
                        </Splitter.Panel>
                        {!fullScreen && (
                            <>
                                {/* 编辑器 */}
                                <Splitter.Panel size={sizes[1]}>
                                    <Outlet></Outlet>
                                </Splitter.Panel>
                                {/* 配置面板 */}
                                <Splitter.Panel collapsible size={sizes[2]} min={250}>
                                    <React.Suspense fallback={<SpinLoading />}>
                                        <ConfigPanel />
                                    </React.Suspense>
                                </Splitter.Panel>
                            </>
                        )}
                    </Splitter>
                </ConfigProvider>
            </div>
        </DndProvider>
    );
};

export default EditLayout;
