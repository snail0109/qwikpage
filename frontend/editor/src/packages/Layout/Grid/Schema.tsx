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
      type: 'Radio',
      label: '排列模式',
      name: 'colNum',
      props: {
        rootClassName: 'flexRadio',
        block: true,
        optionType: 'button',
        options: [
          {
            value: 3,
            label: <div className="columnButton">
              <span>3列样式</span>
              <div className="columnContainer">
                <div style={{ width: '33.3%' }}></div>
                <div style={{ width: '33.3%' }}></div>
                <div style={{ width: '33.3%' }}></div>
              </div>
            </div>
          },
          {
            value: 4,
            label: <div className="columnButton">
              <span>4列样式</span>
              <div className="columnContainer">
                <div style={{ width: '8%' }}></div>
                <div style={{ width: '16%' }}></div>
                <div style={{ width: '33.3%' }}></div>
                <div style={{ width: '41%' }}></div>
              </div>
            </div>
          },
          {
            value: 2,
            label: <div className="columnButton">
              <span>2列样式</span>
              <div className="columnContainer">
                <div style={{ width: '41%' }}></div>
                <div style={{ width: '58%' }}></div>
              </div>
            </div>
          }
        ]
      }
    },
    {
      type: 'InputNumber',
      label: '栅格间隔',
      name: 'gutter',
    },
    {
      type: 'Switch',
      label: '是否自动换行',
      name: 'wrap',
    },
  ],
  config: {
    // 组件默认属性值
    props: {
      gutter: 0,
      wrap: true,
      colNum: 3
    },
    style: {},
    events: [],
    api: {},
  },
  // 组件事件
  events: [
    {
      value: 'onClick',
      name: '点击事件',
    },
  ],
  // 默认子组件
  elements: [
    {
      name: '列组件',
      type: 'Col',
      config: {
        props: {
          span: 8
        }
      }
    },
    {
      name: '列组件',
      type: 'Col',
      config: {
        props: {
          span: 8
        }
      }
    },
    {
      name: '列组件',
      type: 'Col',
      config: {
        props: {
          span: 8
        }
      }
    }
  ]
};
