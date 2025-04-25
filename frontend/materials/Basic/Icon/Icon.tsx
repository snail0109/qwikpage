import React, { useState, useImperativeHandle, forwardRef } from 'react';
import QIcon from '@materials/components/icons/QIcon';
import { ComponentType } from '@materials/types';

const MIcon = (
  {
    config,
    onClick,
  }: ComponentType<{
    icon: string;
    style?: React.CSSProperties;
  }>,
  ref: any
) => {
  const [visible, setVisible] = useState(true);
  
  // 对外暴露方法
  useImperativeHandle(ref, () => ({
    show() {
      setVisible(true);
    },
    hide() {
      setVisible(false);
    },
  }));

  const handleClick = () => {
    onClick?.();
  };

  return (
    visible && (
      <span onClick={handleClick}>
        <QIcon style={config.style} {...config.props} name={config.props.icon || 'GithubOutlined'} />
      </span>
    )
  );
};

export default forwardRef(MIcon);
