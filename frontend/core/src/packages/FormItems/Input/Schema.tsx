/**
 * 组件配置和属性值
 */
import { FormInstance } from 'antd';
import RulesSetting from '../../../components/RulesSetting';
export default {
  // 组件属性配置JSON
  attrs: [
    {
      type: 'Title',
      label: '标签配置',
      key: 'formItem',
    },
    {
      type: 'Input',
      label: '标题',
      name: ['formItem', 'label'],
      props: {
        placeholder: '请输入文本标题',
      },
    },
    // {
    //   type: 'Input',
    //   label: '组件名称',
    //   name: ['formItem', 'name'],
    //   props: {
    //     placeholder: '请输入提交字段',
    //   },
    // },
    {
      type: 'RadioGroupBtn',
      label: '类型',
      name: ['formWrap', 'type'],
      props: {
        options: [
          { label: '文本', value: 'text' },
          { label: '密码', value: 'password' },
        ],
        defaultValue: 'text',
      },
    },
    {
      type: 'Variable',
      label: '值',
      name: ['value'],
      props: {
        placeholder: '请选择表达式',
      },
    },
    {
      type: 'Input',
      label: '占位符',
      name: ['formWrap', 'placeholder'],
      props: {
        placeholder: '请输入',
      },
    },
    {
      type: 'InputNumber',
      label: '最大字符数',
      name: ['formWrap', 'maxLength'],
      props: {
        placeholder: '请输入',
      },
    },
    {
      type: 'Switch',
      label: '自动获取焦点',
      name: ['autoFocus'],
    },
    {
      type: 'Icons',
      label: '前缀图标',
      name: ['formWrap', 'prefixIcons'],
      props: {
        placeholder: '请选择图标',
      },
    },
    {
      type: 'Icons',
      label: '后缀图标',
      name: ['formWrap', 'suffixIcons'],
      props: {
        placeholder: '请选择图标',
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
      label: '可清除',
      name: ['formWrap', 'allowClear'],
    },
    {
      type: 'Switch',
      label: '禁用',
      name: ['formWrap', 'disabled'],
    },
    // {
    //   type: 'Switch',
    //   label: '无样式',
    //   name: ['formItem', 'noStyle'],
    // },
    // {
    //   type: 'Switch',
    //   label: '隐藏域',
    //   name: ['formItem', 'hidden'],
    // },
    // {
    //   type: 'Input',
    //   label: 'Extra',
    //   name: ['formItem', 'extra'],
    //   tooltip: '表单控件下方显示的提示信息',
    //   props: {
    //     placeholder: '默认提示文案',
    //   },
    // },
    // {
    //   type: 'Title',
    //   label: '表单配置',
    //   key: 'formWrap',
    // },
    // {
    //   type: 'Switch',
    //   label: '显示字数',
    //   name: ['formWrap', 'showCount'],
    // },
    // {
    //   type: 'Input',
    //   label: '前置标签',
    //   name: ['formWrap', 'addonBefore'],
    //   props: {
    //     placeholder: 'eg: http://',
    //   },
    // },
    // {
    //   type: 'Input',
    //   label: '后置标签',
    //   name: ['formWrap', 'addonAfter'],
    //   props: {
    //     placeholder: 'eg: .com',
    //   },
    // },
    // {
    //   type: 'Select',
    //   label: '边框样式',
    //   name: ['formWrap', 'variant'],
    //   props: {
    //     options: [
    //       { value: '', label: '无' },
    //       { value: 'outlined', label: '外边框' },
    //       { value: 'borderless', label: '无边框' },
    //       { value: 'filled', label: '填充' },
    //     ],
    //   },
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
        label: '输入框',
        name: 'input',
      },
      // 组件默认属性值
      formWrap: {
        placeholder: '请输入',
        allowClear: true,
      },
      defaultValue: '',
    },
    // 组件样式
    style: {},
  },
  // 组件事件
  events: [
    {
      value: 'onChange',
      name: '输入事件',
    },
    {
      value: 'onBlur',
      name: '失焦事件',
    },
    {
      value: 'onPressEnter',
      name: '回车事件',
    }
  ],
  methods: [
    {
      name: 'setValue',
      title: '赋值',
    },
    {
      name: 'getValue',
      title: '获取值',
    }
  ]
};
