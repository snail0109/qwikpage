import { Button, Collapse, Form, Modal, Popover, Tree } from 'antd';
import type { CollapseProps } from 'antd';
import { forwardRef, memo, useCallback, useImperativeHandle, useState } from 'react';
import { DownOutlined, NotificationOutlined, QuestionCircleOutlined } from '@ant-design/icons';
import { isString } from 'lodash-es';
import { usePageStore } from '@/stores/pageStore';
import { useProjectStore } from '@/stores/projectStore';
import VsEditor from '../VsEditor';
import { getElement, getParentForm } from '@/utils/util';
import { isFormPlugin } from '@/packages/utils/util';
import styles from './variable.module.less';
import { cloneDeep, isEmpty } from 'lodash-es';
import components from '@/config/components';

const SelectVariableModal = ({ onSelect }: { onSelect: (record: any) => void }, ref: any) => {
  const [visible, setVisible] = useState(false);
  const [form] = Form.useForm();
  // 页面
  const { variables, pageName, elements, elementsMap } = usePageStore((state) => {
    return {
      variables: state.page.pageData.variables,
      pageName: state.page.name,
      elements: state.page.pageData.elements,
      elementsMap: state.page.pageData.elementsMap,
    };
  });

  // 项目
  const { projectVariables } = useProjectStore((state) => {
    return {
      projectVariables: state.variables,
    };
  });

  /**
   * 逻辑表达式中，展示表单，表单内的表单项，表单外的表单项，循环项
   * @returns
   */
  const getFormAndTable = useCallback(() => {
    const list: Array<{ id: string; name: string; elements: any[] }> = [];
    Object.keys(elementsMap).map((id) => {
      const formPlugin = isFormPlugin(elementsMap[id]);
      if (id.startsWith('SearchForm_') || id.startsWith('Form_') || id.startsWith('GridForm_') || id.startsWith('MarsTable_')) {
        const { element }: any = getElement(cloneDeep(elements), id);
        if (!element) return;
        element.name = `${element.name}(${id})`
        element.elements = [];
        list.push(element);
      } else if (formPlugin && elementsMap[id].inForm) {
        // 表单内的表单项
        const { element }: any = getElement(cloneDeep(elements), id);
        if (!element) return;
        const { element: parentElement }: any = getParentForm(cloneDeep(elementsMap), elementsMap[id]);
        if (!parentElement) {
          return;
        }
        const targetForm = list.find(item => item.id === parentElement.id);
        if (!targetForm) {
          return;
        }
        const formItem = elementsMap[id]?.config.props.formItem;
        if (formItem && formItem.name) {
          targetForm?.elements.push({
            ...element,
            name: `${formItem.label}(${formItem.name})`,
          })
        }
      } else if (formPlugin && !elementsMap[id].inForm) {
        // 收集不处于表单中的表单控件
        const { element }: any = getElement(cloneDeep(elements), id);
        if (!element) return;
        let eleName = id;
        components.some(v => {
          const targetEle = (v.data || []).find(item => item.type === element.type)
          if (targetEle) {
            eleName = `${targetEle.name}(${id})`
            return true;
          }
          return false;
        })
        element.name = eleName;
        list.push(element)
      } else if (elementsMap[id].type === 'Loop') {
        const { element }: any = getElement(cloneDeep(elements), id);
        if (!element) return;
        element.name = `${element.name}(${id})`
        element.elements = [
          {
            id: id + '_value',
            name: "# 循环项值",
            type: 'loopItemValue',        
            val: id,
          },
          {
            id: id + '_index',
            name: "# 循环项索引",
            type: 'loopItemIndex',
            val: id,       
          }
        ];
        list.push(element);
      }
    });
    return list;
  }, [elementsMap, elements]);

  // 定义树形结构
  const treeData: any = [
    {
      name: `页面【${pageName}】`,
      id: 'page',
      elements: [
        // {
        //   name: '系统变量',
        //   id: 'SystemVariable',
        //   type: 'SystemVariable',
        //   elements: [
        //     {
        //       type: 'Store',
        //       id: 'userId',
        //       name: 'userId',
        //     },
        //     {
        //       type: 'Store',
        //       id: 'nickName',
        //       name: 'nickName',
        //     },
        //     {
        //       type: 'Store',
        //       id: 'userName',
        //       name: 'userName',
        //     },
        //   ],
        // },
        {
          name: '变量',
          id: 'PageVariable',
          type: 'PageVariable',
          elements: transformToList(variables, 'page'),
        },
        ...getFormAndTable(),
      ],
    },
    {
      name: `项目变量`,
      id: 'project',
      elements: transformToList(projectVariables, 'project'),
      // elements: [
      //   {
      //     name: '全局变量',
      //     id: 'ProjectVariable',
      //     type: 'ProjectVariable',
      //     elements: transformToList(projectVariables),
      //   },
      // ],
    }
  ];

  /**
 * 把变量转换为树形结构
 * 对象需要递归展开
 * @param items 变量列表
 * @param variableType 变量类型 'project' | 'page'
 * @returns 
 */
function transformToList(items: Array<any>, variableType: 'project' | 'page') {
  if (!items) {
    return []
  }
  return items.map((item) => {
    const { name, type, defaultValue } = item;
    const node: any = { name, value: defaultValue, elements: [] };

    // 根据变量类型设置不同的 type
    const baseType = variableType === 'project' ? 'ProjectVariable' : 'Variable';

    if (type === 'array') {
      node.type = baseType;
      node.id = item.name;
      node.name = `Array<${item.name}>${item.remark ? '(' + item.remark + ')' : ''}`;
    } else if (type === 'object') {
      node.id = item.name;
      node.name = `${item.name}${item.remark ? '(' + item.remark + ')' : ''}`;
      node.type = baseType;
      node.elements = transformToList(
        Object.entries(defaultValue).map(([key, value]) => {
          return {
            type: Array.isArray(value) ? 'array' : typeof value,
            id: item.name + '.' + key,
            name: item.name + '.' + key,
            defaultValue: value,
          };
        }),
        variableType
      );
    } else {
      node.type = baseType;
      node.id = item.name;
      node.name = `${item.name}${item.remark ? '(' + item.remark + ')' : ''}`;
    }
    return node;
  });
}

  useImperativeHandle(ref, () => {
    return {
      open(value: string) {
        form.setFieldValue('expression', value);
        setVisible(true);
      },
    };
  });

  // 添加表格渲染模板快捷键
  const onAddRenderTmpl = (type: string) => {
    if (type === 'boolean') {
      form.setFieldValue('expression', `true`);
    }
    if (type === 'table') {
      form.setFieldValue(
        'expression',
        `function render(text, record, index) {
    return text;
}`,
      );
    } else if (type === 'expression') {
      form.setFieldValue('expression', `status === 1 ? '启用' : '禁用'`);
    } else if (type === 'function') {
      form.setFieldValue(
        'expression',
        `function run() {
    return context.eventParams;
}`,
      );
    }
  };

  // 参数和变量选择
  const handleSelect = (selectedKeys: any, { node }: any) => {
    if (!node.type) return;
    const beforeExpression = form.getFieldValue('expression') ?? '';

    // 选择系统存储变量
    if (node.type === 'Store') {
      form.setFieldValue('expression', `${beforeExpression} context.store.${node.id}`.trimStart());
      return;
    }
    // 选择页面变量
    if (node.type === 'Variable') {
      form.setFieldValue('expression', `${beforeExpression} context.variable.${node.id}`.trimStart());
      return;
    }

    // 选择项目变量
    if (node.type === 'ProjectVariable') {
      form.setFieldValue('expression', `${beforeExpression} context.globalVariable.${node.id}`.trimStart());
      return;
    }
    // 选择表单变量
    if (node.type === 'EditTable') {
      const name = elementsMap[node.id]?.config.props.field;
      if (name) form.setFieldValue('expression', `${beforeExpression} context.${node.parentId}.${name}`.trimStart());
      return;
    }
    // 选择循环项
    if (node.type === 'loopItemValue') {
      form.setFieldValue('expression', `${beforeExpression} context.forEachValue.${node.val}.item`.trimStart());
      return;
    }
    if (node.type === 'loopItemIndex') {
      form.setFieldValue('expression', `${beforeExpression} context.forEachValue.${node.val}.index`.trimStart());
      return;
    }
    // 判断取值方式，如果是表单项，就按照表单的方式取值
    if (node.type === 'PageVariable') {
      form.setFieldValue('expression', `${beforeExpression} context.variable`);
    } else if (node.type === 'ProjectVariable') {
      form.setFieldValue('expression', `${beforeExpression} context.globalVariable`);
    } else if (node.type === 'EditTable') {
      const name = elementsMap[node.id]?.config.props.field;
      if (name) form.setFieldValue('expression', `${beforeExpression} context.${node.parentId}.${name}`.trimStart());
    } else if (node.type === 'Form' || node.type === 'SearchForm') {
      form.setFieldValue('expression', `${beforeExpression}  context.${node.id}`);
    } else if (node.inForm) {
      // 表单内的表单项
      const formItem = elementsMap[node.id]?.config.props.formItem;
      form.setFieldValue('expression', `${beforeExpression} context.${isString(node.inForm) ? node.inForm : node.parentId}.${formItem.name}`.trimStart());
    } else {
      // 不在表单内的表单项
      form.setFieldValue('expression', `${beforeExpression} context.${node.id}`);
    }
  };

  // 计算函数点击
  const handleClick = (type: string) => {
    const beforeExpression = form.getFieldValue('expression') ?? '';
    form.setFieldValue('expression', `${beforeExpression} context.${type}()`.trimStart());
  };

  // 确认提交
  const handleSubmit = () => {
    const expression = form.getFieldValue('expression');
    onSelect({ value: expression });
    handleCancel();
  };

  // 关闭
  const handleCancel = () => {
    setVisible(false);
  };
  const fnList: CollapseProps['items'] = [
    {
      key: '1',
      label: '逻辑函数',
      children: null,
    },
    {
      key: '2',
      label: '数学函数',
      children: null,
    },
    {
      key: '3',
      label: '日期函数',
      children: (
        <ul>
          <li onClick={() => handleClick('FORMAT')}>FORMAT</li>
        </ul>
      ),
    },
    {
      key: '4',
      label: '数组函数',
      children: null,
    },
  ];
  return (
    <Modal open={visible} onCancel={handleCancel} title="逻辑编辑器" width={1100} onOk={handleSubmit} okText="确认" cancelText="取消">
      <div style={{ marginBlock: 10 }}>
        <NotificationOutlined style={{ color: '#216EF7' }} />
        <span style={{ marginLeft: 5 }}>下表为页面定义的全局变量，选择时，直接鼠标点击对应的行即可。</span>
      </div>
      <div className={styles.container}>
        <div className={styles.leftFn}>
          <div className={styles.title}>计算函数</div>
          <div className={styles.box}>
            <Collapse ghost items={fnList} style={{ marginTop: -13 }} />
          </div>
        </div>
        <div className={styles.content}>
          <div className={styles.title}>
            表达式
            <Popover
              content={
                <div>
                  <p>1. 默认支持普通变量定义，最终会返回该解析后的变量值：variable.current </p>
                  <p>2. 支持三元表达式：userName === 'jack' ? 1 : 2 </p>
                  <p>2. 支持布尔值：true ，最终会被解析会true变量进行返回。</p>
                  <p>3. 支持逻辑判断：if( a {'>'} 1 ) return 1; return 2 </p>
                  <p>4. 支持函数function定义：function getName(){'{ return "jack" } '}</p>
                  <p>5. 定义函数的时候，不支持自定义参数，参数只能是从右侧选择的变量或者表单对象，系统会自动解析。</p>
                  <p>6. 表格自定义渲染时，需要手工定义渲染函数：function render(text, record, index) {'{ return text; }'}</p>
                </div>
              }
            >
              <QuestionCircleOutlined style={{ marginLeft: 10, cursor: 'pointer' }} />
            </Popover>
            <Button type="link" onClick={() => onAddRenderTmpl('boolean')}>
              布尔值
            </Button>
            <Button type="link" onClick={() => onAddRenderTmpl('expression')}>
              三元表达式模板
            </Button>
            <Button type="link" onClick={() => onAddRenderTmpl('function')}>
              函数模板
            </Button>
            <Button type="link" onClick={() => onAddRenderTmpl('table')}>
              表格自定义渲染模板
            </Button>
          </div>
          <div className={styles.formula}>
            <Form form={form}>
              <Form.Item noStyle name="expression">
                <VsEditor height="440px" language="javascript" />
              </Form.Item>
            </Form>
          </div>
        </div>
        <div className={styles.variable}>
          <div className={styles.title}>参数和变量</div>
          <Tree
            showLine
            defaultExpandAll
            switcherIcon={<DownOutlined />}
            fieldNames={{ title: 'name', key: 'id', children: 'elements' }}
            treeData={treeData}
            onSelect={handleSelect}
          />
        </div>
      </div>
    </Modal>
  );
};

export default memo(forwardRef(SelectVariableModal));
