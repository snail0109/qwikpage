/**
 * 组件配置和属性值
 */

export default {
    // 组件属性配置JSON
    attrs: [
        {
            type: "Input",
            label: "rowKey",
            name: ["rowKey"],
            tooltip: "建议把列表返回的唯一值设置为rowKey",
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
