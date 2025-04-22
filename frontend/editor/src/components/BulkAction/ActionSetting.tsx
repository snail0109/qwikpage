import { memo, useRef } from 'react';
import { Form, Input, Space, Button, FormInstance } from 'antd';
import { useDebounceFn } from 'ahooks';
import { cloneDeep } from 'lodash-es';
import { DeleteOutlined, PlusOutlined, EditOutlined, HolderOutlined } from '@ant-design/icons';
import { DndProvider, useDrag, useDrop } from 'react-dnd';
import { HTML5Backend } from 'react-dnd-html5-backend';
import ActionButtonModal from './ActionButtonModal';
import { usePageStore } from '@/stores/pageStore';
import { ComponentType } from '@/packages/types';
import { createId } from '@/utils/util';

// 定义拖拽项组件
const DraggableItem = ({ index, moveItem, children }) => {
  const ref = useRef(null);
  
  const [{ isDragging }, drag] = useDrag({
    type: 'FORM_ITEM',
    item: { index },
    collect: (monitor) => ({
      isDragging: monitor.isDragging(),
    }),
  });
  
  const [, drop] = useDrop({
    accept: 'FORM_ITEM',
    hover: (item, monitor) => {
      if (!ref.current) {
        return;
      }
      const dragIndex = item.index;
      const hoverIndex = index;
      
      // 不替换自己
      if (dragIndex === hoverIndex) {
        return;
      }
      
      // 确定鼠标位置
      const hoverBoundingRect = ref.current?.getBoundingClientRect();
      const hoverMiddleY = (hoverBoundingRect.bottom - hoverBoundingRect.top) / 2;
      const clientOffset = monitor.getClientOffset();
      const hoverClientY = clientOffset.y - hoverBoundingRect.top;
      
      // 向上拖动时，只有当鼠标超过中点时才移动
      if (dragIndex < hoverIndex && hoverClientY < hoverMiddleY) {
        return;
      }
      
      // 向下拖动时，只有当鼠标超过中点时才移动
      if (dragIndex > hoverIndex && hoverClientY > hoverMiddleY) {
        return;
      }
      
      // 执行移动
      moveItem(dragIndex, hoverIndex);
      
      // 注意：我们在这里修改监视器项的索引！
      // 通常情况下，最好避免这样的突变，
      // 但在这里它是必要的，以避免昂贵的索引搜索。
      item.index = hoverIndex;
    },
  });
  
  const opacity = isDragging ? 0.4 : 1;
  
  drag(drop(ref));
  
  return (
    <div ref={ref} style={{ opacity, marginBottom: '8px' }}>
      {children}
    </div>
  );
};

/**
 * 操作栏配置
 */
const ActionSetting = memo(({ form }: { form: FormInstance }) => {
  const { selectedElement, elementsMap, editEvents } = usePageStore((state) => ({
    selectedElement: state.selectedElement,
    elementsMap: state.page.pageData.elementsMap,
    editEvents: state.editEvents,
  }));
  const modalRef = useRef<{ open: (index: number) => void }>();
  
  // 创建批量操作按钮
  const handleCreate = (add: any) => {
    add({
      text: '按钮',
      type: 'primary',
      eventName: createId('BulkAction'),
    });
    handleEvents();
  };
  
  // 删除批量操作按钮
  const handleDelete = (remove: any, name: number) => {
    remove(name);
    handleEvents();
  };
  
  // 更新事件
  const handleEvents = () => {
    const element: ComponentType = elementsMap[selectedElement?.id as string];
    // 需要提前把已经存进去的action事件过滤掉，不然会重复
    const events: any = element.events?.filter((item: any) => item.value.indexOf('BulkAction') == -1);
    form.getFieldValue('bulkActionList')?.map((item: any) => {
      // 动态新增的按钮，需要动态生成事件
      events.push({
        name: item.text + '事件',
        value: item.eventName,
      });
    });
    editEvents({
      id: selectedElement?.id,
      events,
    });
  };
  
  // 设置
  const handleOpen = (index: number) => {
    modalRef.current?.open(index);
  };
  
  const { run } = useDebounceFn(
    (text: string, index: number) => {
      handleUpdate(text, index);
    },
    { wait: 500 },
  );
  
  /**
   * 更新按钮名称和事件名称
   * @param values 来自弹框的修改为对象，直接修改为字符串
   * @param index 索引
   */
  const handleUpdate = (values: any, index: number) => {
    if (typeof values === 'string') {
      form.setFieldValue(['bulkActionList', index, 'text'], values);
    } else {
      form.setFieldValue(['bulkActionList', index], values);
    }
    const eventName = form.getFieldValue(['bulkActionList', index, 'eventName']);
    // 更新事件名称
    const element: ComponentType = elementsMap[selectedElement?.id as string];
    const events: Array<{ name: string; value: string }> = cloneDeep(element.events);
    events.map((event) => {
      if (event.value === eventName) {
        if (typeof values === 'string') {
          event.name = values + '事件';
        } else {
          event.name = values.text + '事件';
        }
      }
      return event;
    });
    editEvents({
      id: selectedElement?.id,
      events,
    });
  };

  return (
    <DndProvider backend={HTML5Backend}>
      <>
        <Form.List name={['bulkActionList']}>
          {(fields, { add, remove, move }) => (
            <>
              {fields.map(({ key, name, ...restField }, index) => (
                <DraggableItem key={key} index={index} moveItem={move}>
                  <Space align="baseline">
                    <Form.Item {...restField} name={[name, 'text']}>
                      <Input placeholder="请输入按钮名称" onChange={(event) => run(event.target.value, name)} />
                    </Form.Item>
                    <Form.Item name={[name, 'eventName']} hidden>
                      <Input />
                    </Form.Item>
                    <Form.Item name={[name, 'type']} hidden>
                      <Input />
                    </Form.Item>
                    <EditOutlined onClick={() => handleOpen(name)} />
                    <DeleteOutlined onClick={() => handleDelete(remove, name)} />
                    <div style={{ cursor: 'grab', padding: '0 8px' }}>
                      <HolderOutlined />
                    </div>
                  </Space>
                </DraggableItem>
              ))}
              <Button 
                type="primary" 
                block 
                ghost 
                onClick={() => handleCreate(add)} 
                icon={<PlusOutlined />}
                style={{ marginTop: '12px' }}
              >
                新增
              </Button>
            </>
          )}
        </Form.List>
        <ActionButtonModal modalRef={modalRef} update={handleUpdate} />
      </>
    </DndProvider>
  );
});

export default ActionSetting;
