/**
 * 组件配置和属性值
 */
import { FormInstance } from 'antd';
import RulesSetting from '../../components/RulesSetting';
export default {
  // 组件属性配置JSON
  attrs: [
    {
      type: 'Title',
      label: '标签配置',
      key: 'title1',
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
      type: 'Variable',
      label: '默认值',
      name: ['defaultValue'],
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
    // {
    //   type: 'Switch',
    //   label: '可检索',
    //   name: ['formWrap', 'showSearch'],
    // },
    {
      type: 'Switch',
      label: '支持清除',
      name: ['formWrap', 'allowClear'],
    },
    {
      type: 'RadioGroupBtn',
      label: '选项模式',
      name: ['formWrap', 'mode'],
      props: {
        options: [
          { value: '', label: '单选' },
          { value: 'multiple', label: '多选' },
        ],
      },
    },
    {
      type: 'InputNumber',
      label: '最大选中数量',
      name: ['formWrap', 'maxTagCount'],
    },
    {
      type: 'Input',
      label: '默认提示',
      name: ['formWrap', 'placeholder'],
    },
    {
      type: 'Switch',
      label: '虚拟滚动',
      name: ['formWrap', 'virtual'],
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
      type: 'function',
      render: (form: FormInstance) => {
        return <RulesSetting key="rule-list" form={form} />;
      },
    },
  ],
  config: {
    props: {
      formItem: {
        label: '下拉框',
        name: 'select',
      },
      // 组件默认属性值
      formWrap: {
        placeholder: '请选择数据',
        allowClear: true,
        mode: '',
        virtual: true,
      },
      field: {
        label: 'label',
        value: 'value',
      },
    },
    // 组件样式
    style: {
      minWidth: 120,
    },
    // 接口配置
    api: {
      sourceType: 'json',
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
