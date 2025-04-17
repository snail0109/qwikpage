import React, { useState, useImperativeHandle, forwardRef, useEffect } from "react";
import QIcon from "@/components/icons/QIcon";
import { ComponentType } from "@/packages/types";

const MIcon = (
  {
    id,
    type,
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
      <span data-id={id} data-type={type} onClick={handleClick}>
        <QIcon style={config.style} {...config.props} name={config.props.icon} />
      </span>
    )
  );
};

export default forwardRef(MIcon);
