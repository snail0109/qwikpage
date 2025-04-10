import { useState, useEffect, useImperativeHandle, forwardRef } from 'react';
import { Typography } from 'antd';
import { ComponentType } from '@/packages/types';
import { omit } from 'lodash-es';
import './index.less';

/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const MText = ({ id, type, config, onClick }: ComponentType, ref: any) => {
  const [text, setText] = useState('');
  const [visible, setVisible] = useState(true);

  useEffect(() => {
    const originText = config.props?.text?.toString() || '';
    setText(originText);
  }, [config.props.text]);

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

  // 根据 hiddenText 属性设置文本样式
  const getTextStyle = () => {
    const hiddenText = config.props?.hiddenText;
    const style = { ...config.style };
    
    switch (hiddenText) {
      case 'ellipsis':
        style.whiteSpace = 'nowrap';
        style.overflow = 'hidden';
        style.textOverflow = 'ellipsis';
        style.display = 'block';
        break;
      case 'break':
        style.whiteSpace = 'break-spaces';
        style.wordBreak = 'break-all';
        break;
      case 'wrap':
        style.whiteSpace = 'pre-wrap';
        style.wordBreak = 'normal';
        break;
      case 'nowrap':
        style.whiteSpace = 'nowrap';
        break;
      default:
        // 默认不处理
        break;
    }
    
    return style;
  };

  return (
    visible && (
      <Typography.Text 
        style={getTextStyle()} 
        {...omit(config.props, ['script', 'text', 'hiddenText', 'formItem'])} 
        onClick={handleClick} 
        data-id={id} 
        data-type={type}
        className="text-placeholder" // 只有编辑器中使用, 防止文本内容为空时, 文本框无法被选中
      >
        {text || ''}
      </Typography.Text>
    )
  );
};
export default forwardRef(MText);
