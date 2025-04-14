/**
 * 组件配置和属性值
 */

export default {
  // 组件属性配置JSON
  attrs: [
    {
      type: 'Title',
      label: '基础配置',
      key: 'basic',
    },
    {
      type: 'Variable',
      label: '文本',
      name: 'text',
      props: {
        placeholder: '请输入文本内容',
      },
    },
    {
      type: 'Input',
      label: '工具提示',
      name: ['formItem', 'tooltip'],
      props: {
        placeholder: '请输入工具提示',
      },
    },
    {
      type: 'RadioGroupBtn',
      label: '连接类型',
      name: ['type'],
      props: {
        options: [
          { value: 'redirect', label: '页面跳转' },
          { value: 'download', label: '文件下载' },
        ],
        defaultValue: 'redirect',
      },
    },
    {
      type: 'Input',
      label: '链接地址',
      name: 'href',
      props: {
        placeholder: '请输入',
      },
    },
    {
      type: 'Select',
      label: '链接打开方式',
      name: 'target',
      props: {
        options: [
          { value: '_self', label: '当前窗口' },
          { value: '_blank', label: '新窗口' },
          { value: '_parent', label: '父级窗口' },
          { value: '_top', label: '顶级窗口' },
        ],
      },
    },
    {
      type: 'Switch',
      label: '禁用',
      name: 'disabled',
    },
  ],
  config: {
    // 组件默认属性值
    props: {
      text: '欢迎使用QwikPage设计器',
      href: '',
      target: '_blank',
    },
    style: {},
    events: [],
    api: {},
    source: '',
  },
  // 组件事件
  events: [
    {
      value: 'onClick',
      name: '点击事件',
    },
  ],
  // 组件接口
  api: {},
};
