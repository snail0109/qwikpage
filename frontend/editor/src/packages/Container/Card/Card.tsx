import React, { forwardRef, useImperativeHandle, useMemo, useState } from 'react';
import { ComponentType, IDragTargetItem } from '@/packages/types';
import { Button, Card, Avatar } from 'antd';
import { useDrop } from 'react-dnd';
import { getComponent } from '@/packages/index';
import MarsRender from '@/packages/MarsRender/MarsRender';
import { usePageStore } from '@/stores/pageStore';
import QIcon from "@/components/icons/QIcon";
import { omit } from 'lodash-es';
import { handleActionFlow } from '@/packages/utils/action';
/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const MCard = ({ id, type, config, elements, onClick, onClickMore }: ComponentType, ref: any) => {
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
    // TODO: 拖拽组件时，容器呈现背景色（后期需要判断组件是否可以拖入）
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

  const meta = useMemo(() => config.props.meta, [config.props.meta]);
  const avatar = useMemo(() => config.props.avatar || undefined, [config.props.avatar]);

  const handleOperate = (eventName: string) => {
    const btnEvent = config.events.find((event) => event.eventName === eventName);
    handleActionFlow(btnEvent?.actions, {});
  };

  const bulkActionList = config.props.bulkActionList || [];

  return (
    visible && (
      <Card
        style={config.style}
        {...omit(config.props, ['cover', 'meta', 'title'])}
        data-id={id}
        data-type={type}
        {...(config.props.header ? { title: config.props.title } : {})}
        cover={config.props.cover ? <img src={config.props.cover} /> : null}
        extra={
          config.props.header && (
          <div style={{ display: 'flex', gap: 10 }}>
            {bulkActionList.map((item: any, index: number) => {
              return (
                <Button
                  key={item.eventName}
                  type={item.type}
                  danger={item.danger}
                  icon={item.icon ? <QIcon name={item.icon} /> : null}
                  onClick={() => handleOperate(item.eventName)}
                >
                  {item.text}
                </Button>
              );
            })}
          </div>)
        }
        onClick={() => onClick?.()}
        ref={drop}
      >
        {meta.title || meta.description ? <Card.Meta {...meta} avatar={avatar && <Avatar src={avatar} />} /> : null}
        {elements?.length ? (
          <MarsRender elements={elements || []} />
        ) : (
          <div className="slots" style={{ lineHeight: '100px' }}>
            拖拽组件到这里
          </div>
        )}
      </Card>
    )
  );
};
export default forwardRef(MCard);
