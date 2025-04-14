import React, { lazy, useEffect, useState } from "react";
import { Outlet, useNavigate } from "react-router-dom";
import { DndProvider } from "react-dnd";
import { HTML5Backend } from "react-dnd-html5-backend";
import { ConfigProvider, Splitter } from "antd";
import { useShallow } from "zustand/react/shallow";
import { usePageStore } from "@/stores/pageStore";
import SpinLoading from "@/components/SpinLoading";
import Notice from "../components/Notice";
import styles from "./index.module.less";
import { PanelKey } from "@/constants/panelKeys";
import { GlobalHotKeys } from 'react-hotkeys';
import { keyMap } from '@/constants/hotKeys';


const Menu = lazy(() => import("../components/Menu"));
const ConfigPanel = lazy(() => import("../components/ConfigPanel/ConfigPanel"));

// 左侧菜单宽度
const DEFAULT_LEFT_SIZE = 310;
// 配置面板宽度
const DEFAULT_CONFIG_SIZE = 260;
// 菜单固定宽度
const MENU_SIZE = 50;

/**
 * 编辑器布局组件
 */
const EditLayout = () => {
    const [sizes, setSizes] = useState<(number | string)[]>([DEFAULT_LEFT_SIZE, window.innerWidth - (DEFAULT_LEFT_SIZE + DEFAULT_CONFIG_SIZE), DEFAULT_CONFIG_SIZE]);
    const mode = usePageStore(useShallow((state) => state.mode));
    const [menuCollapsed, setMenuCollapsed] = useState(false);
    const [previousMenuCollapsed, setPreviousMenuCollapsed] = useState(false);
    const [isFullscreen, setIsFullscreen] = useState(false);
    // 添加当前选中的标签状态
    const { currentTab, setCurrentTab } = usePageStore(state => ({
        currentTab: state.currentTab,
        setCurrentTab: state.setCurrentTab
    }));
    // 拖拽后左侧宽度
    const [lastLeftWidth, setLastLeftWidth] = useState<number>(DEFAULT_LEFT_SIZE);
    // 拖拽后配置面板宽度
    const [lastConfigWidth, setLastConfigWidth] = useState<number>(DEFAULT_CONFIG_SIZE);

    const onTabChange = (tab: string) => {
        // 使用 store 的 setCurrentTab 代替本地状态
        console.log("EditLayout接收到的标签:", tab);
        setCurrentTab(tab);

        const isFullscreenTab = [PanelKey.CodingPanel, PanelKey.ApiList, PanelKey.Variable].includes(tab);

        if (isFullscreenTab) {
            // 切换到全屏模式
            // 保存当前菜单折叠状态
            setPreviousMenuCollapsed(menuCollapsed);
            // 如果左侧菜单是折叠状态，需要展开
            if (menuCollapsed) {
                setMenuCollapsed(false);
            }
            setSizes([window.innerWidth, 0, 0]);
            setIsFullscreen(true);
        } else {
            // 切换到非全屏模式

            // 只有在从全屏模式切换回来时才恢复之前保存的折叠状态
            if (isFullscreen) {
                setMenuCollapsed(previousMenuCollapsed);

                // 根据折叠状态设置合适的尺寸
                if (previousMenuCollapsed) {
                    setSizes([MENU_SIZE, window.innerWidth - (MENU_SIZE + lastConfigWidth), lastConfigWidth]);
                } else {
                    setSizes([lastLeftWidth, window.innerWidth - (lastLeftWidth + lastConfigWidth), lastConfigWidth]);
                }

                setIsFullscreen(false);
            } else {
                // 在非全屏标签之间切换，保持当前的折叠状态
                if (menuCollapsed) {
                    setSizes([MENU_SIZE, window.innerWidth - (MENU_SIZE + lastConfigWidth), lastConfigWidth]);
                } else {
                    setSizes([lastLeftWidth, window.innerWidth - (lastLeftWidth + lastConfigWidth), lastConfigWidth]);
                }
            }
        }
    };

    const onMenuCollapse = (collapsed: boolean) => {
        setMenuCollapsed(collapsed);
        if (collapsed) {
            const currentWidth = Number(sizes[0]);
            if (currentWidth > MENU_SIZE) {
                setLastLeftWidth(currentWidth);
            }
            // 折叠后左侧宽度
            setSizes([MENU_SIZE, Number(sizes[1]) + (Number(sizes[0]) - MENU_SIZE), sizes[2]]);
        } else {
            // 展开后左侧宽度
            setSizes([lastLeftWidth, Number(sizes[1]) - (lastLeftWidth - MENU_SIZE), sizes[2]]);
        }
    };

    const handleResize = (newSizes: (number | string)[]) => {
        setSizes(newSizes);
        if (!menuCollapsed && typeof newSizes[0] === 'number' && newSizes[0] > MENU_SIZE) {
            // 拖拽后左侧宽度
            setLastLeftWidth(newSizes[0]);
        }
        if (typeof newSizes[2] === 'number' && newSizes[2] > 0) {
            // 拖拽后配置面板宽度
            setLastConfigWidth(newSizes[2]);
        }
    };

    useEffect(() => {
        if (mode === "preview") {
            setSizes([0, "100%", 0]);
        } else {
            setSizes([DEFAULT_LEFT_SIZE, window.innerWidth - (DEFAULT_LEFT_SIZE + DEFAULT_CONFIG_SIZE), DEFAULT_CONFIG_SIZE]);
        }
    }, [mode]);

    const handlers = {
        'ESC': (e: KeyboardEvent | undefined) => {
            e?.preventDefault();
            e?.stopPropagation();
            console.log('esc');
            history.back();
        }
    }
    // 模式切换，会导致子组件重新渲染
    return (
        <GlobalHotKeys keyMap={keyMap} handlers={handlers} allowChanges={true}>
            <DndProvider backend={HTML5Backend}>
                {/* 编辑器 */}
                <div className={styles.editor} style={{ height: "calc(100vh - 32px)" }} >
                    {/* 使用说明 */}
                    {/* <Notice /> */}
                    <ConfigProvider
                        theme={{
                            token: {
                                borderRadiusLG: 4,
                            },
                            components: {
                                Splitter: {
                                    colorFill: "#e8e9eb",
                                    controlItemBgActive: "#1677ff",
                                    controlItemBgActiveHover: "#1677ff",
                                    splitBarSize: 1,
                                    controlItemBgHover: '#D9D9D9',
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
                                size={menuCollapsed ? MENU_SIZE : sizes[0]}
                                min={menuCollapsed ? MENU_SIZE : DEFAULT_LEFT_SIZE}
                                resizable={!menuCollapsed}
                                style={{
                                    overflow: 'visible',
                                    position: 'relative',
                                    paddingRight: menuCollapsed || currentTab === PanelKey.CodingPanel ? 0 : 10,
                                }}
                            >
                                <React.Suspense fallback={<SpinLoading />} >
                                    <Menu
                                        onTabChange={onTabChange}
                                        onCollapse={onMenuCollapse}
                                        collapsed={menuCollapsed}
                                    />
                                </React.Suspense>
                            </Splitter.Panel>
                            {/* 编辑器 */}
                            <Splitter.Panel size={sizes[1]}>
                                <Outlet />
                            </Splitter.Panel>
                            {/* 配置面板 */}
                            <Splitter.Panel collapsible size={sizes[2]} min={DEFAULT_CONFIG_SIZE}>
                                <React.Suspense fallback={<SpinLoading />}>
                                    <ConfigPanel />
                                </React.Suspense>
                            </Splitter.Panel>
                        </Splitter>
                    </ConfigProvider>
                </div>
            </DndProvider>
        </GlobalHotKeys>
    );
};

export default EditLayout;
