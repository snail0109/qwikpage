import React, { lazy, useEffect, useState } from "react";
import { Outlet } from "react-router-dom";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";
import { ConfigProvider, Splitter } from "antd";
import { useShallow } from "zustand/react/shallow";
import { usePageStore } from "@/stores/pageStore";
import SpinLoading from "@/components/SpinLoading";
import Notice from "../components/Notice";
import "./index.less";
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

    const onTabChange = (tab: string) => {
        if ([PanelKey.CodingPanel, PanelKey.ApiList, PanelKey.Variable].includes(tab)) {
            setFullScreen(true);
        } else {
            setFullScreen(false);
        }
    };

    useEffect(() => {
        if (mode === "preview") {
            setSizes([0, "100%", 0]);
        } else {
            setSizes([266, window.innerWidth - 516, 250]);
        }
    }, [mode]);
    // 模式切换，会导致子组件重新渲染
    return (
        <DndProvider backend={HTML5Backend}>
            {/* 编辑器 */}
            <div style={{ height: "100vh" }}>
                <Notice />
                <ConfigProvider
                    theme={{
                        components: {
                            Splitter: {
                                colorFill: "#e8e9eb",
                                controlItemBgActive: "#7d33ff",
                                controlItemBgActiveHover: "#7d33ff",
                            },
                        },
                    }}
                >
                    <Splitter onResize={setSizes}>
                        {/* 菜单及其tab */}
                        <Splitter.Panel collapsible size={sizes[0]} min={266}>
                            <React.Suspense fallback={<SpinLoading />} >
                                <Menu onTabChange={onTabChange} />
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
