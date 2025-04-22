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
        // 布局配置
        {
            type: 'Switch',
            label: '垂直布局',
            name: 'vertical',
          },
          {
            type: 'Select',
            label: '换行方式',
            name: 'wrap',
            props: {
              options: [
                { value: 'nowrap', label: '不换行' },
                { value: 'wrap', label: '换行' },
                { value: 'wrap-reverse', label: '逆换行' },
              ],
            },
          },
          {
            type: 'Select',
            label: '主轴对齐',
            name: 'justify',
            props: {
              options: [
                { value: 'flex-start', label: '左对齐' },
                { value: 'flex-end', label: '右对齐' },
                { value: 'center', label: '居中对齐' },
                { value: 'space-between', label: '两端对齐' },
                { value: 'space-around', label: '环绕对齐' },
                { value: 'space-evenly', label: '均匀对齐' },
              ],
            },
          },
          {
            type: 'Select',
            label: '副轴对齐',
            name: 'align',
            props: {
              options: [
                { value: 'start', label: '起点对齐' },
                { value: 'end', label: '终点对齐' },
                { value: 'center', label: '居中对齐' },
                { value: 'baseline', label: '文字基线对齐' },
                { value: 'stretch', label: '拉伸对齐' },
              ],
            },
          },
          {
            type: 'InputPx',
            label: '元素间隙',
            name: 'gap',
            props: {
              placeholder: 'eg: 10',
            },
          },
    ],
    config: {
        props: {
            rowKey: "id",
            wrap: 'nowrap',
            justify:"space-between",
            align: 'center',
            gap: 10,
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
