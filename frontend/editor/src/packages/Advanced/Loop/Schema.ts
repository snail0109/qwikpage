/**
 * 组件配置和属性值
 */

export default {
    // 组件属性配置JSON
    attrs: [
        {
            type: "Input",
            label: "唯一性字段名",
            name: ["rowKey"],
            tooltip: "列表返回数据的唯一性字段名, 请确保该字段名在列表数据中唯一",
            props: {
                placeholder: "eg: id",
            },
        },
    ],
    config: {
        props: {
            rowKey: "id",
        },
        // 组件样式
        style: {
            backgroundColor: "#fff",
            padding: "20px",
        },
        events: [],
        api: {
            sourceType: "json",
            source: [
                {
                    name: "详情1",
                    id: 1,
                },
                {
                    name: "详情2",
                    id: 2,
                },
            ],
        },
    },
    // 组件事件
    events: [],
    // TODO 组件方法
    methods: [],
};
