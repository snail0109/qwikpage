import { ComponentType, IDragTargetItem } from '../../types';
import { useDrop } from 'react-dnd';
import { Row, Col } from 'antd';
import { getComponent } from '../../index';
import MarsRender from '../../MarsRender/MarsRender';
import { usePageStore } from '../../../stores/pageStore';
import { forwardRef, useImperativeHandle, useState } from 'react';

/*泛型只需要定义组件本身用到的属性*/
export interface IConfig {
  text: string;
}
/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const Grid = ({ id, type, config, elements }: ComponentType, ref: any) => {
  const addChildElements = usePageStore((state) => state.addChildElements);
  const [visible, setVisible] = useState(true);
  // 拖拽接收
  const [, drop] = useDrop({
    accept: 'MENU_ITEM',
    async drop(item: IDragTargetItem, monitor) {
      if (monitor.didDrop()) return;
      // 生成默认配置
      const { config, events, methods = [] }: any = (await getComponent(item.type + 'Config'))?.default || {};
      addChildElements({
        type: item.type,
        name: item.name,
        parentId: id,
        id: item.id,
        config,
        events,
        methods,
      });
    },
    collect: (monitor) => ({
      isOver: monitor.isOver(),
      canDrop: monitor.canDrop(),
    }),
  });

  // 对外暴露方法
  useImperativeHandle(ref, () => {
    return {
      show() {
        setVisible(true);
      },
      hide() {
        setVisible(false);
      },
    };
  });

  const gutter = config.props?.gutter || 0;

  return (
    visible && (
      <Row style={config.style} {...config.props} gutter={gutter} data-id={id} data-type={type} ref={drop}>
        {elements?.length ? (
          <MarsRender elements={elements || []} />
        ) : (
          <div className="slots" style={{ height: 100, lineHeight: '100px' }}>
            拖拽组件到这里
          </div>
        )}
      </Row>
    )
  );
};
export default forwardRef(Grid);
