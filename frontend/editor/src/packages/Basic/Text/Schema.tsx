import TextSetting from '@/packages/components/TextSetting';

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
      label: '文本内容',
      name: 'text',
    },
    {
      type: 'Select',
      label: '隐藏过长文本',
      name: 'hiddenText', // TODO
      props: {
        options: [
          { value: '', label: '默认不处理' },
          { value: 'ellipsis', label: '多余的文本省略' },
          { value: 'break', label: '换行且强制截断单词和数字' },
          { value: 'wrap', label: '换行但保持单词和数字的完整性' },
          { value: 'nowrap', label: '始终不换行' },
        ],
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
  ],
  config: {
    // 组件默认属性值
    props: {
      text: '欢迎使用QwikPage设计器',
      type: '',
      hiddenText: '',
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
