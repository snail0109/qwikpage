import React, { useState, useImperativeHandle, forwardRef } from 'react';
import * as Icons from '@ant-design/icons';
import { ComponentType } from '@materials/types';
/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const MIcon = (
  {
    config,
    onClick,
  }: ComponentType<{
    icon: string;
    style?: React.CSSProperties;
  }>,
  ref: any,
) => {
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

  const iconsList: { [key: string]: any } = Icons;

   // 获取icon名称，如果没有则使用默认值GithubOutlined
   const iconName = config.props?.icon || 'GithubOutlined';
   const IconComponent = iconsList[iconName] || iconsList['GithubOutlined'];
   
  return (
    visible &&
    React.createElement(IconComponent, {
      style: config.style,
      ...config.props,
      onClick: handleClick,
    })
  );
};
export default forwardRef(MIcon);
