import React, { useState, useImperativeHandle, forwardRef, useEffect } from 'react';
import * as icons from '@qwikpage/icons';
import { renderIconDefinitionToSVGElement } from '@qwikpage/icons/es/helpers';
import { ComponentType } from '@/packages/types';

const MImage = (
  {
    id,
    type,
    config,
    onClick,
  }: ComponentType<{
    icon: string;
    style?: React.CSSProperties;
  }>,
  ref: any,
) => {
  const [visible, setVisible] = useState(true);
  const [iconSvg, setIconSvg] = useState<string>('');

  // 加载图标SVG
  useEffect(() => {
    if (visible && config?.props?.icon) {
      const icon = config.props.icon in icons
        ? icons[config.props.icon as keyof typeof icons]
        : null;

      if (!icon) {
        setIconSvg(''); // 清空图标
        return; // 直接 return，不返回任何值
      }

      const svgHTMLString = renderIconDefinitionToSVGElement(icon, {
        extraSVGAttrs: { width: '1em', height: '1em', fill: 'currentColor' },
      });
      setIconSvg(svgHTMLString);
    }
  }, [visible, config?.props?.icon]);

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

  if (!visible || !iconSvg) return null;

  return (
    <span
      className="anticon"
      style={{
        fontSize: '18px',
        verticalAlign: 'middle',
        ...config?.style,
      }}
      data-id={id}
      data-type={type}
      onClick={handleClick}
      dangerouslySetInnerHTML={{ __html: iconSvg }}
    />
  );
};

export default forwardRef(MImage);
