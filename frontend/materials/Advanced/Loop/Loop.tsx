import { forwardRef, useContext, useEffect, useImperativeHandle, useState } from "react";
import { handleApi } from "@materials/utils/handleApi";
import LoopItemValueContext from "./LoopItemValueContext";
import { usePageStore } from "@materials/stores/pageStore";
import { ComponentType } from "@materials/types";
import MarsRender from "@materials/MarsRender/MarsRender";
import { Flex } from "antd";

const Loop = ({ id, type, config, elements: childElements }: ComponentType, ref: any) => {
    const [dataItems, setDataItems] = useState([]);
    const [visible, setVisible] = useState(true);
    const contextValue = useContext(LoopItemValueContext);
    const { rowKey, ...restLayout } = config.props

    const variableData = usePageStore((state) => state.page.pageData.variableData);

    useEffect(() => {
        getDataList({});
    }, [config.api, config.api?.sourceType == "variable" ? variableData : ""]);

    // 列表加载
    const getDataList = async (params: any) => {
        try {
            const res = await handleApi(config.api, params);
            if (!Array.isArray(res.data)) {
                setDataItems([]);
            } else {
                setDataItems(res.data);
            }
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
            <div style={config.style} data-id={id} data-type={type}>
                {childElements?.length > 0 && (
                    <Flex style={config.style} {...restLayout}>
                        {dataItems.map((itemData, itemIndex) => (
                            <LoopItemValueContext.Provider
                                key={itemData[rowKey || "id"]}
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
                        ))}
                    </Flex>
                )}
            </div>
        )
    );
};

export default forwardRef(Loop);
