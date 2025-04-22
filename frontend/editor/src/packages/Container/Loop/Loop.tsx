import { getComponent } from "@/packages";
import { usePageStore } from "@/stores/pageStore";
import { useDrop } from "react-dnd";
import { forwardRef, useContext, useEffect, useImperativeHandle, useState } from "react";
import { ComponentType, IDragTargetItem } from "@/packages/types";
import { useShallow } from "zustand/react/shallow";
import { handleApi } from "@/packages/utils/handleApi";
import LoopItemValueContext from "./LoopItemValueContext";
import MarsRender from "@/packages/MarsRender/MarsRender";

const Loop = ({ id, type, config, elements: childElements }: ComponentType, ref: any) => {
    const [dataItems, setDataItems] = useState([]);
    const [visible, setVisible] = useState(true);
    const contextValue = useContext(LoopItemValueContext);

    const { addChildElements, variableData } = usePageStore(
        useShallow((state) => ({
            addChildElements: state.addChildElements,
            formData: state.page.pageData.formData,
            setFormData: state.setFormData,
            variableData: state.page.pageData.variableData,
        }))
    );

    // 拖拽接收
    const [, drop] = useDrop({
        accept: "MENU_ITEM",
        async drop(item: IDragTargetItem, monitor) {
            if (monitor.didDrop()) return;
            // 生成默认配置
            const { config, events, methods = [] }: any = (await getComponent(item.type + "Config"))?.default || {};
            addChildElements({
                type: item.type,
                name: item.name,
                parentId: id,
                id: item.id,
                config,
                events,
                methods,
            });
        },
        // TODO: 拖拽组件时，容器呈现背景色（后期需要判断组件是否可以拖入）
        collect: (monitor) => ({
            isOver: monitor.isOver(),
            canDrop: monitor.canDrop(),
        }),
    });

    useEffect(() => {
        getDataList({});
    }, [config.api, config.api?.sourceType == "variable" ? variableData : ""]);

    // 列表加载
    const getDataList = async (params: any) => {
        try {
            const res = await handleApi(config.api, params);
            setDataItems(res.data);
        } catch (error) {
            setDataItems([]);
        }
    };

    // 暴露给外部的方法
    useImperativeHandle(ref, () => ({
        show() {
            setVisible(true);
        },
        hide() {
            setVisible(false);
        },
        search: async (searchQuery: any) => {
            await getDataList(searchQuery);
        },
        reload: async () => {
            await getDataList({});
        },
        clearData: () => {
            setDataItems([]);
        },
    }));

    return (
        visible && (
            <div style={config.style} data-id={id} data-type={type} ref={drop}>
                {childElements?.length > 0 ? (
                    dataItems.map((itemData, itemIndex) => (
                        <LoopItemValueContext.Provider
                            key={itemData[config.props.rowKey || "id"]}
                            value={{
                                ...contextValue,
                                [id]: {
                                    item: itemData,
                                    index: itemIndex,
                                },
                            }}
                        >
                            <MarsRender elements={childElements} />
                        </LoopItemValueContext.Provider>
                    ))
                ) : (
                    <div className="slots" style={{ lineHeight: "100px" }}>
                        拖拽组件到这里
                    </div>
                )}
            </div>
        )
    );
};

export default forwardRef(Loop);
