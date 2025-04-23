import { forwardRef, useImperativeHandle, useMemo, useState } from 'react';
import { ComponentType } from '@materials/types';
import { Button, Card, Avatar } from 'antd';
import MarsRender from '@materials/MarsRender/MarsRender';
import { usePageStore } from '@materials/stores/pageStore';
import QIcon from '@materials/components/icons/QIcon';
import { omit } from 'lodash-es';
import { handleActionFlow } from '@materials/utils/action';
/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const MCard = ({ config, elements, onClick }: ComponentType, ref: any) => {
  const [visible, setVisible] = useState(true);
 

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

  const handleClick = () => {
    onClick?.();
  };
  
  const handleOperate = (eventName: string) => {
    const btnEvent = config.events.find((event) => event.eventName === eventName);
    handleActionFlow(btnEvent?.actions, {});
  };

  const bulkActionList = config.props.bulkActionList || [];

  const meta = useMemo(() => config.props.meta, [config.props.meta]);
  const avatar = useMemo(() => config.props.avatar || undefined, [config.props.avatar]);

  // 处理meta.title和meta.description的值为对象的情况
  const parseMetaValue = (value: any) => {
    if (typeof value === 'object' && value !== null && 'value' in value) {
      return value.value; // 如果是 {type: "static", value: ""} 结构，提取 value
    }
    return value;
  };

   // 处理 meta 数据
   const processedMeta = useMemo(() => {
    if (!meta) return null;
    return {
      ...meta,
      title: parseMetaValue(meta.title),
      description: parseMetaValue(meta.description)
    };
  }, [meta]);

  return (
    visible && (
      <Card
        style={config.style}
        {...omit(config.props, ['cover', 'meta', 'title'])}
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
        onClick={handleClick}
      >
        {config.props.showmeta && (processedMeta?.title || processedMeta?.description) ? (
          <Card.Meta 
            {...processedMeta} 
            avatar={avatar && <Avatar src={avatar} />} 
          />
        ) : null}
       <MarsRender elements={elements} />
      </Card>
    )
  );
};
export default forwardRef(MCard);
