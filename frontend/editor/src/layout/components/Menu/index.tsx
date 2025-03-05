import { lazy, Suspense, useState } from "react";
import { Col, Flex, Row, Space, Tabs, Tooltip } from "antd";
import { QuestionCircleOutlined, RightOutlined, LeftOutlined } from "@ant-design/icons";
import ComponentIcon from "@/assets/icons/Component.svg?react";
import OutlineIcon from "@/assets/icons/Outline.svg?react";
import InterfaceIcon from "@/assets/icons/Interface.svg?react";
import DSLIcon from "@/assets/icons/DSL.svg?react";
import VariableIcon from "@/assets/icons/Variable.svg?react";
import ComponentPanel from "./Components/ComponentPanel/ComponentPanel";
import SpinLoading from "@/components/SpinLoading";
import { PanelKey } from "@/constants/panelKeys";
import styles from "./index.module.less";

// 组件大纲
const OutlinePanel = lazy(() => import("./OutlinePanel"));
// 页面源码
const CodingPanel = lazy(() => import("./Components/CodingPanel/CodingPanel"));
// 接口列表
const ApiList = lazy(() => import("./Components/ApiList/ApiList"));
// 页面变量
const VariableList = lazy(() => import("./Components/Variable/VariableList"));
/**
 * 左侧面板类型
 */
const panels = [
    {
        key: PanelKey.ComponentPanel,
        icon: <ComponentIcon style={{ fontSize: 16 }} />,
        label: "组件",
        title: (
            <Space>
                <span>组件</span>
                <Tooltip title="无需拖拽，直接点击就能添加到画布中。">
                    <QuestionCircleOutlined />
                </Tooltip>
            </Space>
        ),
        children: () => {
            return <ComponentPanel />;
        },
    },
    {
        key: PanelKey.OutlinePanel,
        icon: <OutlineIcon style={{ fontSize: 16 }} />,
        label: "大纲",
        title: (
            <Space>
                <span>页面大纲</span>
                <Tooltip title="组件支持拖拽排序">
                    <QuestionCircleOutlined />
                </Tooltip>
            </Space>
        ),
        children: () => {
            return <OutlinePanel />;
        },
    },
    {
        key: PanelKey.CodingPanel,
        icon: <DSLIcon style={{ fontSize: 16 }} />,
        label: "DSL",
        title: "页面JSON",
        children: () => {
            return <CodingPanel />;
        },
    },
    {
        key: PanelKey.ApiList,
        icon: <InterfaceIcon style={{ fontSize: 16 }} />,
        label: "接口",
        title: "页面接口",
        children: () => {
            return <ApiList />;
        },
    },
    {
        key: PanelKey.Variable,
        icon: <VariableIcon style={{ fontSize: 16 }} />,
        label: "变量",
        title: "页面变量",
        children: () => {
            return <VariableList />;
        },
    },
];

/**
 * 生成左侧组件列表
 */

interface MenuProps {
    onTabChange: (tab: string) => void;
    onCollapse?: (collapsed: boolean) => void;
    collapsed?: boolean;
}

const Menu: React.FC<MenuProps> = (props) => {
    const { onTabChange, onCollapse, collapsed: externalCollapsed } = props;
    const [internalCollapsed, setInternalCollapsed] = useState(false);

    const collapsed = externalCollapsed !== undefined ? externalCollapsed : internalCollapsed;

    const handleCollapseToggle = () => {
        const newCollapsed = !collapsed;
        if (externalCollapsed === undefined) {
            setInternalCollapsed(newCollapsed);
        }
        if (onCollapse) {
            onCollapse(newCollapsed);
        }
    };

    return (
        <div className={`${styles.menuContainer} ${collapsed ? styles.collapsedMenu : ''}`}>
            <div className={styles.collapseButtonContainer} style={{ right: collapsed ? '-16px' : '-26px' }}>
                <div
                    className={styles.collapseButton}
                    onClick={handleCollapseToggle}
                    style={{
                        backgroundColor: collapsed ? '#1677ff' : undefined,
                        color: collapsed ? '#fff' : undefined
                    }}
                >
                    {collapsed ? <RightOutlined /> : <LeftOutlined />}
                </div>
            </div>

            <Tabs
                size={"small"}
                defaultActiveKey={panels[0].key}
                tabPosition="left"
                tabBarStyle={{
                    width: 50,
                    height: "100vh",
                    margin: collapsed ? 0 : undefined,
                    padding: collapsed ? 0 : undefined
                }}
                className={styles.leftTool}
                centered={true}
                items={panels.map((item) => {
                    return {
                        key: item.key,
                        label: (
                            <Flex vertical justify="center" align="center" gap={5}>
                                {item.icon}
                                <span style={{ fontSize: 12 }}>{item.label}</span>
                            </Flex>
                        ),
                        children: !collapsed ? (
                            <div className={styles.menuContent}>
                                <Row style={{ height: 36 }} align={"middle"} justify={"space-between"}>
                                    <Col>
                                        <span style={{ fontWeight: "bold" }}>{item.title}</span>
                                    </Col>
                                </Row>
                                <Suspense fallback={<SpinLoading />}>{item.children?.()}</Suspense>
                            </div>
                        ) : null,
                    };
                })}
                onChange={(key) => {
                    console.log("切换到标签:", key);
                    onTabChange(key);
                }}
            />
        </div>
    );
};

export default Menu;
