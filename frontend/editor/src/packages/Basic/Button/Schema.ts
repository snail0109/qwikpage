/**
 * 组件配置和属性值
 */
export default {
  // 组件属性配置JSON
  attrs: [
    {
      type: 'Input',
      label: '文本',
      name: ['text'],
    },
    {
      type: 'Select',
      label: '按钮类型',
      name: ['type'],
      props: {
        options: [
          { value: 'primary', label: '主要按钮' },
          { value: 'dashed', label: '虚线按钮' },
          { value: 'text', label: '文本按钮' },
          { value: 'default', label: '普通按钮' },
        ],
        defaultValue: 'default',
      },
    },
    // {
    //   type: 'Select',
    //   label: '按钮形状',
    //   name: ['shape'],
    //   props: {
    //     options: [
    //       { value: 'default', label: 'default' },
    //       { value: 'circle', label: 'circle' },
    //       { value: 'round', label: 'round' },
    //     ],
    //   },
    // },
    // {
    //   type: 'Switch',
    //   label: '块状按钮',
    //   name: ['block'],
    // },
    // {
    //   type: 'Switch',
    //   label: '幽灵按钮',
    //   name: ['ghost'],
    // },
    // {
    //   type: 'Switch',
    //   label: '危险按钮',
    //   name: ['danger'],
    // },
    {
      type: 'Icons',
      label: '图标',
      name: ['icon'],
    },
    {
      type: 'RadioGroupBtn',
      label: '图标位置',
      name: ['iconPosition'],
      props: {
        options: [
          { value: 'start', label: '左' },
          { value: 'end', label: '右' },
        ],
        defaultValue: 'start',
      },
    },
    {
      type: 'Input',
      label: '工具提示',
      name: ['tooltip'],
      props: {
        placeholder: '请输入工具提示',
      },
    },
    {
      type: 'Switch',
      label: '禁用',
      name: ['disabled'],
    },
    {
      type: 'Select',
      label: '尺寸',
      name: ['size'],
      props: {
        options: [
          { value: 'small', label: '小' },
          { value: 'middle', label: '中' },
          { value: 'large', label: '大' },
        ],
        defaultValue: 'middle',
      },
    },
  ],
  config: {
    // 组件默认属性值
    props: {
      type: 'primary',
      size: 'middle',
      text: '按钮',
      shape: 'default',
    },
    // 组件样式
    style: {},
    // 事件
    events: [],
  },
  // 组件事件
  events: [
    {
      value: 'onClick',
      name: '点击事件',
    },
  ],
  methods: [
    {
      name: 'startLoading',
      title: '开始loading',
    },
    {
      name: 'endLoading',
      title: '结束loading',
    },
  ],
};
