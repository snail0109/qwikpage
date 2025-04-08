/**
 * 组件配置和属性值
 */
import { FormInstance } from 'antd';
import RulesSetting from '../../components/RulesSetting';
export default {
  attrs: [
    {
      type: 'Title',
      label: '标签配置',
      key: 'title',
    },
    {
      type: 'Input',
      label: '标题',
      name: ['formItem', 'label'],
    },
    // {
    //   type: 'Input',
    //   label: '字段',
    //   name: ['formItem', 'name'],
    // },
    {
      type: 'Variable',
      label: '数据源',
      name: ['options'],
    },
     {
      type: 'Input',
      label: '文本显示字段',
      name: ['field', 'label'],
    },
    {
      type: 'Input',
      label: '值字段',
      name: ['field', 'value'],
    },
    {
      type: 'Variable',
      label: '选中值',
      name: ['value'],
    },
    {
      type: 'Input',
      label: '工具提示',
      name: ['formItem', 'tooltip'],
      props: {
        placeholder: '请输入工具提示',
      },
    },
    // {
    //   type: 'Variable',
    //   label: '默认值',
    //   name: ['defaultValue'],
    //   tooltip: '多选以逗号分隔，如：1,2',
    // },
    // {
    //   type: 'Switch',
    //   label: '无样式',
    //   name: ['formItem', 'noStyle'],
    // },
    {
      type: 'Switch',
      label: '禁用',
      name: ['formWrap', 'disabled'],
    },
    // {
    //   type: 'Input',
    //   label: 'Extra',
    //   name: ['formItem', 'extra'],
    //   tooltip: '表单控件下方显示的提示信息',
    // },
    // {
    //   type: 'Input',
    //   label: 'Tooltip',
    //   name: ['formItem', 'tooltip'],
    //   tooltip: '表单项后面显示的提示信息',
    // },
    // {
    //   type: 'Title',
    //   label: '布局',
    //   key: 'FormLayout',
    // },
    {
      type: 'InputNumber',
      label: '标签栅格布局',
      name: ['formItem', 'labelCol', 'span'],
      props: {
        placeholder: '请输入',
      },
    },
    // {
    //   type: 'InputNumber',
    //   label: '标签偏移',
    //   name: ['formItem', 'labelCol', 'offset'],
    //   props: {
    //     placeholder: '偏移数',
    //   },
    // },
    {
      type: 'InputNumber',
      label: '控件栅格布局',
      name: ['formItem', 'wrapperCol', 'span'],
      props: {
        placeholder: '请输入',
      },
    },
    // {
    //   type: 'InputNumber',
    //   label: '控件偏移',
    //   name: ['formItem', 'wrapperCol', 'offset'],
    //   props: {
    //     placeholder: '偏移数',
    //   },
    // },
    // {
    //   type: 'Title',
    //   label: '字段映射',
    //   key: 'fieldMap',
    // },
    // {
    //   type: 'Title',
    //   label: '校验规则',
    //   key: 'rules',
    // },
    {
      type: 'function',
      render: (form: FormInstance) => {
        return <RulesSetting key="rule-list" form={form} />;
      },
    },
  ],
  config: {
    props: {
      formItem: {
        label: '多选',
        name: 'checkbox',
      },
      formWrap: {},
      field: {
        label: 'label',
        value: 'value',
      },
    },
    // 组件样式
    style: {},
    // 接口配置
    api: {
      sourceType: 'json',
      // 数据源
      source: [
        {
          label: '选项1',
          value: 1,
        },
        {
          label: '选项2',
          value: 2,
        },
      ],
    },
  },
  // 组件事件
  events: [
    {
      value: 'onChange',
      name: 'onChange事件',
    },
  ],
  methods: [
    {
      name: 'update',
      title: '更新数据',
    },
    {
      name: 'setValue',
      title: '赋值',
    },
    {
      name: 'getValue',
      title: '获取值',
    }
  ],
};
