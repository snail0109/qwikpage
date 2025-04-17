import { ComponentType } from '@/packages/types';
import { useState, useEffect, useImperativeHandle, forwardRef } from 'react';

/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const MLink = ({ id, type, config }: ComponentType, ref: any) => {
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
  return (
    visible && (
      <a style={config.style} {...config.props} data-id={id} data-type={type} onClick={e => e.preventDefault()}>
        {text || "超链接文本占位"}
      </a>
    )
  );
};
export default forwardRef(MLink);
