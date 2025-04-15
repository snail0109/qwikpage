import { ComponentType } from '@materials/types';
import { useState, useEffect, useImperativeHandle, forwardRef } from 'react';

/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const MLink = ({ config }: ComponentType, ref: any) => {
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
      <a style={config.style} {...config.props}>
        {text || "超链接文本占位"}
      </a>
    )
  );
};
export default forwardRef(MLink);
