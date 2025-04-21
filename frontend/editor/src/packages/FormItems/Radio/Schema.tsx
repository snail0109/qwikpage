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
    {
      type: 'Input',
      label: '字段',
      name: ['formItem', 'name'],
    },
    {
      type: 'Input',
      label: 'label',
      name: ['field', 'label'],
    },
    {
      type: 'Input',
      label: 'value',
      name: ['field', 'value'],
    },
    {
      type: 'Variable',
      label: '选中值',
      name: ['defaultValue'],
      props: {
        placeholder: '请输入默认值',
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
      type: 'Switch',
      label: '禁用',
      name: ['formWrap', 'disabled'],
    },
    {
      type: 'Title',
      label: '校验规则',
      key: 'rules',
    },
    {
      type: 'InputNumber',
      label: '标签栅格布局',
      name: ['formItem', 'labelCol', 'span'],
      props: {
        placeholder: '请输入',
      },
    },
    {
      type: 'InputNumber',
      label: '控件栅格布局',
      name: ['formItem', 'wrapperCol', 'span'],
      props: {
        placeholder: '请输入',
      },
    },
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
        label: '单选',
        name: 'radio',
      },
      formWrap: {
        optionType: 'default',
        buttonStyle: 'solid',
      },
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
  ],
};
