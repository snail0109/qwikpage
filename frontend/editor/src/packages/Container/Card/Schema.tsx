/**
 * 组件配置和属性值
 */
import { FormInstance } from 'antd';
import ActionSetting from '@/components/BulkAction/ActionSetting';
export default {
  // 组件属性配置JSON
  attrs: [
    {
      type: 'Title',
      label: '基础配置',
      key: 'basic',
    },
    {
      type: 'Switch',
      label: '显示边框',
      name: 'bordered',
    },
    {
      type: 'RadioGroupBtn',
      label: 'Card尺寸',
      name: 'size',
      props: {
        options: [
          { label: 'default', value: 'default' },
          { label: 'small', value: 'small' },
        ],
        defaultValue: 'default',
      },
    },
    {
      type: 'Switch',
      label: '显示头部',
      name: 'header',
    },
    {
      type: 'Variable',
      label: '头部标题',
      name: 'title',
    },
    // {
    //   type: 'Input',
    //   label: '头部按钮',
    //   name: ['extra', 'text'],
    // },
    {
      type: 'Actions',
      label: '头部按钮',
      render(form: FormInstance) {
        return <ActionSetting key="ActionSetting" form={form} />;
      },
    },
    // {
    //   type: 'Select',
    //   label: '按钮类型',
    //   name: ['extra', 'type'],
    //   props: {
    //     options: [
    //       { label: '默认', value: 'default' },
    //       { label: '主要', value: 'primary' },
    //       { label: '幽灵', value: 'ghost' },
    //       { label: '链接', value: 'link' },
    //       { label: '文本', value: 'text' },
    //     ],
    //   },
    // },
    {
      type: 'Variable',
      label: '封面',
      name: 'cover',
    },
    // {
    //   type: 'Switch',
    //   label: '悬浮效果',
    //   name: 'hoverable',
    // },
    // {
    //   type: 'Title',
    //   label: '按钮配置（右上角）',
    //   key: 'btnConfig',
    // },
    // {
    //   type: 'Switch',
    //   label: '显示危险',
    //   name: ['extra', 'danger'],
    // },
    // {
    //   type: 'Title',
    //   label: '封面配置',
    //   key: 'coverConfig',
    // },
    {
      type: 'Switch',
      label: '显示内容Meta',
      key: 'meta',
    },
    {
      type: 'Variable',
      label: 'Meta头像',
      name: ['meta', 'avatar'],
    },
    {
      type: 'Variable',
      label: 'Meta标题',
      name: ['meta', 'title'],
    },
    {
      type: 'Variable',
      label: 'Meta描述',
      name: ['meta', 'description'],
    },
  ],
  config: {
    // 组件默认属性值
    props: {
      title: {
        type: 'static',
        value: 'QwikPage搭建',
      },
      size: 'default',
      bordered: true,
      meta: {
        title: '低代码搭建平台',
        description: '全栈自研、低代码搭建、逻辑编排、权限控制',
      },
    },
    // 组件样式
    style: {},
  },
  // 组件事件
  events: [
    {
      value: 'onClick',
      name: '点击卡片事件',
    },
    {
      value: 'onClickMore',
      name: '点击更多事件',
    },
  ],
};
