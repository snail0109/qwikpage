import { memo, useEffect, useRef, useState } from "react";
import { Collapse, Spin } from "antd";
import { IGroup } from "@/types";
import ExpandIcon from "@/assets/icons/ExpandIcon.svg?react";

interface GroupCollapseProps {
    loading: boolean;
    dataSource: Array<IGroup>;
    renderChildren: (item: any) => React.ReactNode; 
    renderHeader: (item: any) => React.ReactNode;
    renderemptyChildren: (groupId: any) => React.ReactNode; 
}

const GroupCollapse = ({
    loading,
    dataSource,
    renderHeader,
    renderChildren,
    renderemptyChildren
}: GroupCollapseProps) => {
    const [activeKeys, setActiveKeys] = useState<string[]>([]);

    useEffect(() => {
        setActiveKeys(dataSource.map((item: { id: string }) => item.id));
    }, [dataSource]);


    // 折叠面板展开折叠
    const handleChange = (key: string[]) => {
      setActiveKeys(key);
  };

    return (
        <Spin spinning={loading} size="large" tip="加载中...">
            <Collapse
                ghost
                activeKey={activeKeys}
                onChange={handleChange}
                expandIcon={({ isActive }) => (
                    <ExpandIcon
                        width={20}
                        height={20}
                        style={{
                            transform: isActive ? "rotate(0deg)" : "rotate(-90deg)",
                            transition: "transform 0.3s ease",
                        }}
                    />
                )}
            >
                {dataSource.map((item) => (
                    <Collapse.Panel key={item.id} header={renderHeader(item)}>
                        {item.projects.length <= 0 ? renderemptyChildren(item.id) : renderChildren(item)}
                    </Collapse.Panel>
                ))}
            </Collapse>
        </Spin>
    );
};

export default GroupCollapse;
